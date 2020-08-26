use std::{
    collections::HashMap,
    str::FromStr, string::ToString,
    io,
};
use crate::kvparser::Parsable;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Kind {
    /// Used for distributing binaries only
    Binary,
    /// This is used for distributing sources only
    Source,
    /// Supported only in DEB822 format
    Both,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Repository {
    /// specifies type of repo packages - Binary or Source
    pub kind: Kind,

    /// Specifies the root of the archive
    pub url: String,

    /// Specifies a subdirectory in $ARCHIVE_ROOT/dists
    pub distribution: String,

    pub components: Vec<String>,
}

impl FromStr for Kind {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "deb" => Ok(Self::Binary),
            "deb-src" => Ok(Self::Source),
            "deb deb-src" | "deb-src deb" => Ok(Self::Both),
            _ => Err(io::Error::from(io::ErrorKind::InvalidInput))
        }
    }
}

impl Kind {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Binary => "deb",
            Self::Source => "deb-src",
            Self::Both => "deb deb-src",
        }
    }
}

impl Parsable for Repository {
    type Output = Self;

    /// Performs parsing repo model in DEB822 format
    /// #### Doesn't support options
    fn new(fields: HashMap<String, String>) -> Option<Self::Output> {
        let kind = Kind::from_str(fields.get("Types")?);
        if kind.is_err() {
            return None;
        }

        Some(Self{
            kind: kind.unwrap(),
            url: fields.get("URIs")?.to_string(),
            distribution: fields.get("Suites")?.to_string(),
            components: fields.get("Components").unwrap_or(&"".to_string())
                .split(" ").map(|str| str.to_string()).collect(),
        })
    }
}

impl Repository {
    /// Performs parsing model from one-line style.
    ///
    /// #### This func doesn't support options as they aren't used in iOS.
    pub fn from_one_line(line: &str) -> Option<Self> {
        let components: Vec<&str> = line.split(" ").collect();
        // type, uri and suite are required, so break if they don't exist
        if components.len() < 3 {
            return None;
        }

        let _type = Kind::from_str(components[0]);
        if _type.is_err() {
            return None;
        }

        Some(Self{
            kind: _type.unwrap(),
            url: components[1].to_string(),
            distribution: components[2].to_string(),
            components: components.iter().skip(3).map(|str| str.to_string()).collect(),
        })
    }

    /// Performs fields formatting in the one-line style.
    /// #### Doesn't support options
    pub fn to_one_line(&self) -> String {
        format!("{} {} {} {}",
            self.kind.as_str(), self.url, self.distribution, self.components.join(" ")
        ).trim().to_string()
    }

    /// Performs fields formatting in multiple-lines style. (Also known as DEB822 Style)
    /// #### Doesn't support options
    pub fn to_deb822(&self) -> String {
        format!(
            "Types: {}\nURIs: {}\nSuites: {}\nComponents: {}",
            self.kind.as_str(), self.url, self.distribution, self.components.join(" ")
        ).trim().to_string()
    }

    pub fn to_cydia_key(&self) -> String {
        format!("{}:{}:{}", self.kind.as_str(), self.url, self.distribution)
    }

    pub fn to_dict(&self) -> plist::Dictionary {
        let mut dict = plist::Dictionary::new();
        dict.insert(
            "Distribution".to_string(),
            plist::Value::String(self.distribution.clone())
        );
        dict.insert(
            "URI".to_string(),
            plist::Value::String(self.url.clone())
        );
        dict.insert(
            "Type".to_string(),
            plist::Value::String(self.kind.as_str().to_string())
        );
        dict.insert(
            "Sections".to_string(),
            plist::Value::Array(self.components.iter().map(|val| {
                plist::Value::String(val.clone())
            }).collect())
        );

        return dict;
    }
}
