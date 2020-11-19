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

use std::{
    collections::HashMap,
    fs::File, path::Path,
    io::{self, BufReader, BufRead},
};
use crate::kvparser::Parsable;

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

#[derive(Clone, PartialEq)]
pub enum Priority {
    Optional,
    Required,
    Important,
    Standard,
    Unknown,
}

#[derive(Clone)]
pub struct Package {
    /// The name of the binary package. This field MUST NOT be empty.
    pub id: String,

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

    pub priority: Priority,

    fields: HashMap<String, String>,
}

impl Parsable for Package {
    type Output = Self;

    fn new(fields: HashMap<String, String>) -> Option<Self::Output> {
        let package_id = fields.get("Package")?;
        // Ignore virtual packages
        if package_id.starts_with("gsc.") || package_id.starts_with("cy+") {
            return None;
        }

        Some(Self{
            id: package_id.to_string(),
            name: fields.get("Name").unwrap_or(package_id).to_string(),
            version: fields.get("Version")?.to_string(),
            architecture: fields.get("Architecture")?.to_string(),
            state: State::from_dpkg(fields.get("Status")),
            section: Section::from_string_opt(fields.get("Section")),
            priority: Priority::from_string_opt(fields.get("Priority")),
            fields,
        })
    }
}

impl Package {
    #[inline]
    pub fn get_installed_files(&self, dpkg_dir: &Path) -> io::Result<Vec<String>> {
        let file = File::open(dpkg_dir.join(format!("info/{}.list", self.id)))?;
        return BufReader::new(file).lines().collect();
    }

    /// Creates canonical DEB filename in format of id_version_arch
    #[inline]
    pub fn canonical_name(&self) -> String {
        format!("{}_{}_{}", self.id, self.version, self.architecture)
    }

    /// Converts model to DEB control file
    pub fn to_control(&self) -> String {
        let mut fields_len = 0;
        for (key, value) in self.fields.iter() {
            fields_len += key.len() + value.len() + 3;
        }

        let mut control = String::with_capacity(fields_len);

        for (key, value) in self.fields.iter() {
            // Skip status field. It is invalid in control
            if key.as_str() == "Status" { continue; }
            control.push_str(key);
            control.push_str(": ");
            control.push_str(value);
            control.push_str("\n");
        }

        return control;
    }

    /// Splits optional string by comma to vector of strings
    #[inline]
    fn split_by_comma(&self, field: Option<&String>) -> Vec<String> {
        match field {
            None => Vec::new(),
            Some(value) => value.split(",").map(|val| val.trim().to_string()).collect()
        }
    }

    /// Returns packages of which this one depends.
    #[inline]
    pub fn depends(&self) -> Vec<String> { self.split_by_comma(self.fields.get("Depends")) }

    /// Returns packages of which the installation of this package depends.
    #[inline]
    pub fn predepends(&self) -> Vec<String> { self.split_by_comma(self.fields.get("Pre-Depends")) }

    /// Returns true if this package us a dependency of other.
    pub fn is_dependency_of(&self, pkg: &Package) -> bool {
        let id = &self.id;
        return pkg.depends().contains(id) || pkg.predepends().contains(id);
    }

    #[allow(dead_code)]
    pub fn get_field<F: AsRef<str>>(&self, field: F) -> Option<&String> {
        self.fields.get(field.as_ref())
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


impl Section {
    pub fn from_string(value: &str) -> Section {
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
            _ => Section::Other(value.to_string())
        }
    }

    pub fn from_string_opt(value: Option<&String>) -> Section {
        match value {
            Some(value) => Self::from_string(value),
            None => Section::Unknown
        }
    }
}

impl Priority {
    pub fn from_string(value: &str) -> Priority {
        match value.to_lowercase().as_str() {
            "optional" => Priority::Optional,
            "required" => Priority::Required,
            "important" => Priority::Important,
            "standard" => Priority::Standard,
            _ => Priority::Unknown,
        }
    }

    pub fn from_string_opt(value: Option<&String>) -> Priority {
        match value {
            Some(value) => Self::from_string(value),
            None => Priority::Unknown
        }
    }
}
