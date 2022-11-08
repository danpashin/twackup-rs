use crate::{
    error::Result,
    flock::{lock_exclusive, unlock},
    kvparser::Parser,
    package::{Package, Priority, Section},
};
use ansi_term::{ANSIString, Colour};
use indicatif::ProgressBar;
use std::{
    collections::BTreeMap,
    fs,
    path::Path,
    time::{Duration, Instant},
};

pub struct Context {
    start_time: Instant,
}

impl Context {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    pub fn progress_bar(&self, length: u64) -> ProgressBar {
        let progress_bar = ProgressBar::new(length);
        progress_bar.set_style(
            indicatif::ProgressStyle::default_bar()
                .template("{pos}/{len} [{wide_bar:.cyan/blue}] {msg}")
                .unwrap()
                .progress_chars("##-"),
        );

        progress_bar
    }

    /// Returns true if the `Uid` represents privileged user - root. (If it equals zero.)
    #[inline]
    pub fn is_root(&self) -> bool {
        unsafe { libc::getuid() == 0 }
    }

    pub async fn packages<P: AsRef<Path>>(
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

#[inline]
pub fn non_root_warn_msg() -> ANSIString<'static> {
    Colour::Yellow.paint(
        "You seem not to be a root user. It is highly recommended to use root, \
         in other case some operations can fail.",
    )
}
