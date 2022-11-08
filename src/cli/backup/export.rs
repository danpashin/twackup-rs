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

use super::{super::*, *};
use crate::kvparser::Parser;
use std::{
    collections::LinkedList,
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

#[derive(clap::Parser)]
pub struct Export {
    /// Use custom dpkg <directory>.
    /// This option is used for detecting installed packages
    #[clap(long, default_value = ADMIN_DIR, value_parser)]
    admindir: PathBuf,

    /// Use another output format
    /// (e.g. for using output with third-party parser like jq)
    #[clap(short, long, value_enum, default_value = "json")]
    format: DataFormat,

    /// Data to export
    /// (e.g. if you want to export only packages)
    #[clap(short, long, value_enum, default_value = "all")]
    data: DataType,

    /// Output file, stdout if not present
    #[clap(short, long)]
    output: Option<PathBuf>,
}

#[async_trait::async_trait]
impl CliCommand for Export {
    async fn run(&self) {
        eprintln!("Exporting data for {:?}...", self.data);
        let data = match self.data {
            DataType::Packages => DataLayout {
                packages: Some(self.get_packages().await),
                repositories: None,
            },
            DataType::Repositories => DataLayout {
                packages: None,
                repositories: Some(self.get_repos().await),
            },
            DataType::All => {
                let packages = self.get_packages().await;
                let repos = self.get_repos().await;
                DataLayout {
                    packages: Some(packages),
                    repositories: Some(repos),
                }
            }
        };

        let format = serde_any::guess_format_from_extension(self.format.as_str())
            .expect("Unsupported format");

        if let Some(path) = &self.output {
            let file = File::create(path).expect("Can't open fd for writing");
            serde_any::to_writer(file, &data, format).unwrap();
        } else {
            serde_any::to_writer(io::stdout(), &data, format).unwrap();
            println!();
        }

        eprintln!("Successfully exported {:?} data!", self.data);
    }
}

impl Export {
    async fn get_packages(&self) -> LinkedList<String> {
        utils::get_packages(&self.admindir, true)
            .await
            .into_iter()
            .map(|pkg| pkg.id)
            .collect()
    }

    async fn get_repos(&self) -> LinkedList<RepoGroup> {
        let mut sources = LinkedList::new();

        for (name, path) in MODERN_MANAGERS {
            if let Ok(parser) = Parser::new(path) {
                let repos = parser.parse::<Repository>().await.into_iter().collect();
                sources.push_back(RepoGroup {
                    format: RepoGroupFormat::Modern,
                    path: path.to_string(),
                    executable: name.to_string(),
                    sources: repos,
                });
            }
        }

        for (name, path) in CLASSIC_MANAGERS {
            if let Ok(file) = File::open(path) {
                let mut repos = LinkedList::new();
                for line in BufReader::new(file).lines().flatten() {
                    if let Some(repo) = Repository::from_one_line(line.as_str()) {
                        repos.push_back(repo);
                    }
                }
                sources.push_back(RepoGroup {
                    format: RepoGroupFormat::Classic,
                    path: path.to_string(),
                    executable: name.to_string(),
                    sources: repos,
                });
            }
        }

        sources
    }
}
