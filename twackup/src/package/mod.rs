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
mod priority;
mod section;
mod state;

pub use self::{field::Field, priority::Priority, section::Section, state::State};
use crate::kvparser::Parsable;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    str::FromStr,
};

#[derive(Clone)]
pub struct Package {
    /// The name of the binary package. This field MUST NOT be empty.
    pub id: String,

    /// Name of package that displays in every package manager.
    /// If this field is empty, identifier will be used.
    pub name: String,

    /// Version of package. This field MUST NOT be empty.
    pub version: String,

    /// State of package as it was marked by dpkg itself.
    /// If this field is empty, Unknown state must be used.
    pub state: State,

    /// This field specifies an application area into which
    /// the package has been classified
    pub section: Section,

    pub priority: Priority,

    fields: HashMap<Field, String>,
}

impl Parsable for Package {
    type Output = Self;

    fn new(fields: HashMap<String, String>) -> Option<Self::Output> {
        let fields: HashMap<Field, String> = fields
            .into_iter()
            .map(|(key, value)| {
                // Safe to unwrap because from_str doesn't return error
                (Field::from_str(key.as_str()).unwrap(), value)
            })
            .collect();

        let package_id = fields.get(&Field::Package)?;
        // Ignore virtual packages
        if package_id.starts_with("gsc.") || package_id.starts_with("cy+") {
            return None;
        }

        Some(Self {
            id: package_id.to_string(),
            name: fields.get(&Field::Name).unwrap_or(package_id).to_string(),
            version: fields.get(&Field::Version)?.to_string(),
            state: State::from_dpkg(fields.get(&Field::Status)),
            section: Section::from_string_opt(fields.get(&Field::Section)),
            priority: Priority::from_string_opt(fields.get(&Field::Priority)),
            fields,
        })
    }
}

impl Package {
    #[inline]
    pub fn get_installed_files(&self, dpkg_dir: &Path) -> io::Result<Vec<String>> {
        let file = File::open(dpkg_dir.join(format!("info/{}.list", self.id)))?;
        BufReader::new(file).lines().collect()
    }

    /// Creates canonical DEB filename in format of id_version_arch
    #[inline]
    pub fn canonical_name(&self) -> String {
        let arch = self.get_field(Field::Architecture).unwrap_or_default();
        format!("{}_{}_{}", self.id, self.version, arch)
    }

    /// Converts model to DEB control file
    pub fn to_control(&self) -> String {
        let mut fields_len = 0;
        for (key, value) in self.fields.iter() {
            if *key == Field::Status {
                continue;
            }
            fields_len += key.as_str().len() + value.len() + 3;
        }

        let mut control = String::with_capacity(fields_len);

        for (key, value) in self.fields.iter() {
            // Skip status field. It is invalid in control
            if *key == Field::Status {
                continue;
            }
            control.push_str(key.as_str());
            control.push_str(": ");
            control.push_str(value);
            control.push('\n');
        }

        control
    }

    pub fn dependencies(&self) -> impl Iterator<Item = &str> {
        fn parse(string: &str) -> impl Iterator<Item = &str> {
            string
                .split([',', '|'])
                .map(|dep| match dep.find('(').zip(dep.find(')')) {
                    Some((start, _)) => dep[0..start].trim(),
                    _ => dep.trim(),
                })
        }

        let depends = self.get_field(Field::Depends).unwrap_or_default();
        let predepends = self.get_field(Field::Depends).unwrap_or_default();

        parse(depends).chain(parse(predepends))
    }

    #[inline]
    pub fn get_field(&self, field: Field) -> Option<&str> {
        self.fields.get(&field).map(|value| value.as_str())
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

        let package = Package::new(package_info).unwrap();

        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/packages");
        let files = package.get_installed_files(Path::new(path));
        assert_eq!(files.is_err(), true);
    }
}
