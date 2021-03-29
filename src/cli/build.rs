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

use std::{
    path::{Path, PathBuf},
    io, fs, time::Instant,
    sync::{Arc, Mutex},
};
use clap::Clap;
use chrono::Local;
use ansi_term::Colour;
use gethostname::gethostname;

use crate::{package::*, builder::*};
use super::{
    ADMIN_DIR, TARGET_DIR, CLICommand, utils::{self, get_packages},
};

const DEFAULT_ARCHIVE_NAME: &str = "%host%_%date%.tar.gz";

#[derive(Clap)]
#[clap(version, after_help = "
Beware, this command doesn't guarantee to copy all files to the final DEB! \
Some files can be skipped because of being renamed or removed in the installation process.
If you see yellow warnings, it means the final deb will miss some contents \
and may not work properly anymore.
")]
pub struct Build {
    /// By default twackup rebuilds only that packages which are not dependencies of others.
    /// This flag disables this restriction - command will rebuild all found packages.
    #[clap(short, long)]
    all: bool,

    /// Use custom dpkg <directory>.
    /// This option is used for detecting installed packages
    #[clap(long, default_value = ADMIN_DIR, parse(from_os_str))]
    admindir: PathBuf,

    /// Package identifier or number from the list command.
    /// This argument can have multiple values separated by space ' '.
    packages: Vec<String>,

    /// Use custom destination <directory>.
    #[clap(long, short, default_value = TARGET_DIR, parse(from_os_str))]
    destination: PathBuf,

    /// Packs all rebuilded DEB's to single archive
    #[clap(short = 'A', long)]
    archive: bool,

    /// Name of archive if --archive is set. Supports only .tar.gz archives for now.
    #[clap(long, default_value = DEFAULT_ARCHIVE_NAME)]
    archive_name: String,

    /// Removes all DEB's after adding to archive. Makes sense only if --archive is set.
    #[clap(short = 'R', long)]
    remove_after: bool,
}

impl Build {
    fn build_user_specified(&self) {
        let mut all_packages = get_packages(&self.admindir, false);
        all_packages.sort_by(|a, b| {
            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        });

        let mut to_build: Vec<Package> = Vec::with_capacity(self.packages.len());

        for package_id in self.packages.iter() {
            if let Ok(human_pos) = package_id.parse::<usize>() {
                match all_packages.get(human_pos - 1) {
                    Some(pkg) => to_build.push(pkg.clone()),
                    None => {
                        match all_packages.iter().find(|pkg| pkg.id == *package_id) {
                            Some(pkg) => to_build.push(pkg.clone()),
                            None => eprintln!("Can't find any package with name or index {}", package_id)
                        }
                    }
                }
            } else {
                match all_packages.iter().find(|pkg| pkg.id == *package_id) {
                    Some(pkg) => to_build.push(pkg.clone()),
                    None => eprintln!("Can't find any package with name or index {}", package_id)
                }
            }
        }

        self.build(to_build);
    }

    fn build(&self, packages: Vec<Package>) {
        let started = Instant::now();
        self.create_dir_if_needed();
        let threadpool = threadpool::ThreadPool::default();

        let all_count = packages.len();
        let pb = indicatif::ProgressBar::new(all_count as u64);
        pb.set_style(
            indicatif::ProgressStyle::default_bar()
                .template("{pos}/{len} [{wide_bar:.cyan/blue}] {msg}")
                .progress_chars("##-")
        );
        let progress_bar = Arc::new(pb);

        if !utils::is_root() {
            progress_bar.println(utils::non_root_warn_msg().to_string());
        }

        let archive = self.create_archive_if_needed();
        let archive_ptr = Arc::new(Mutex::new(archive));

        for package in packages {
            let builder = BuildWorker::new(
                &self.admindir, package, &self.destination, Arc::clone(&progress_bar)
            );
            let b_archive_ptr = Arc::clone(&archive_ptr);
            let perform_archive = self.archive;
            let remove_deb = self.remove_after;
            threadpool.execute(move || {
                builder.progress.set_message(
                    format!("Processing {}", builder.package.name).as_str()
                );
                let status = builder.run();
                builder.progress.inc(1);
                if let Err(error) = status {
                    builder.progress.println(Colour::Red.paint(
                        format!("Building {} error. {}", builder.package.name, error)
                    ).to_string());
                } else {
                    builder.progress.set_message(
                        format!("Done {}", builder.package.name).as_str()
                    );

                    if perform_archive {
                        let mut archive = b_archive_ptr.lock().unwrap();
                        let file = status.unwrap();
                        let abs_file = Path::new(".").join(file.file_name().unwrap());
                        let result = archive.as_mut().unwrap().get_mut()
                            .append_path_with_name(&file, &abs_file);
                        if result.is_ok() && remove_deb {
                            let _ = fs::remove_file(file);
                        }
                    }
                }
            });
        }

        threadpool.join();
        progress_bar.finish_and_clear();
        println!(
            "Processed {} packages in {}",
            all_count, indicatif::HumanDuration(started.elapsed())
        );
    }

    fn create_archive_if_needed(&self) -> Option<deb::DebTarArchive> {
        if !self.archive {
            return None;
        }

        let filename = if self.archive_name == DEFAULT_ARCHIVE_NAME {
            format!("{}_{}.tar.gz", gethostname().to_str().unwrap(), Local::now().format("%v_%T"))
        } else {
            self.archive_name.clone()
        };

        let filepath = self.destination.join(&filename);
        let file = fs::File::create(filepath).expect("Can't open archive file");
        let compression = flate2::Compression::default();
        let encoder = flate2::write::GzEncoder::new(file, compression);

        Some(deb::TarArchive::new(encoder))
    }

    fn create_dir_if_needed(&self) {
        if let Ok(metadata) = fs::metadata(&self.destination) {
            if !metadata.is_dir() {
                fs::remove_file(&self.destination).expect("Failed to remove working dir");
            }

            return;
        }

        fs::create_dir_all(&self.destination).expect("Failed to create working dir");
    }
}

impl CLICommand for Build {
    fn run(&self) {
        if !self.packages.is_empty() {
            self.build_user_specified();
        } else if !self.all {
            eprint!("No packages specified. Build leaves? [Y/N] [default N] ");

            let mut buffer = String::new();
            let _ = io::stdin().read_line(&mut buffer);
            if buffer.trim().to_lowercase() == "y" {
                self.build(get_packages(&self.admindir, true));
            } else {
                eprintln!("Ok, cancelling...");
            }
        } else {
            self.build(get_packages(&self.admindir, false));
        }
    }
}
