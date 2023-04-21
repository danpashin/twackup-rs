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

use super::{CliCommand, GlobalOptions};
use crate::{error::Result, paths, progress_bar::ProgressBar};
use chrono::Local;
use console::style;
use gethostname::gethostname;
use libproc::libproc::proc_pid::am_root;
use std::{collections::LinkedList, fs, io, iter::Iterator, path::PathBuf, sync::Arc};
use tokio::sync::Mutex;
use twackup::{
    archiver::Level as CompressionLevel,
    builder::{AllPackagesArchive, Preferences, Worker},
    package::Package,
    progress::Progress,
    Dpkg, GenericError, PackagesSort,
};

const DEFAULT_ARCHIVE_NAME: &str = "%host%_%date%.tar.gz";

#[derive(clap::Parser, clap::ValueEnum, Debug, Copy, Clone)]
enum CompressionType {
    Gzip,
    Xz,
    Zst,
    Bz2,
}

impl From<CompressionType> for twackup::archiver::Type {
    fn from(value: CompressionType) -> Self {
        match value {
            CompressionType::Gzip => Self::Gz,
            CompressionType::Xz => Self::Xz,
            CompressionType::Zst => Self::Zst,
            CompressionType::Bz2 => Self::Bz2,
        }
    }
}

#[derive(clap::Parser)]
#[clap(
    version,
    after_help = "
Beware, this command doesn't guarantee to copy all files to the final DEB!
Some files can be skipped because of being renamed or removed in the installation process.
If you see warnings, it means the final deb misses some contents
and may not work properly anymore.
"
)]
pub(crate) struct Build {
    #[clap(flatten)]
    global_options: GlobalOptions,

    /// By default twackup rebuilds only that packages which are not dependencies of others.
    /// This flag disables this restriction - command will rebuild all found packages.
    #[arg(short, long)]
    all: bool,

    /// Use custom destination <directory>.
    #[arg(long, short, default_value = paths::debs_target_dir(), value_parser)]
    destination_dir: PathBuf,

    /// Packs all rebuilded DEB's to single archive
    #[arg(short = 'A', long, default_value_t = false)]
    archive: bool,

    /// Name of archive if --archive is set. Supports only .tar.gz archives for now.
    #[arg(long, default_value = DEFAULT_ARCHIVE_NAME)]
    archive_name: String,

    /// Removes all DEB's after adding to archive. Makes sense only if --archive is set.
    #[arg(long, short = 'R', default_value_t = false)]
    remove_after: bool,

    /// DEB Compression level. 0 means no compression while 9 - strongest. Default is 6
    #[arg(long, short = 'l', default_value_t = 6)]
    #[arg(value_parser = clap::value_parser!(u32).range(0..=9))]
    compression_level: u32,

    /// DEB Compression type
    #[arg(long, short = 'c', default_value = "gzip")]
    compression_type: CompressionType,

    /// Package identifier or number from the list command.
    /// This argument can have multiple values separated by space ' '.
    packages: Vec<String>,
}

impl Build {
    async fn build_user_specified(&self) -> Result<()> {
        let all_packages = self
            .global_options
            .packages(false, PackagesSort::Name)
            .await?;

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

        let to_build: Vec<_> = numeric_packages.chain(alphabetic_packages).collect();

        let to_build_ids: Vec<_> = to_build.iter().map(|pkg| pkg.id.as_str()).collect();
        log::info!("Building {}...", to_build_ids.join(", "));

        self.build(to_build).await
    }

    async fn build(&self, packages: Vec<Package>) -> Result<()> {
        self.create_dir_if_needed()?;

        let all_count = packages.len() as u64;
        let progress = ProgressBar::default(all_count);

        if !am_root() {
            log::warn!("{}", GenericError::NotRunningAsRoot);
        }

        let archive = self.create_archive_if_needed().await?;

        let mut preferences =
            Preferences::new(&self.global_options.admin_dir, &self.destination_dir);
        preferences.remove_deb = self.remove_after;
        preferences.compression.level = CompressionLevel::Custom(self.compression_level);
        preferences.compression.r#type = self.compression_type.into();

        let contents = Dpkg::new(&self.global_options.admin_dir, false).info_dir_contents()?;
        let contents = Arc::new(contents);

        futures::future::join_all(packages.into_iter().map(|package| {
            let progress = progress.clone();
            let archive = archive.clone();
            let preferences = preferences.clone();
            let contents = contents.clone();

            tokio::spawn(async move {
                let builder = Worker::new(&package, progress, archive, preferences, contents);
                builder.run().await
            })
        }))
        .await;

        progress.finished_all();
        log::info!("Processed {} packages", all_count);

        Ok(())
    }

    async fn create_archive_if_needed(&self) -> Result<Option<AllPackagesArchive>> {
        if !self.archive {
            return Ok(None);
        }

        let filename = match &*self.archive_name {
            DEFAULT_ARCHIVE_NAME => format!(
                "{}_{}.tar.gz",
                gethostname().to_str().unwrap_or_default(),
                Local::now().format("%v_%T")
            ),
            _ => self.archive_name.clone(),
        };

        let filepath = self.destination_dir.join(&filename);
        let file = tokio::fs::File::create(filepath).await?;

        let archive = tokio_tar::Builder::new(file);
        Ok(Some(Arc::new(Mutex::new(archive))))
    }

    fn create_dir_if_needed(&self) -> Result<()> {
        match fs::metadata(&self.destination_dir) {
            Ok(metadata) if !metadata.is_dir() => fs::remove_file(&self.destination_dir)?,
            _ => {}
        }

        Ok(fs::create_dir_all(&self.destination_dir)?)
    }
}

#[async_trait::async_trait]
impl CliCommand for Build {
    async fn run(&self) -> Result<()> {
        if !self.packages.is_empty() {
            return self.build_user_specified().await;
        }

        let mut leaves_only = false;
        if !self.all {
            eprint!(
                "{} [Y/N] [default N]",
                style("No packages specified. Build leaves?").yellow()
            );

            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer)?;
            if buffer.trim().to_lowercase() == "y" {
                leaves_only = true;
            } else {
                eprintln!("Ok, cancelling...");
                return Ok(());
            }
        }

        let packages = self.global_options.unsorted_packages(leaves_only).await?;
        let packages = packages.into_iter().collect();
        self.build(packages).await
    }
}
