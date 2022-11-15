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
mod paths;

use crate::{
    error::Result,
    package::{Package, Priority, Section},
    parser::Parser,
};
use lock::Lock;
pub use paths::Paths;
use std::{
    collections::{BTreeMap, HashSet, LinkedList},
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum PackagesSort {
    Identifier,
    Name,
}

#[derive(Clone, Debug)]
pub struct Dpkg {
    pub paths: Paths,
    should_lock: bool,
}

impl Dpkg {
    pub fn new<P: AsRef<Path>>(dpkg_dir: P, should_lock: bool) -> Self {
        Self {
            paths: Paths::new(dpkg_dir),
            should_lock,
        }
    }

    /// Fetches packages from dpkg database
    ///
    /// # Parameters
    /// - `leaves_only` - if packages that aren't dependencies of others should be returned
    ///
    /// # Errors
    /// Returns error if parsing database failed or dpkg directory lock failed
    pub async fn unsorted_packages(&self, leaves_only: bool) -> Result<LinkedList<Package>> {
        // lock database as it can be modified while parsing
        let lock = if self.should_lock {
            Some(Lock::new(&self.paths)?)
        } else {
            None
        };

        let parser = Parser::new(self.paths.status_file())?;
        let packages = parser.parse::<Package>().await;

        // remove database lock as it is not needed
        drop(lock);

        if !leaves_only {
            return Ok(packages);
        }

        // Collect all identifiers package depends on
        // Skip system and required
        let mut ids: HashSet<_> = HashSet::new();
        for pkg in &packages {
            ids.extend(pkg.dependencies());
        }

        // Detect leaves - packages that are not depends of others
        let mut leaves_identifiers: HashSet<String> = packages
            .iter()
            .filter(|pkg| {
                pkg.section != Section::System && pkg.priority != Some(Priority::Required)
            })
            .filter(|pkg| !ids.contains(pkg.id.as_str()))
            .map(|pkg| pkg.id.clone())
            .collect();

        // Hacky solution to not call clone on leaves packages as `ids` borrows them
        let leaves = packages
            .into_iter()
            .filter(|pkg| leaves_identifiers.remove(&pkg.id))
            .collect();

        Ok(leaves)
    }

    /// Fetches packages from dpkg database
    ///
    /// # Parameters
    /// - `leaves_only` - if packages that aren't dependencies of others should be returned
    /// - `sort` - type of sorting packages
    ///
    /// # Errors
    /// Returns error if parsing database failed or dpkg directory lock failed
    pub async fn packages(
        &self,
        leaves_only: bool,
        sort: PackagesSort,
    ) -> Result<BTreeMap<String, Package>> {
        let unsorted = self.unsorted_packages(leaves_only).await?;

        let sorted = unsorted
            .into_iter()
            .map(|pkg| match sort {
                PackagesSort::Identifier => (pkg.id.clone(), pkg),
                PackagesSort::Name => (pkg.human_name().to_string(), pkg),
            })
            .collect();

        Ok(sorted)
    }

    /// Fetches packages info directory contents
    ///
    /// # Errors
    /// Returns error if dpkg directory read failed
    pub fn info_dir_contents(&self) -> Result<HashSet<PathBuf>> {
        Ok(fs::read_dir(self.paths.info_dir())?
            .flatten()
            .filter_map(|entry| match entry.metadata() {
                Ok(metadata) if !metadata.is_dir() => Some(entry.path()),
                _ => None,
            })
            .collect())
    }
}
