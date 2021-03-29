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

pub mod export;
pub mod import;

use std::collections::LinkedList;
use clap::Clap;
use serde::{Deserialize, Serialize};
use crate::repository::Repository;

const MODERN_MANAGERS: &[(&str, &str)] = &[
    ("Sileo", "/etc/apt/sources.list.d/sileo.sources")
];

const CLASSIC_MANAGERS: &[(&str, &str)] = &[
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
