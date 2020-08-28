
pub mod export;
pub mod import;

use std::collections::LinkedList;
use clap::Clap;
use serde::{Deserialize, Serialize};
use crate::repository::Repository;

const MODERN_MANAGERS: &'static [(&'static str, &'static str)] = &[
    ("Sileo", "/etc/apt/sources.list.d/sileo.sources")
];

const CLASSIC_MANAGERS: &'static [(&'static str, &'static str)] = &[
    ("Cydia", "/var/mobile/Library/Caches/com.saurik.Cydia/sources.list"),
    ("Zebra", "/var/mobile/Library/Application Support/xyz.willy.Zebra/sources.list"),
];

/// Data format used for export and import commands
#[derive(Clap, PartialEq)]
enum DataFormat {
    Json,
    Toml,
    Yaml,
}

/// Describes what data should be used for exporting or importing
#[derive(Clap, PartialEq, Debug)]
enum DataType {
    Packages,
    Repositories,
    All,
}

/// Package manager repos data format
#[derive(PartialEq, Serialize, Deserialize)]
enum RepoGroupFormat {
    /// Store repo data in DEB822 format
    Modern,

    /// Store repo data in one line format
    Classic,
}

#[derive(Serialize, Deserialize)]
struct RepoGroup {
    format: RepoGroupFormat,
    path: String,
    executable: String,
    sources: LinkedList<Repository>,
}

#[derive(Serialize, Deserialize)]
struct DataLayout {
    packages: Option<LinkedList<String>>,
    repositories: Option<LinkedList<RepoGroup>>,
}

impl DataFormat {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Json => "json",
            Self::Toml => "toml",
            Self::Yaml => "yaml",
        }
    }

    fn to_serde(&self) -> serde_any::Format {
        match self {
            Self::Json => serde_any::Format::Json,
            Self::Toml => serde_any::Format::Toml,
            Self::Yaml => serde_any::Format::Yaml,
        }
    }
}
