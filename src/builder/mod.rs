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
    sync::Arc
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
    working_dir: PathBuf
}

impl BuildWorker {
    pub fn new<P: AsRef<Path>, D: AsRef<Path>>(
        admin_dir: P,
        pkg: Package,
        destination: D,
        progress: Arc<ProgressBar>
    ) -> Self {
        let name = pkg.canonical_name();
        Self {
            package: pkg, progress,
            admin_dir: admin_dir.as_ref().to_path_buf(),
            destination: destination.as_ref().to_path_buf(),
            working_dir: destination.as_ref().join(name)
        }
    }

    /// Runs worker. Should be executed in a single thread usually
    pub fn run(&self) -> io::Result<PathBuf>  {
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

        return Ok(deb_path);
    }

    /// Archives package files and compresses in a single archive
    fn archive_files(&self, archiver: &mut DebTarArchive) -> io::Result<()> {
        let files = self.package.get_installed_files(&self.admin_dir)?;

        for file in files {
            // Remove root slash because tars don't contain absolute paths
            let name = file.trim_start_matches("/");
            let res = archiver.get_mut().append_path_with_name(&file, name);
            if let Err(error) = res {
                self.progress.println(format!(
                    "[{}] {}", self.package.id,
                    ansi_term::Colour::Yellow.paint(format!("{}", error))
                ));
            }
        }

        return Ok(());
    }

    /// Collects package metadata such as install scripts,
    /// creates control and packages all this together
    fn archive_metadata(&self, archiver: &mut DebTarArchive) -> io::Result<()> {
        // Order in this archive doesn't matter. So we'll add control at first
        archiver.append_new_file("control", &self.package.to_control().as_bytes())?;

        // Then add every matching metadata file in dpkg database dir
        let files = fs::read_dir(self.admin_dir.join("info"))?;
        for entry in files {
            if let Ok(entry) = entry {
                let file_name = entry.file_name().into_string().unwrap();

                // Firstly, reject every file not starting with package id
                if !file_name.starts_with(&self.package.id) { continue; }
                let id_len = self.package.id.len();
                // Then reject every file without dot after package id
                if file_name.chars().skip(id_len).take(1).next().unwrap_or('\0') != '.' {
                    continue;
                }
                let ext = file_name
                    .chars().skip(id_len + 1)
                    .collect::<String>();
                // And skip this two files
                // First one contains package files list
                // Second - md5 sums for every package file. Maybe it shouldn't be rejected
                // but i don't sure
                if ext == "list" || ext == "md5sums" { continue; }

                let res = archiver.get_mut().append_path_with_name(entry.path(), ext);
                if let Err(error) = res {
                    self.progress.println(format!(
                        "[{}] {}", self.package.id,
                        ansi_term::Colour::Yellow.paint(format!("{}", error))
                    ));
                }
            }
        }

        return Ok(());
    }
}
