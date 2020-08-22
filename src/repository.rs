use std::{
    collections::HashMap,
    str::FromStr, string::ToString,
};
use crate::kvparser::Parsable;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum Type {
    Binaries,
    SourceCode,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Repository {
    pub repo_type: Type,
    pub address: String,
    pub distribution: String,
    pub components: String,
}

impl FromStr for Type {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "deb" => Ok(Self::Binaries),
            "deb-src" => Ok(Self::SourceCode),
            _ => Err(())
        }
    }
}

impl Type {
    fn as_str(&self) -> &str {
        match self {
            Self::Binaries => "deb",
            Self::SourceCode => "deb-src"
        }
    }
}

impl Parsable for Repository {
    type Output = Self;

    fn new(fields: HashMap<String, String>) -> Option<Self::Output> {
        let _type = Type::from_str(fields.get("Types")?);
        if _type.is_err() {
            return None;
        }

        Some(Self{
            repo_type: _type.unwrap(),
            address: fields.get("URIs")?.to_string(),
            distribution: fields.get("Suites")?.to_string(),
            components: fields.get("Components").unwrap_or(&"".to_string()).to_string(),
        })
    }
}

impl Repository {
    pub fn from_oneline(line: &str) -> Option<Self> {
        let components: Vec<&str> = line.split(" ").collect();
        if components.len() < 3 {
            return None;
        }

        let _type = Type::from_str(components[0]);
        if _type.is_err() {
            return None;
        }

        Some(Self{
            repo_type: _type.unwrap(),
            address: components[1].to_string(),
            distribution: components[2].to_string(),
            components: components.iter()
                .skip(3).fold(String::new(), |r, c| r + c.trim() + " ")
                .trim().to_string(),
        })
    }

    pub fn to_flat(&self) -> String {
        format!("{} {} {} {}",
            self.repo_type.as_str(), self.address, self.distribution, self.components
        ).trim().to_string()
    }

    pub fn to_modern(&self) -> String {
        format!(
            "Types: {}\nURIs: {}\nSuites: {}\nComponents: {}",
            self.repo_type.as_str(), self.address, self.distribution, self.components
        ).trim().to_string()
    }
}
