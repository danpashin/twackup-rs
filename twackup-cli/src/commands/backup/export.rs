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

use super::{DataType, ExportData, RepoGroup, RepoGroupFormat};
use crate::{
    commands::{CliCommand, GlobalOptions},
    error::Result,
    serializer::Format,
};
use std::{
    fs::File,
    io::{self},
    path::PathBuf,
};

#[derive(clap::Parser)]
pub(crate) struct Export {
    #[clap(flatten)]
    global_options: GlobalOptions,

    /// Use another output format
    /// (e.g. for using output with third-party parser like jq)
    #[arg(short, long, value_enum, default_value = "json")]
    format: Format,

    /// Data to export
    /// (e.g. if you want to export only packages)
    #[arg(short, long, value_enum, default_value = "all")]
    data: DataType,

    /// Output file, stdout if not present
    #[arg(short, long)]
    output: Option<PathBuf>,
}

#[async_trait::async_trait]
impl CliCommand for Export {
    async fn run(&self) -> Result<()> {
        log::info!("Exporting data for {:?}...", self.data);

        let data = self.construct_data().await?;
        if let Some(path) = &self.output {
            let file = File::create(path)?;
            self.format.ser_to_writer(file, &data)?;
        } else {
            self.format.ser_to_writer(io::stdout(), &data)?;
            println!();
        }

        log::info!("Successfully exported {:?} data!", self.data);

        Ok(())
    }
}

impl Export {
    async fn construct_data(&self) -> Result<ExportData> {
        let (packages, repositories) = match self.data {
            DataType::Packages => (Some(self.get_packages().await?), None),
            DataType::Repositories => (None, Some(self.get_repos().await?)),
            DataType::All => (
                Some(self.get_packages().await?),
                Some(self.get_repos().await?),
            ),
        };

        Ok(ExportData {
            packages,
            repositories,
        })
    }

    async fn get_packages(&self) -> Result<Vec<String>> {
        let packages = self.global_options.unsorted_packages(false).await?;
        Ok(packages.into_iter().map(|pkg| pkg.id).collect())
    }

    async fn get_repos(&self) -> Result<Vec<RepoGroup>> {
        let managers = super::package_manager::PackageManager::supported();

        let capacity = managers.len();
        let mut sources = Vec::with_capacity(capacity);

        for manager in managers {
            let executable = manager.name().to_string();
            let repos = match manager.repositories().await {
                Ok(val) => val,
                Err(error) => {
                    log::warn!("[{}] {:?}", executable, error);
                    continue;
                }
            };

            let format = if manager.is_modern() {
                RepoGroupFormat::Modern
            } else {
                RepoGroupFormat::Classic
            };

            sources.push(RepoGroup {
                format,
                path: manager.sources_path(),
                executable,
                sources: repos,
            });
        }

        Ok(sources)
    }
}
