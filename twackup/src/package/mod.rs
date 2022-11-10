/*
 * Copyright 2020 DanP
 *
 * This file is part of Twackup
 *
 * Twackup is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Twackup is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Twackup. If not, see <http://www.gnu.org/licenses/>.
 */

mod field;
mod package_error;
mod priority;
mod section;
mod status;

pub use self::{
    field::FieldName, package_error::PackageError, priority::Priority, section::Section,
    status::Status,
};
use crate::kvparser::Parsable;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    str::FromStr,
};

#[derive(Clone, Debug)]
pub struct Package {
    /// The name of the binary package. This field MUST NOT be empty.
    pub id: String,

    /// Name of package that displays in every package manager.
    /// If this field is empty, identifier will be used.
    pub name: Option<String>,

    /// Version of package. This field MUST NOT be empty.
    pub version: String,

    /// State of package as it was marked by dpkg itself.
    /// If this field is empty, Unknown state must be used.
    pub status: Status,

    /// This field specifies an application area into which
    /// the package has been classified
    pub section: Section,

    pub priority: Option<Priority>,

    other_fields: HashMap<FieldName, String>,
}

impl Parsable for Package {
    type Error = PackageError;

    fn new(fields: HashMap<String, String>) -> Result<Self, Self::Error> {
        let mut fields: HashMap<_, _> = fields
            .into_iter()
            .map(|(key, value)| {
                // Safe to unwrap because from_str doesn't return error
                (FieldName::from_str(key.as_str()).unwrap(), value)
            })
            .collect();

        let mut fetch_field = |field: FieldName| -> Result<String, PackageError> {
            fields
                .remove(&field)
                .ok_or(PackageError::MissingField(field))
        };

        let package_id = fetch_field(FieldName::Package)?;

        #[cfg(feature = "ios")]
        {
            // Ignore virtual packages
            if package_id.starts_with("gsc.") || package_id.starts_with("cy+") {
                return Err(PackageError::VirtualPackage);
            }
        }

        Ok(Self {
            id: package_id,
            name: fetch_field(FieldName::Name).ok(),
            version: fetch_field(FieldName::Version)?,
            status: Status::try_from(fetch_field(FieldName::Status)?.as_str())?,
            section: Section::from(fetch_field(FieldName::Section)?.as_str()),
            priority: fetch_field(FieldName::Priority)
                .and_then(|priority| Priority::try_from(priority.as_str()))
                .ok(),
            other_fields: fields,
        })
    }
}

impl Package {
    /// Searches for installed files
    ///
    /// # todo!("REFACTOR THIS")
    #[inline]
    pub fn get_installed_files(&self, dpkg_dir: &Path) -> io::Result<Vec<String>> {
        let file = File::open(dpkg_dir.join(format!("info/{}.list", self.id)))?;
        BufReader::new(file).lines().collect()
    }

    /// Creates canonical DEB filename in format of **id_version_arch**
    #[inline]
    pub fn canonical_name(&self) -> String {
        let arch = self.get(FieldName::Architecture).unwrap_or_default();
        format!("{}_{}_{}", self.id, self.version, arch)
    }

