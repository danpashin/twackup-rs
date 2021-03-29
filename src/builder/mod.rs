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

use crate::package::Package;
use std::{
    io, fs,
    path::{Path, PathBuf},
    sync::Arc,
    os::unix::ffi::OsStringExt,
};

use indicatif::ProgressBar;
pub mod deb;
use deb::*;

const DEB_COMPRESSION_LEVEL: u32 = 6;

/// Creates DEB from filesystem contents
pub struct BuildWorker {
    pub package: Package,
    pub progress: Arc<ProgressBar>,
    admin_dir: PathBuf,
    destination: PathBuf,
    working_dir: PathBuf,
}

impl BuildWorker {
    pub fn new<P: AsRef<Path>, D: AsRef<Path>>(
        admin_dir: P,
        pkg: Package,
        destination: D,
        progress: Arc<ProgressBar>,
    ) -> Self {
        let name = pkg.canonical_name();
        Self {
            package: pkg,
            progress,
            admin_dir: admin_dir.as_ref().to_path_buf(),
            destination: destination.as_ref().to_path_buf(),
            working_dir: destination.as_ref().join(name),
        }
    }

    /// Runs worker. Should be executed in a single thread usually
    pub fn run(&self) -> io::Result<PathBuf> {
        // Removing all dir contents
        let _ = fs::remove_dir_all(&self.working_dir);
        fs::create_dir(&self.working_dir)?;

        let deb_name = format!("{}.deb", self.package.canonical_name());
        let deb_path = self.destination.join(deb_name);

        let mut deb = Deb::new(&self.working_dir, &deb_path, DEB_COMPRESSION_LEVEL)?;
        self.archive_files(deb.data_mut_ref())?;
        self.archive_metadata(deb.control_mut_ref())?;
        deb.package()?;

        let _ = fs::remove_dir_all(&self.working_dir);

        Ok(deb_path)
    }

    /// Archives package files and compresses in a single archive
    fn archive_files(&self, archiver: &mut DebTarArchive) -> io::Result<()> {
        let files = self.package.get_installed_files(&self.admin_dir)?;

        for file in files {
            // Remove root slash because tars don't contain absolute paths
            let name = file.trim_start_matches('/');
            let res = archiver.get_mut().append_path_with_name(&file, name);
            if let Err(error) = res {
                self.progress.println(format!(
                    "[{}] {}", self.package.id,
                    ansi_term::Colour::Yellow.paint(format!("{}", error))
                ));
            }
        }

        Ok(())
    }

    /// Collects package metadata such as install scripts,
    /// creates control and packages all this together
    fn archive_metadata(&self, archiver: &mut DebTarArchive) -> io::Result<()> {
        // Order in this archive doesn't matter. So we'll add control at first
        archiver.append_new_file("control", &self.package.to_control().as_bytes())?;

        // Then add every matching metadata file in dpkg database dir
        let pid_len = self.package.id.len();
        let package_id = self.package.id.as_bytes();
        for entry in fs::read_dir(self.admin_dir.join("info"))?.flatten() {
            let file_name = entry.file_name().into_vec();

            // Reject every file not starting with package id + dot
            if file_name.len() <= pid_len + 1 { continue; }
            if &file_name[..pid_len] != package_id || file_name[pid_len] != b'.' {
                continue;
            }
            let ext = unsafe { std::str::from_utf8_unchecked(&file_name[pid_len + 1..]) };
            // And skip .list file as it contains package files list
            if ext == "list" { continue; }

            let res = archiver.get_mut().append_path_with_name(entry.path(), ext);
            if let Err(error) = res {
                self.progress.println(format!(
                    "[{}] {}", self.package.id,
                    ansi_term::Colour::Yellow.paint(format!("{}", error))
                ));
            }
        }

        Ok(())
    }
}
