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

use super::{Hooks, OldStylePackageManager, PackageManagerDescription};
use crate::{commands::backup::RepoGroup, Result};
use plist::Value as PValue;
use std::{io, path::PathBuf};
use twackup::repository::Repository;

pub(crate) struct Cydia {
    pub(crate) binary_name: &'static str,
    pub(crate) sources: &'static str,
    prefs_path: &'static str,
}

impl Cydia {
    pub(crate) const fn new() -> Self {
        Self {
            binary_name: "Cydia",
            sources: "/var/mobile/Library/Caches/com.saurik.Cydia/sources.list",
            prefs_path: "/var/mobile/Library/Preferences/com.saurik.Cydia.plist",
        }
    }

    fn repo_to_dictionary(repo: &Repository) -> PValue {
        let mut dict = plist::Dictionary::new();
        dict.insert(
            "Distribution".to_owned(),
            PValue::String(repo.distribution.clone()),
        );
        dict.insert("URI".to_owned(), PValue::String(repo.url.clone()));
        dict.insert(
            "Type".to_owned(),
            PValue::String(repo.category.as_str().to_owned()),
        );
        dict.insert(
            "Sections".to_owned(),
            PValue::Array(
                repo.components
                    .iter()
                    .map(|val| PValue::String(val.clone()))
                    .collect(),
            ),
        );

        PValue::Dictionary(dict)
    }
}

impl Hooks for Cydia {
    fn post_import(&self, repo_group: &RepoGroup) -> Result<()> {
        let mut prefs = PValue::from_file(self.prefs_path)?;

        let prefs_dict = prefs
            .as_dictionary_mut()
            .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidInput))?;

        let sources: plist::Dictionary = repo_group
            .sources
            .iter()
            .map(|src| {
                let key = format!("{}:{}:{}", src.category.as_str(), src.url, src.distribution);
                (key, Self::repo_to_dictionary(src))
            })
            .collect();

        prefs_dict.insert("CydiaSources".to_string(), PValue::Dictionary(sources));

        Ok(prefs.to_file_binary(self.prefs_path)?)
    }
}

impl OldStylePackageManager for Cydia {}

impl PackageManagerDescription for Cydia {
    fn exec_name(&self) -> &str {
        self.binary_name
    }

    fn repos_file_path(&self) -> PathBuf {
        PathBuf::from(self.sources)
    }
}
