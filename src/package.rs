use std::{
    collections::HashMap,
    fs::File, path::Path,
    io::{self, BufReader, BufRead}
};

#[derive(Clone, PartialEq)]
pub enum State {
    Unknown,
    Install,
    Remove,
    Purge,
    Hold,
}

#[derive(Clone)]
pub struct Package {
    /// The name of the binary package. This field MUST NOT be empty.
    pub identifier: String,

    /// Name of package that displays in every package manager.
    /// If this field is empty, identifier will be used.
    pub name: String,

    /// Version of package. This field MUST NOT be empty.
    pub version: String,

    /// Architecture of package. This field MUST NOT be empty.
    pub architecture: String,

    /// State of package as it was marked by dpkg itself.
    /// If this field is empty, Unknown state must be used.
    pub state: State,

    /// This field specifies an application area into which
    /// the package has been classified
    pub section: String,

    hashmap: HashMap<String, String>,
}

impl Package {
    pub fn new(fields: &HashMap<String, String>) -> Option<Self> {
        let package_id = fields.get("Package")?.to_string();

        return Some(Package{
            identifier: package_id.clone(),
            name: fields.get("Name").unwrap_or(&package_id).to_string(),
            version: fields.get("Version")?.to_string(),
            architecture: fields.get("Architecture")?.to_string(),
            state: State::from_dpkg(fields.get("Status")),
            section: fields.get("Section").unwrap_or(&"".to_string()).to_string(),
            hashmap: fields.clone(),
        });
    }

    pub fn get_installed_files(&self, dpkg_dir: &Path) -> io::Result<Vec<String>> {
        let file = File::open(dpkg_dir.join("info").join(format!("{}.list", self.identifier)))?;
        return BufReader::new(file).lines().collect();
    }

    pub fn canonical_name(&self) -> String {
        return format!("{}_{}_{}", self.identifier, self.version, self.architecture);
    }

    pub fn create_control(&self) -> String {
        let mut fields_len = 0;
        for (key, value) in self.hashmap.iter() {
            fields_len += key.len() + value.len() + 4;
        }

        let mut control = String::with_capacity(fields_len);

        for (key, value) in self.hashmap.iter() {
            if *key == "Status".to_string() { continue; }
            control.push_str(format!("{}: {}\n", key, value).as_str());
        }

        return  control;
    }

    fn parse_depends(value: Option<&String>) -> Vec<String> {
        match value {
            None => Vec::new(),
            Some(depends) => depends
                .split(",")
                .map(|dependency| dependency.trim().to_string())
                .collect::<Vec<String>>()
        }
    }

    /// Returnes packages of which this one depends.
    pub fn depends(&self) -> Vec<String> { Self::parse_depends(self.hashmap.get("Depends")) }

    /// Returnes packages of which the installation of this package depends.
    pub fn predepends(&self) -> Vec<String> { Self::parse_depends(self.hashmap.get("Pre-Depends")) }

    /// Returnes true if this package us a dependency of other.
    pub fn is_dependency_of(&self, pkg: &Package) -> bool {
        let id = &self.identifier;
        return pkg.depends().contains(id) || pkg.predepends().contains(id);
    }
}

impl State {
    pub fn from_dpkg(string: Option<&String>) -> Self {
        if let Some(status) = string {
            let mut components = status.split_whitespace();
            if let Some(state) = components.next() {
                return match state.to_lowercase().as_str() {
                    "install" => Self::Install,
                    "deinstall" => Self::Remove,
                    "hold" => Self::Hold,
                    _ => Self::Unknown
                }
            }
        }

        return Self::Unknown;
    }
}
