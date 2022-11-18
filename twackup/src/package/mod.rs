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

//! Package module represents some package info
//! that was parsed from dpkg database

mod field;
mod priority;
mod section;
mod status;

pub use self::{
    field::Field,
    priority::Priority,
    section::Section,
    status::{Flags as StatusFlags, SelectionState, State, Status},
};
use crate::parser::Parsable;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

/// Different errors for package fields
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Package priority is unknown
    #[error("Unknown package priority: `{0}`")]
    UnknownPriority(String),

    /// State flag is unknown
    #[error("Unknown package field: `{0}`")]
    UnknownFlag(String),

    /// Installation state is unknown
    #[error("Unknown package state: `{0}`")]
    UnknownState(String),

    /// Selection state in unknown
    #[error("Unknown package selection state `{0}`")]
    UnknownSelectionState(String),

    /// Some field is missing
    #[error("Field is missed: `{0}`")]
    MissingField(Field),

    /// This package is virtual
    #[error("This package is virtual")]
    VirtualPackage,
}

/// Wrapper for dpkg database package
#[derive(Clone, Debug)]
pub struct Package {
    /// The name of the binary package.
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

    /// Priority of the package
    pub priority: Option<Priority>,

    /// Other parsed fields
    other_fields: HashMap<Field, String>,
}

impl Parsable for Package {
    type Error = Error;

    fn new(fields: HashMap<String, String>) -> Result<Self, Self::Error> {
        let mut fields: HashMap<_, _> = fields
            .into_iter()
            .map(|(key, value)| (Field::from(key.as_str()), value))
            .collect();

        let mut fetch_field = |field: Field| -> Result<String, Error> {
            fields.remove(&field).ok_or(Error::MissingField(field))
        };

        let package_id = fetch_field(Field::Package)?;

        // Ignore virtual packages
        #[cfg(feature = "ios")]
        if package_id.starts_with("gsc.") || package_id.starts_with("cy+") {
            return Err(Error::VirtualPackage);
        }

        Ok(Self {
            id: package_id,
            name: fetch_field(Field::Name).ok(),
            version: fetch_field(Field::Version)?,
            status: Status::try_from(fetch_field(Field::Status)?.as_str())?,
            section: Section::from(fetch_field(Field::Section)?.as_str()),
            priority: if let Ok(priority) = fetch_field(Field::Priority) {
                Some(
                    Priority::try_from(priority.as_str())
                        .map_err(|error: &str| Error::UnknownPriority(error.to_string()))?,
                )
            } else {
                None
            },
            other_fields: fields,
        })
    }
}

impl Package {
    /// Searches for installed files
    ///
    /// # Errors
    /// Returns error if dpkg directory couldn't be read or package is not installed
    #[inline]
    pub fn get_installed_files(&self, dpkg_dir: &Path) -> io::Result<Vec<String>> {
        let file = File::open(dpkg_dir.join(format!("info/{}.list", self.id)))?;
        BufReader::new(file).lines().collect()
    }

    /// Creates canonical DEB filename in format of `id_version_arch`
    #[inline]
    #[must_use]
    pub fn canonical_name(&self) -> String {
        let arch = self.get(Field::Architecture).unwrap_or_default();
        format!("{}_{}_{}", self.id, self.version, arch)
    }

    /// Constructs control file of DEB archive.
    /// Respects fields order.
    #[must_use]
    pub fn to_control(&self) -> String {
        let get = |name: Field| self.get(name).unwrap_or_default();

        let header_fields = [
            (Field::Package, self.id.as_str()),
            (Field::Name, self.human_name()),
            (Field::Version, self.version.as_str()),
            (Field::Description, get(Field::Description)),
            (Field::Author, get(Field::Author)),
            (Field::Section, get(Field::Section)),
            (Field::Architecture, get(Field::Architecture)),
            (Field::Depiction, get(Field::Depiction)),
        ];

        // 3 bytes - ": " and '\n'
        let between_kv_length = 3;

        let control_length = header_fields.iter().fold(0, |sum, (name, value)| {
            sum + name.as_str().len() + value.len() + between_kv_length
        });

        let important_fields: HashSet<_> = header_fields.iter().map(|(name, _)| name).collect();

        let other_fields = self
            .other_fields
            .iter()
            .filter(|(name, _)| !important_fields.contains(name));

        // Count total control length to effectively allocate memory
        let control_length = other_fields
            .clone()
            .fold(control_length, |sum, (name, value)| {
                sum + name.as_str().len() + value.len() + between_kv_length
            });

        let push = |mut control: String, name: &Field, value: &str| -> String {
            control.push_str(name.as_str());
            control.push_str(": ");
            control.push_str(value);
            control.push('\n');

            control
        };

        // Build header with important fields
        let control = String::with_capacity(control_length);
        let control = header_fields
            .iter()
            .fold(control, |control, (name, value)| push(control, name, value));

        // And build other fields
        other_fields.fold(control, |control, (name, value)| push(control, name, value))
    }

