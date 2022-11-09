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

use super::progress_bar::ProgressBar;
use std::{
    collections::BTreeMap,
    fs,
    path::Path,
    time::{Duration, Instant},
};
use twackup::{
    error::Result,
    flock::{lock_exclusive, unlock},
    kvparser::Parser,
    package::{Package, Priority, Section},
};

pub(crate) struct Context {
    start_time: Instant,
}

impl Context {
    pub(crate) fn new() -> Self {
        Self {
            start_time: Instant::now(),
        }
    }

    pub(crate) fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    pub(crate) fn progress_bar(&self, length: u64) -> &'static ProgressBar {
        let progress_bar = indicatif::ProgressBar::new(length);
        progress_bar.set_style(
            indicatif::ProgressStyle::default_bar()
                .template("{pos}/{len} [{wide_bar:.cyan/blue}] {msg}")
                .expect("Progress bar template error!")
                .progress_chars("##-"),
        );

        let progress_bar = ProgressBar(progress_bar);
        progress_bar.make_static()
    }

    /// Returns true if the `Uid` represents privileged user - root. (If it equals zero.)
    #[inline]
    pub(crate) fn is_root(&self) -> bool {
        unsafe { libc::getuid() == 0 }
    }

    pub(crate) async fn packages<P: AsRef<Path>>(
        &self,
        admin_dir: P,
        leaves_only: bool,
    ) -> Result<BTreeMap<String, Package>> {
        // lock database as it can be modified while parsing
        let lock_file_path = admin_dir.as_ref().join("lock");
        let lock_file = fs::File::create(&lock_file_path)?;
        lock_exclusive(&lock_file)?;

        let status_file = admin_dir.as_ref().join("status");
        let parser = Parser::new(status_file)?;

        let packages = parser.parse::<Package>().await;

        // remove database lock as it is not needed anymore
        let _ = unlock(&lock_file);
        let _ = fs::remove_file(lock_file_path);

        if !leaves_only {
            return Ok(packages
                .into_iter()
                .map(|pkg| (pkg.id.clone(), pkg))
                .collect());
        }

        let mut leaves_indexes = Vec::with_capacity(packages.len());
        for (index, package) in packages.iter().enumerate() {
            // Skip package if it is system
            if package.section == Section::System || package.priority == Priority::Required {
                continue;
            }
            // Skip this package if it is the dependency of other
            let mut is_dependency = false;
            for pkg in packages.iter() {
                if package.is_dependency_of(pkg) {
                    is_dependency = true;
                    break;
                }
            }
            // Save index to filter packages in further
            if !is_dependency {
                leaves_indexes.push(index);
            }
        }

        Ok(packages
            .into_iter()
            .enumerate()
            .filter(|(index, _)| leaves_indexes.contains(index))
            .map(|(_, pkg)| (pkg.id.clone(), pkg))
            .collect())
    }
}
