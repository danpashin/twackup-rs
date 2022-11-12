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

pub(crate) mod export;
pub(crate) mod import;

use serde::{Deserialize, Serialize};
use twackup::repository::Repository;

const MODERN_MANAGERS: &[(&str, &str)] = &[("Sileo", "/etc/apt/sources.list.d/sileo.sources")];

const CLASSIC_MANAGERS: &[(&str, &str)] = &[
    (
        "Cydia",
        "/var/mobile/Library/Caches/com.saurik.Cydia/sources.list",
    ),
    (
        "Zebra",
        "/var/mobile/Library/Application Support/xyz.willy.Zebra/sources.list",
    ),
];

/// Describes what data should be used for exporting or importing
#[derive(clap::Parser, clap::ValueEnum, PartialEq, Debug, Clone)]
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
    sources: Vec<Repository>,
}

#[derive(Serialize, Deserialize)]
struct DataLayout {
    packages: Option<Vec<String>>,
    repositories: Option<Vec<RepoGroup>>,
}
