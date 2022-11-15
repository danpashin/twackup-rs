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

use super::{DataLayout, DataType, RepoGroup, RepoGroupFormat, CLASSIC_MANAGERS, MODERN_MANAGERS};
use crate::{
    commands::{CliCommand, GlobalOptions},
    error::Result,
    serializer::Format,
    Context,
};
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};
use twackup::{repository::Repository, PackagesSort, Parser};

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
    async fn run(&self, context: Context) -> Result<()> {
        log::info!("Exporting data for {:?}...", self.data);

        let data = self.construct_data(context).await?;
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
    async fn construct_data(&self, _context: Context) -> Result<DataLayout> {
        Ok(match self.data {
            DataType::Packages => DataLayout {
                packages: Some(self.get_packages().await?),
                repositories: None,
            },
            DataType::Repositories => DataLayout {
                packages: None,
                repositories: Some(self.get_repos().await?),
            },
            DataType::All => {
                let packages = self.get_packages().await?;
                let repos = self.get_repos().await?;
                DataLayout {
                    packages: Some(packages),
                    repositories: Some(repos),
                }
            }
        })
    }

    async fn get_packages(&self) -> Result<Vec<String>> {
        let packages = self
            .global_options
            .packages(false, PackagesSort::Name)
            .await?;

        Ok(packages
            .into_iter()
            .map(|(identifier, _)| identifier)
            .collect())
    }

    async fn get_repos(&self) -> Result<Vec<RepoGroup>> {
        let capacity = MODERN_MANAGERS.len() + CLASSIC_MANAGERS.len();
        let mut sources = Vec::with_capacity(capacity);

        for (name, path) in MODERN_MANAGERS {
            let parser = match Parser::new(path) {
                Ok(parser) => parser,
                Err(error) => {
                    log::warn!("[{}] {:?}", name, error);
                    continue;
                }
            };

            let repos = parser.parse::<Repository>().await.into_iter().collect();
            sources.push(RepoGroup {
                format: RepoGroupFormat::Modern,
                path: (*path).to_string(),
                executable: (*name).to_string(),
                sources: repos,
            });
        }

        for (name, path) in CLASSIC_MANAGERS {
            let file = match File::open(path) {
                Ok(file) => file,
                Err(error) => {
                    log::warn!("[{}] {:?}", name, error);
                    continue;
                }
            };

            let mut repos = Vec::new();
            for line in BufReader::new(file).lines().flatten() {
                match Repository::from_one_line(line.as_str()) {
                    Ok(repo) => repos.push(repo),
                    Err(error) => log::warn!("[{}] {:?}", name, error),
                }
            }

            sources.push(RepoGroup {
                format: RepoGroupFormat::Classic,
                path: (*path).to_string(),
                executable: (*name).to_string(),
                sources: repos,
            });
        }

        Ok(sources)
    }
}
