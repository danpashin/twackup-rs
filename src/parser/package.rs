use std::collections::HashMap;

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

    /// Packages of which this one depends.
    /// If this field is empty, empty string must be used.
    pub depends: String,

    /// Packages of which installition of this one depends.
    /// If this field is empty, empty string must be used.
    pub predepends: String,

    /// This field specifies an application area into which
    /// the package has been classified
    pub section: String,
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
            depends: fields.get("Depends").unwrap_or(&"".to_string()).to_string(),
            predepends: fields.get("Pre-Depends").unwrap_or(&"".to_string()).to_string(),
            section: fields.get("Section").unwrap_or(&"".to_string()).to_string(),
        });
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
