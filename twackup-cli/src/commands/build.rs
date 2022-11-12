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

use super::CliCommand;
use crate::{context::Context, error::Result, ADMIN_DIR, TARGET_DIR};
use chrono::Local;
use gethostname::gethostname;
use std::{
    collections::LinkedList,
    fs, io,
    iter::Iterator,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use twackup::{
    builder::{
        deb::{DebianTarArchive, TarArchive},
        Preferences, Worker,
    },
    dpkg::Dpkg,
    error::Generic as GenericError,
    package::Package,
    progress::Progress,
};

const DEFAULT_ARCHIVE_NAME: &str = "%host%_%date%.tar.gz";

#[derive(clap::Parser)]
#[clap(
    version,
    after_help = "
Beware, this command doesn't guarantee to copy all files to the final DEB!
Some files can be skipped because of being renamed or removed in the installation process.
If you see yellow warnings, it means the final deb will miss some contents
and may not work properly anymore.
"
)]
pub(crate) struct Build {
    /// By default twackup rebuilds only that packages which are not dependencies of others.
    /// This flag disables this restriction - command will rebuild all found packages.
    #[clap(short, long)]
    all: bool,

    /// Use custom dpkg <directory>.
    /// This option is used for detecting installed packages
    #[clap(long, default_value = ADMIN_DIR, value_parser)]
    admindir: PathBuf,

    /// Package identifier or number from the list command.
    /// This argument can have multiple values separated by space ' '.
    packages: Vec<String>,

    /// Use custom destination <directory>.
    #[clap(long, short, default_value = TARGET_DIR, value_parser)]
    destination: PathBuf,

    /// Packs all rebuilded DEB's to single archive
    #[clap(short = 'A', long, default_value_t = false)]
    archive: bool,

    /// Name of archive if --archive is set. Supports only .tar.gz archives for now.
    #[clap(long, default_value = DEFAULT_ARCHIVE_NAME)]
    archive_name: String,

    /// Removes all DEB's after adding to archive. Makes sense only if --archive is set.
    #[clap(long, short = 'R', default_value_t = false)]
    remove_after: bool,

    /// DEB Compression level. 0 means no compression while 9 - strongest. Default is 6
    #[clap(long, short = 'c', default_value_t = 6)]
    compression_level: u32,
}

impl Build {
    async fn build_user_specified(&self, context: Context) -> Result<()> {
        let all_packages = context.packages(&self.admindir, false).await?;

        // Try to detect if user package is numeric or not
        let packages: LinkedList<_> = self
            .packages
            .iter()
            .map(|identifier| identifier.parse::<usize>().map_err(|_| identifier))
            .collect();

        // If numeric - add package by its index
        let numeric_packages = packages
            .iter()
            .filter_map(|id| id.as_ref().ok())
            .filter_map(|&id| {
                if let Some((_, package)) = all_packages.iter().nth(id - 1) {
                    Some(package.clone())
                } else {
                    log::warn!("Can't find any package at index {}", id);
                    None
                }
            });

        // If not numeric - add by its identifier
        let alphabetic_packages =
            packages
                .iter()
                .filter_map(|x| x.as_ref().err())
                .filter_map(|&id| {
                    if let Some(package) = all_packages.get(id) {
                        Some(package.clone())
                    } else {
                        log::warn!("Can't find any package with identifier {}", id);
                        None
                    }
                });

        let to_build = numeric_packages.chain(alphabetic_packages).collect();
        self.build(to_build, context).await
    }

    async fn build(&self, packages: Vec<Package>, context: Context) -> Result<()> {
        self.create_dir_if_needed()?;

        let all_count = packages.len() as u64;
        let progress = context.progress_bar(all_count);

        if !context.is_root() {
            log::warn!("{}", GenericError::NotRunningAsRoot);
        }

        let archive = self.create_archive_if_needed();

        let mut preferences = Preferences::new(&self.admindir, &self.destination);
        preferences.remove_deb = self.remove_after;
        preferences.compression_level = self.compression_level;

        let contents = Dpkg::new(&self.admindir, false).info_dir_contents()?;
        let contents = Arc::new(contents);

        futures::future::join_all(packages.into_iter().map(|package| {
            let progress = progress.clone();
            let archive = archive.clone();
            let preferences = preferences.clone();
            let contents = contents.clone();

            tokio::spawn(async move {
                let builder = Worker::new(package, progress, archive, preferences, contents);
                builder.work()
            })
        }))
        .await;

        progress.finish();
        log::info!(
            "Processed {} packages in {}",
            all_count,
            indicatif::HumanDuration(context.elapsed())
        );

        Ok(())
    }

    fn create_archive_if_needed(&self) -> Option<Arc<Mutex<DebianTarArchive>>> {
        if !self.archive {
            return None;
        }

        let filename = match &*self.archive_name {
            DEFAULT_ARCHIVE_NAME => format!(
                "{}_{}.tar.gz",
                gethostname().to_str().unwrap_or_default(),
                Local::now().format("%v_%T")
            ),
            _ => self.archive_name.clone(),
        };

        let filepath = self.destination.join(&filename);
        let file = fs::File::create(filepath).expect("Can't open archive file");
        let compression = flate2::Compression::default();
        let encoder = flate2::write::GzEncoder::new(file, compression);

        let archive = TarArchive::new(encoder);
        Some(Arc::new(Mutex::new(archive)))
    }

    fn create_dir_if_needed(&self) -> Result<()> {
        match fs::metadata(&self.destination) {
            Ok(metadata) if !metadata.is_dir() => fs::remove_file(&self.destination)?,
            _ => {}
        }

        Ok(fs::create_dir_all(&self.destination)?)
    }
}

#[async_trait::async_trait]
impl CliCommand for Build {
    async fn run(&self, context: Context) -> Result<()> {
        if !self.packages.is_empty() {
            return self.build_user_specified(context).await;
        }

        let mut leaves_only = false;
        if !self.all {
            eprint!("No packages specified. Build leaves? [Y/N] [default N] ");

            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer)?;
            if buffer.trim().to_lowercase() == "y" {
                leaves_only = true;
            } else {
                eprintln!("Ok, cancelling...");
            }
        }

        let packages = context.packages(&self.admindir, leaves_only).await?;
        let packages = packages.into_iter().map(|(_, pkg)| pkg).collect();
        self.build(packages, context).await
    }
}
