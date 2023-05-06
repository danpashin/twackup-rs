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

//! Builder is a work-in-progress Twackup module
//! that rebuilds installed package back into debian archive
//!
//! ### Example usage
//!
//! ```no_run
//! use twackup::builder::{Worker, Preferences};
//! use twackup::{Result, Dpkg, progress::Progress, package::Package};
//! use std::{collections::HashSet, sync::Arc, path::Path};
//!
//! // some progress struct btw
//! struct ProgressImpl;
//!
//! impl Progress for ProgressImpl {
//!     fn started_processing(&self, _package: &Package) {}
//!     fn finished_processing<P: AsRef<Path>>(&self, _package: &Package, _deb_path: P) {}
//!     fn finished_all(&self) {}
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let dpkg_dir = "/var/lib/dpkg";
//!     // simple way to get package
//!     let package = get_package(dpkg_dir).await?;
//!    
//!     let preferences = Preferences::new(dpkg_dir, "/tmp");
//!     let progress = ProgressImpl;
//!     let dpkg_contents = Arc::new(HashSet::new());
//!
//!     let worker = Worker::new(&package, progress, None, preferences, dpkg_contents);
//!     let deb_path = worker.run().await?;
//!     println!("Deb is located at {:?}", deb_path);
//!
//!     Ok(())
//! }
//!
//! async fn get_package(dpkg_dir: &str) -> Result<Package> {
//!     let dpkg = Dpkg::new(dpkg_dir, false);
//!     let mut packages = dpkg.unsorted_packages(false).await?;
//!     let package = packages.pop_back().expect("Packages must not be null");
//!     Ok(package)
//! }
//! ```
//!

mod deb;

use crate::{
    archiver::Compression,
    dpkg::Paths,
    error::{Generic, Result},
    package::Package,
    progress::Progress,
};
use deb::{Deb, DebianInnerTar};
use std::{
    collections::HashSet,
    fs, io,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::{fs::File, sync::Mutex};

/// Alias for creating archive for all packaged files. Probably should be removed
pub type AllPackagesArchive = Arc<Mutex<tokio_tar::Builder<File>>>;

/// Builder preferences
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Preferences {
    /// Should remove deb after packaging. Probably should be removed
    /// false by default
    pub remove_deb: bool,
    /// Compression type and level
    /// Gzip and 6 level by default
    pub compression: Compression,
    /// Should follow symlinks while creating deb or not.
    /// Disabling can produce broken debs.
    pub follow_symlinks: bool,
    /// Dpkg dir paths
    paths: Paths,
    /// Directory to which final deb should be moved
    destination_dir: PathBuf,
}

/// Creates DEB from filesystem contents
#[non_exhaustive]
pub struct Worker<'a, T: Progress> {
    package: &'a Package,
    progress: T,
    archive: Option<AllPackagesArchive>,
    preferences: Preferences,
    dpkg_contents: Arc<HashSet<PathBuf>>,
}

impl Preferences {
    /// Constructs preferences instance
    ///
    /// # Parameters
    /// - `admin_dir` - Directory with dpkg database. Typically **/var/lib/dpkg**
    /// - `destination_dir` - Directory to which final deb package should be placed
    #[inline]
    pub fn new<A: Into<Paths>, D: AsRef<Path>>(admin_dir: A, destination_dir: D) -> Self {
        Self {
            remove_deb: false,
            compression: Compression::default(),
            follow_symlinks: false,
            paths: admin_dir.into(),
            destination_dir: destination_dir.as_ref().to_path_buf(),
        }
    }
}

impl<'a, T: Progress> Worker<'a, T> {
    /// Constructs worker instance
    ///
    /// # Parameters
    /// - `package` - Package model which needs to be rebuild
    /// - `progress` - Progress to which worker will send it's status
    /// - `archive` - Tar instance if final deb should be placed in common archive
    /// - `preferences` - Worker preferences
    /// - `dpkg_contents` - **/var/lib/dpkg/info** contents. Used for IO optimization
    #[inline]
    pub fn new(
        package: &'a Package,
        progress: T,
        archive: Option<AllPackagesArchive>,
        preferences: Preferences,
        dpkg_contents: Arc<HashSet<PathBuf>>,
    ) -> Self {
        Self {
            package,
            progress,
            archive,
            preferences,
            dpkg_contents,
        }
    }

