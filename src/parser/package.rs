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
    pub identifier: String,
    pub name: String,
    pub version: String,
    pub architecture: String,
    pub state: State,
    pub depends: String,
    pub predepends: String,
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