    /// Constructs control file of DEB archive.
    /// Respects fields order.
    pub fn to_control(&self) -> String {
        let get = |name: FieldName| self.get(name).unwrap_or_default();

        let header_fields = [
            (FieldName::Package, self.id.as_str()),
            (FieldName::Name, self.human_name()),
            (FieldName::Version, self.version.as_str()),
            (FieldName::Description, get(FieldName::Description)),
            (FieldName::Author, get(FieldName::Author)),
            (FieldName::Section, get(FieldName::Section)),
            (FieldName::Architecture, get(FieldName::Architecture)),
            (FieldName::Depiction, get(FieldName::Depiction)),
        ];

        // 3 bytes - ": " and '\n'
        let between_kv_length = 3;

        let control_length = header_fields.iter().fold(0, |mut sum, (name, value)| {
            sum += name.as_str().len() + value.len() + between_kv_length;
            sum
        });

        let important_fields: HashSet<_> =
            header_fields.iter().map(|(name, _)| name.clone()).collect();

        let other_fields = self
            .other_fields
            .iter()
            .filter(|(name, _)| !important_fields.contains(name));

        // Count total control length to effectively allocate memory
        let control_length = other_fields
            .clone()
            .fold(control_length, |mut sum, (name, value)| {
                sum += name.as_str().len() + value.len() + between_kv_length;
                sum
            });

        // Build header with important fields
        let control = String::with_capacity(control_length);
        let control = header_fields
            .iter()
            .fold(control, |mut control, (name, value)| {
                control.push_str(name.as_str());
                control.push_str(": ");
                control.push_str(value);
                control.push('\n');

                control
            });

        // And build other fields
        other_fields.fold(control, |mut control, (name, value)| {
            control.push_str(name.as_str());
            control.push_str(": ");
            control.push_str(value);
            control.push('\n');

            control
        })
    }

    /// Searches any package identifiers this package depends on.
    /// Ignores version or any other dependency modifiers
    pub fn dependencies(&self) -> impl Iterator<Item = &str> {
        fn parse(string: &str) -> impl Iterator<Item = &str> {
            string
                .split([',', '|'])
                .map(|dep| match dep.find('(').zip(dep.find(')')) {
                    Some((start, _)) => dep[0..start].trim(),
                    _ => dep.trim(),
                })
        }

        let depends = self.get(FieldName::Depends).unwrap_or_default();
        let predepends = self.get(FieldName::Depends).unwrap_or_default();

        parse(depends).chain(parse(predepends))
    }

    /// Fetches value associated with this field.
    ///
    /// # Returns
    /// None if there's no field
    pub fn get<N: AsRef<FieldName>>(&self, field: N) -> Option<&str> {
        match field.as_ref() {
            FieldName::Package => Some(self.id.as_str()),
            FieldName::Name => self.name.as_deref(),
            FieldName::Version => Some(self.version.as_str()),
            FieldName::Section => Some(self.section.as_str()),
            _ => self
                .other_fields
                .get(field.as_ref())
                .map(|value| value.as_str()),
        }
    }

    /// Returns package name or identifier if there's no such
    pub fn human_name(&self) -> &str {
        match &self.name {
            Some(name) => name.as_str(),
            None => self.id.as_str(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Package;
    use crate::kvparser::Parsable;
    use std::{collections::HashMap, path::Path};

    #[test]
    fn valid_package_get_files() {
        let mut package_info: HashMap<String, String> = HashMap::new();
        package_info.insert("Package".to_string(), "valid-package".to_string());
        package_info.insert("Version".to_string(), "1.0.0".to_string());
        package_info.insert("Architecture".to_string(), "all".to_string());
        package_info.insert("Status".to_string(), "install ok installed".to_string());
        package_info.insert("Section".to_string(), "Tweaks".to_string());

        let package = Package::new(package_info).unwrap();

        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/packages");
        let files = package.get_installed_files(Path::new(path));
        assert_eq!(files.is_err(), false);
        assert_eq!(files.unwrap().len(), 3);
    }

    #[test]
    fn non_valid_package_get_files() {
        let mut package_info = HashMap::new();
        package_info.insert("Package".to_string(), "non-valid-package".to_string());
        package_info.insert("Version".to_string(), "1.0.0".to_string());
        package_info.insert("Architecture".to_string(), "all".to_string());
        package_info.insert("Status".to_string(), "install ok installed".to_string());
        package_info.insert("Section".to_string(), "Tweaks".to_string());

        let package = Package::new(package_info).unwrap();

        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/packages");
        let files = package.get_installed_files(Path::new(path));
        assert_eq!(files.is_err(), true);
    }
}
