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
    Hold,
}

#[derive(Clone, PartialEq)]
pub enum Section {
    Other(String),
    Unknown,
    System,
    Tweaks,
    Utilities,
    Packaging,
    Development,
    TerminalSupport,
    Themes,
    Archiving,
    Networking,
    TextEditors,
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
    pub section: Section,

    hashmap: HashMap<String, String>,
}

impl Package {
    pub fn new(fields: &HashMap<String, String>) -> Option<Self> {
        let package_id = fields.get("Package")?.to_string();

        Some(Package{
            identifier: package_id.clone(),
            name: fields.get("Name").unwrap_or(&package_id).to_string(),
            version: fields.get("Version")?.to_string(),
            architecture: fields.get("Architecture")?.to_string(),
            state: State::from_dpkg(fields.get("Status")),
            section: Section::from_string_opt(fields.get("Section")),
            hashmap: fields.clone(),
        })
    }

    pub fn get_installed_files(&self, dpkg_dir: &Path) -> io::Result<Vec<String>> {
        let file = File::open(dpkg_dir.join("info").join(format!("{}.list", self.identifier)))?;
        return BufReader::new(file).lines().collect();
    }

    pub fn canonical_name(&self) -> String {
        format!("{}_{}_{}", self.identifier, self.version, self.architecture)
    }

    pub fn create_control(&self) -> String {
        let mut fields_len = 0;
        for (key, value) in self.hashmap.iter() {
            fields_len += key.len() + value.len() + 3;
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

    /// Returns packages of which this one depends.
    pub fn depends(&self) -> Vec<String> { Self::parse_depends(self.hashmap.get("Depends")) }

    /// Returns packages of which the installation of this package depends.
    pub fn predepends(&self) -> Vec<String> { Self::parse_depends(self.hashmap.get("Pre-Depends")) }

    /// Returns true if this package us a dependency of other.
    pub fn is_dependency_of(&self, pkg: &Package) -> bool {
        let id = &self.identifier;
        return pkg.depends().contains(id) || pkg.predepends().contains(id);
    }

    pub fn description(&self) -> Option<&String> { self.hashmap.get("Description") }
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


impl Section {
    pub fn from_string(value: &String) -> Section {
        match value.to_lowercase().as_str() {
            "system" => Section::System,
            "tweaks" => Section::Tweaks,
            "utilities" => Section::Utilities,
            "packaging" => Section::Packaging,
            "development" => Section::Development,
            "themes" => Section::Themes,
            "terminal_support" | "terminal support" => Section::TerminalSupport,
            "networking" => Section::Networking,
            "archiving" => Section::Archiving,
            "text_editors" => Section::TextEditors,
            _ => Section::Other(value.clone())
        }
    }

    pub fn from_string_opt(value: Option<&String>) -> Section {
        match value {
            Some(value) => Self::from_string(value),
            None => Section::Unknown
        }
    }
}
