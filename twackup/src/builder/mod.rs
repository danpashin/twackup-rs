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

pub mod deb;

use crate::{
    dpkg::Paths,
    error::{Generic, Result},
    package::Package,
    progress::Progress,
};
use deb::{Deb, DebianTarArchive, TarArchive};
use std::{
    collections::HashSet,
    fs, io,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::{fs::File, sync::Mutex};

pub type AllPackagesArchive = Arc<Mutex<TarArchive<File>>>;

#[derive(Clone)]
pub struct Preferences {
    pub remove_deb: bool,
    pub paths: Paths,
    pub destination: PathBuf,
    pub compression_level: u32,
}

/// Creates DEB from filesystem contents
pub struct Worker<T: Progress> {
    pub package: Package,
    pub progress: T,
    pub archive: Option<AllPackagesArchive>,
    pub preferences: Preferences,
    pub dpkg_contents: Arc<HashSet<PathBuf>>,
}

impl Preferences {
    pub fn new<A: AsRef<Path>, D: AsRef<Path>>(admin_dir: A, destination: D) -> Self {
        Self {
            remove_deb: true,
            paths: Paths::new(admin_dir),
            destination: destination.as_ref().to_path_buf(),
            compression_level: 0,
        }
    }
}

impl<T: Progress> Worker<T> {
    pub fn new(
        package: Package,
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
    pub async fn run(&self) -> io::Result<PathBuf> {
        let deb_name = format!("{}.deb", self.package.canonical_name());
        let deb_path = self.preferences.destination.join(deb_name);

        let mut deb = Deb::new(&deb_path, self.preferences.compression_level);
        self.archive_files(deb.data_mut_ref()).await?;
        self.archive_metadata(deb.control_mut_ref()).await?;
        deb.build().await?;

        Ok(deb_path)
    }

    /// Archives package files and compresses in a single archive
    ///
    /// # Errors
    /// Returns error if dpkg directory couldn't be read or any of underlying operation failed
    async fn archive_files(&self, archiver: &mut DebianTarArchive) -> io::Result<()> {
        let files = self
            .package
            .get_installed_files(self.preferences.paths.as_ref())?;

        for file in files {
            // Remove root slash because tars don't contain absolute paths
            let name = file.trim_start_matches('/');
            let res = archiver.get_mut().append_path_with_name(&file, name).await;
            if let Err(error) = res {
                log::warn!("[{}] {}", self.package.id, error);
            }
        }

        Ok(())
    }

    /// Collects package metadata such as install scripts,
    /// creates control and packages all this together
    ///
    /// # Errors
    /// Returns error if control file couldn't be appended
    async fn archive_metadata(&self, archiver: &mut DebianTarArchive) -> io::Result<()> {
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
        ];

        let contents = self.dpkg_contents.iter().filter_map(|entry| {
            let file_name = entry.file_name()?.to_str()?;
            let rem = file_name.strip_prefix(&self.package.id)?;
            let rem = rem.strip_prefix('.')?;
            if possible_extensions.contains(&rem) {
                Some((entry, rem))
            } else {
                None
            }
        });

        for (path, ext) in contents {
            let res = archiver.get_mut().append_path_with_name(path, ext).await;
            if let Err(error) = res {
                log::warn!("[{}] {}", self.package.id, error);
            }
        }

        Ok(())
    }

    /// Creates package debian archive and optionally add it to shared TAR archive
    ///
    /// # Errors
    /// Returns error if any of underlying operations failed
    pub async fn work(&self) -> Result<()> {
        let progress = format!("Processing {}", self.package.human_name());
        self.progress.set_message(progress);

        let file = self.run().await?;
        self.progress.increment(1);

        let progress = format!("Done {}", self.package.human_name());
        self.progress.set_message(progress);

        self.add_to_archive(file).await?;

        Ok(())
    }

    /// Adds already assembled package to common TAR archive
    ///
    /// # Errors
    /// Returns error if any of underlying operations failed
    async fn add_to_archive(&self, file: PathBuf) -> Result<()> {
        if let Some(archive) = &self.archive {
            let mut archive = archive.try_lock().map_err(|_| Generic::LockError)?;

            let abs_file = Path::new(".").join(file.file_name().unwrap());
            archive
                .get_mut()
                .append_path_with_name(&file, &abs_file)
                .await?;

            if self.preferences.remove_deb {
                fs::remove_file(file).ok();
            }
        }

        Ok(())
    }
}