    /// Runs worker
    ///
    /// # Errors
    /// Returns error if temp dir creation or any of underlying package operation failed
    #[inline]
    pub async fn run(&self) -> Result<PathBuf> {
        self.progress.started_processing(self.package);

        let deb_name = format!("{}.deb", self.package.canonical_name());
        let deb_path = self.preferences.destination_dir.join(deb_name);

        let mut deb = Deb::new(
            &deb_path,
            self.preferences.compression,
            self.preferences.follow_symlinks,
        )?;
        self.archive_files(deb.data_mut_ref()).await?;
        self.archive_metadata(deb.control_mut_ref()).await?;
        deb.build().await?;

        self.add_to_archive(&deb_path).await?;

        self.progress.finished_processing(self.package, &deb_path);
        Ok(deb_path)
    }

    /// Archives package files and compresses in a single archive
    ///
    /// # Errors
    /// Returns error if dpkg directory couldn't be read or any of underlying operation failed
    async fn archive_files(&self, archiver: &mut DebianInnerTar) -> io::Result<()> {
        let files = self
            .package
            .get_installed_files(self.preferences.paths.as_ref())?;

        for file in files {
            // Remove root slash because tars don't contain absolute paths
            let name = file.trim_start_matches('/');
            let res = archiver.get_mut().append_path_with_name(&file, name).await;
            if let Err(error) = res {
                log::warn!(target: &self.package.id, "{}", error);
            }
        }

        Ok(())
    }

    /// Collects package metadata such as install scripts,
    /// creates control and packages all this together
    ///
    /// # Errors
    /// Returns error if control file couldn't be appended
    async fn archive_metadata(&self, archiver: &mut DebianInnerTar) -> Result<()> {
        // Order in this archive doesn't matter. So we'll add control at first
        archiver
            .append_new_file("control", self.package.to_control().as_bytes())
            .await?;

        let possible_extensions = [
            "md5sums",
            "preinst",
            "postinst",
            "prerm",
            "postrm",
            "extrainst_",
            "conffiles",
            "config",
            "shlibs",
        ];

        let contents = self.dpkg_contents.iter().filter_map(|entry| {
            let file_name = entry.file_name()?.to_str()?;
            let rem = file_name.strip_prefix(&self.package.id)?;
            let rem = rem.strip_prefix('.')?;
            possible_extensions.contains(&rem).then_some((entry, rem))
        });

        for (path, ext) in contents {
            let res = archiver.get_mut().append_path_with_name(path, ext).await;
            if let Err(error) = res {
                log::warn!(target: &self.package.id, "{}", error);
            }
        }

        Ok(())
    }

    /// Adds already assembled package to common TAR archive
    ///
    /// # Errors
    /// Returns error if any of underlying operations failed
    async fn add_to_archive(&self, file: &PathBuf) -> Result<()> {
        if let Some(ref archive) = self.archive {
            let mut archive = archive.try_lock().map_err(|_| Generic::Lock)?;

            let file_name = file.file_name().ok_or(Generic::PathMustHaveFileEnding)?;
            let abs_file = Path::new(".").join(file_name);

            archive.append_path_with_name(&file, &abs_file).await?;

            if self.preferences.remove_deb {
                fs::remove_file(file).ok();
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        builder::{Preferences, Worker},
        package::Package,
        progress::Progress,
        Dpkg, Result,
    };
    use std::{fs, path::Path, process::Command, sync::Arc};

    struct ProgressImpl;

    impl Progress for ProgressImpl {
        fn started_processing(&self, _package: &Package) {}
        fn finished_processing<P: AsRef<Path>>(&self, _package: &Package, _deb_path: P) {}
        fn finished_all(&self) {}
    }

    #[tokio::test]
    async fn package_build_correctness() -> Result<()> {
        let dpkg_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/dpkg_database_dir");
        let dpkg = Dpkg::new(dpkg_dir, false);

        let mut packages = dpkg.unsorted_packages(false).await?;
        let package = packages.pop_back().unwrap();

        let dpkg_contents = Arc::new(dpkg.info_dir_contents()?);
        let preferences = Preferences::new(dpkg_dir, "/tmp");

        let worker = Worker::new(&package, ProgressImpl, None, preferences, dpkg_contents);
        let deb_path = worker.run().await?;

        let dpkg = Command::new("dpkg")
            .args(["-I", deb_path.to_str().unwrap()])
            .output()?;
        let output = String::from_utf8(dpkg.stdout).unwrap();

        assert!(output.contains("Package: hosts"));
        assert!(output.contains("Version: 1.0.0"));
        assert!(output.contains("preinst"));

        let dpkg = Command::new("dpkg")
            .args(["-c", deb_path.to_str().unwrap()])
            .output()?;
        let output = String::from_utf8(dpkg.stdout).unwrap();

        assert!(output.contains("etc/hosts"));

        fs::remove_file(deb_path)?;

        Ok(())
    }
}