    /// Searches any package identifiers this package depends on.
    /// Ignores version or any other dependency modifiers
    pub fn dependencies(&self) -> impl Iterator<Item = &str> {
        fn parse(string: &str) -> impl Iterator<Item = &str> {
            string
                .split([',', '|'])
                .map(|dep| match dep.find('(').zip(dep.find(')')) {
                    Some((start, _)) => dep[..start].trim(),
                    _ => dep.trim(),
                })
        }

        let depends = self.get(Field::Depends).unwrap_or_default();
        let pre_depends = self.get(Field::PreDepends).unwrap_or_default();

        parse(depends).chain(parse(pre_depends))
    }

    /// Fetches value associated with this field.
    ///
    /// # Errors
    /// Returns error if there's not such field
    pub fn get<N: AsRef<Field>>(&self, field: N) -> Result<&str, Error> {
        let field = field.as_ref();
        match field {
            Field::Package => Ok(self.id.as_str()),
            Field::Name => self.name.as_deref().ok_or(Error::MissingField(Field::Name)),
            Field::Version => Ok(self.version.as_str()),
            Field::Section => Ok(self.section.as_str()),
            _ => self
                .other_fields
                .get(field)
                .map(String::as_str)
                .ok_or_else(|| Error::MissingField(field.clone())),
        }
    }

    /// Returns package name or identifier if there's no such
    #[inline]
    #[must_use]
    pub fn human_name(&self) -> &str {
        match &self.name {
            Some(name) => name.as_str(),
            None => self.id.as_str(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        error::Result,
        package::{Field, Package},
        parser::{Parsable, Parser},
    };
    use std::{
        collections::HashMap,
        env,
        fs::{self, File},
        io::{self, Write},
        os::unix::fs::PermissionsExt,
        path::Path,
    };

    #[test]
    fn valid_package_get_files() -> Result<()> {
        let mut package_info: HashMap<String, String> = HashMap::new();
        package_info.insert("Package".to_string(), "valid-package".to_string());
        package_info.insert("Version".to_string(), "1.0.0".to_string());
        package_info.insert("Architecture".to_string(), "all".to_string());
        package_info.insert("Status".to_string(), "install ok installed".to_string());
        package_info.insert("Section".to_string(), "Tweaks".to_string());

        let package = Package::new(package_info)?;

        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/dpkg_database_dir");
        let files = package.get_installed_files(Path::new(path))?;
        assert_eq!(files.len(), 3);

        Ok(())
    }

    #[test]
    fn non_valid_package_get_files() -> Result<()> {
        let mut package_info = HashMap::new();
        package_info.insert("Package".to_string(), "non-valid-package".to_string());
        package_info.insert("Version".to_string(), "1.0.0".to_string());
        package_info.insert("Architecture".to_string(), "all".to_string());
        package_info.insert("Status".to_string(), "install ok installed".to_string());
        package_info.insert("Section".to_string(), "Tweaks".to_string());

        let package = Package::new(package_info)?;

        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/dpkg_database_dir");
        let files = package.get_installed_files(Path::new(path));
        assert!(files.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn valid_database() -> Result<()> {
        let database = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/databases/valid");
        let parser = Parser::new(database)?;
        let packages = parser.parse::<Package>().await;
        assert_eq!(packages.len(), 3);

        Ok(())
    }

    #[tokio::test]
    async fn partially_valid_database() -> Result<()> {
        let database = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/databases/partially_valid"
        );
        let parser = Parser::new(database)?;
        let packages = parser.parse::<Package>().await;
        assert_ne!(packages.len(), 3);

        Ok(())
    }

    #[tokio::test]
    async fn multiline() -> Result<()> {
        let database = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/databases/multiline");
        let parser = Parser::new(database)?;

        let packages = parser.parse::<Package>().await;
        let packages: HashMap<String, Package> = packages
            .into_iter()
            .map(|pkg| (pkg.id.clone(), pkg))
            .collect();

        let package = packages.get("valid-package-1").unwrap();
        let description = package.get(Field::Description)?;
        assert_eq!(description, "First Line\n Second Line\n  Third Line");

        assert!(packages.get("invalid-package-1").is_none());

        Ok(())
    }

    #[test]
    fn no_permissions_database() -> Result<()> {
        let database = env::temp_dir().join("twackup-no-permissions");
        let mut file = File::create(&database)?;
        file.write_all("This contents will never be read".as_bytes())?;
        fs::set_permissions(&database, fs::Permissions::from_mode(0o333))?;

        let parser = Parser::new(database.as_path());
        assert!(parser.is_err());
        assert_eq!(
            io::Error::last_os_error().kind(),
            io::ErrorKind::PermissionDenied
        );

        fs::remove_file(&database)?;

        Ok(())
    }

    #[tokio::test]
    async fn real_database() -> Result<()> {
        let database = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/databases/real-system-632"
        );
        let parser = Parser::new(database)?;
        let packages = parser.parse::<Package>().await;
        assert_eq!(packages.len(), 632);

        Ok(())
    }
}
