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

mod lock;

use crate::{
    error::Result,
    kvparser::Parser,
    package::{Package, Priority, Section},
};
use lock::Lock;
use std::{
    collections::{BTreeMap, HashSet},
    path::{Path, PathBuf},
};

pub struct Dpkg {
    dpkg_dir: PathBuf,
    should_lock: bool,
}

impl Dpkg {
    pub fn new<P: AsRef<Path>>(dpkg_dir: P, should_lock: bool) -> Self {
        Self {
            dpkg_dir: dpkg_dir.as_ref().to_path_buf(),
            should_lock,
        }
    }

    pub async fn packages(&self, leaves_only: bool) -> Result<BTreeMap<String, Package>> {
        // lock database as it can be modified while parsing
        let lock = if self.should_lock {
            Some(Lock::new(&self.dpkg_dir))
        } else {
            None
        };

        let status_file = self.dpkg_dir.join("status");
        let parser = Parser::new(status_file)?;

        let packages = parser.parse::<Package>().await;

        // remove database lock as it is not needed
        drop(lock);

        if !leaves_only {
            return Ok(packages
                .into_iter()
                .map(|pkg| (pkg.id.clone(), pkg))
                .collect());
        }

        // Collect all identifiers package depends on
        // Skip system and required
        let mut ids: HashSet<&str> = HashSet::new();
        for pkg in packages.iter() {
            if pkg.section != Section::System && pkg.priority != Priority::Required {
                ids.extend(pkg.dependencies());
            }
        }

        // Detect leaves - packages that are not depends of others
        let mut leaves_identifiers: HashSet<String> = packages
            .iter()
            .filter(|pkg| pkg.section != Section::System && pkg.priority != Priority::Required)
            .filter(|pkg| !ids.contains(pkg.id.as_str()))
            .map(|pkg| pkg.id.clone())
            .collect();

        // Hacky solution to not call clone on leaves packages as `ids` borrows them
        let leaves = packages
            .into_iter()
            .filter_map(|pkg| leaves_identifiers.take(&pkg.id).map(|id| (id, pkg)))
            .collect();

        Ok(leaves)
    }
}
