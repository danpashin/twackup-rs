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

use memmap::Mmap;
use std::{
    collections::{HashMap, LinkedList},
    fs::File,
    io::{self, BufRead},
    marker::Send,
    ops::Range,
    path::Path,
    sync::Arc,
};

pub trait Parsable: Send + Sized {
    type Error: Send;

    fn new(key_values: HashMap<String, String>) -> Result<Self, Self::Error>;
}

pub struct Parser {
    file: File,
    mmap: Arc<Mmap>,
}

struct ChunkWorker {
    file: Arc<Mmap>,
    range: Range<usize>,
}

impl Parser {
    /// Prepares environment and creates parser instance
    ///
    /// Will return error when user has no permissions to read
    /// or file is empty
    pub fn new<P: AsRef<Path>>(file_path: P) -> io::Result<Self> {
        let file = File::open(file_path.as_ref())?;
        let mmap = Arc::new(unsafe { Mmap::map(&file) }?);
        Ok(Self { file, mmap })
    }

    /// This method will parse file with key-value syntax on separate lines.
    pub async fn parse<P: Parsable + 'static>(&self) -> io::Result<LinkedList<P>> {
        let mut workers = LinkedList::new();

        let mut last_nl_pos = 0;
        let file_len = self.file.metadata()?.len() as usize;

        for pos in 0..file_len - 1 {
            if self.mmap[pos] == b'\n' && self.mmap[pos + 1] == b'\n' {
                let worker = ChunkWorker::new(self.mmap.clone(), last_nl_pos..pos);
                workers.push_back(tokio::spawn(async { worker.run() }));
                last_nl_pos = pos;
            }
        }

        let mut models = LinkedList::new();
        for worker in workers {
            if let Ok(Ok(worker_models)) = worker.await {
                models.push_back(worker_models);
            }
        }

        Ok(models)
    }
}

impl ChunkWorker {
    /// Prepares environment and creates parser instance
    fn new(file: Arc<Mmap>, range: Range<usize>) -> Self {
        Self { file, range }
    }

    /// Parses chunk to model
    fn run<P: Parsable>(self) -> Result<P, P::Error> {
        let chunk = &self.file[self.range.start..self.range.end];
        let fields = self.parse_chunk(chunk);
        P::new(fields)
    }

    /// Converts raw chunk bytes to list of lines with multi-line syntax support
    fn parse_chunk(&self, chunk: &[u8]) -> HashMap<String, String> {
        let mut fields: LinkedList<String> = LinkedList::new();

        // Now we'll process each line of chunk
        for line in chunk.lines().flatten() {
            // If line is empty (but it shouldn't) - skip
            if line.is_empty() {
                continue;
            }

            // Keys can have multi-line syntax starting with single space
            // So we'll process them and concat with previous line in list
            if line.starts_with(' ') {
                let prev_line = fields.pop_back().unwrap_or_default();
                fields.push_back(format!("{}\n{}", prev_line, line));
            } else {
                fields.push_back(line);
            }
        }

        fields
            .into_iter()
            .filter_map(|field| {
                // Dpkg uses key-value syntax, so firstly, we'll find delimiter
                // Every line without delimiter is invalid and will be skipped
                field.split_once(':').map(|(key, value)| {
                    // Then we'll split line into two ones and trim the result
                    // to remove linebreaks and spaces
                    (key.to_string(), value.trim().to_string())
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::{
        error::Result,
        package::{FieldName, Package},
        repository::Repository,
    };
    use std::{
        collections::HashMap,
        env,
        fs::{self, File},
        io::{self, BufRead, BufReader, Write},
        os::unix::fs::PermissionsExt,
    };

    #[tokio::test]
    async fn valid_database() -> Result<()> {
        let database = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/database/valid");
        let parser = Parser::new(database)?;
        let packages = parser.parse::<Package>().await?;
        assert_eq!(packages.len(), 3);

        Ok(())
    }

    #[tokio::test]
    async fn partially_valid_database() -> Result<()> {
        let database = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/database/partially_valid"
        );
        let parser = Parser::new(database)?;
        let packages = parser.parse::<Package>().await?;
        assert_ne!(packages.len(), 3);

        Ok(())
    }

    #[tokio::test]
    async fn multiline() -> Result<()> {
        let database = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/database/multiline");
        let parser = Parser::new(database)?;

        let packages = parser.parse::<Package>().await?;
        let packages: HashMap<String, Package> = packages
            .into_iter()
            .map(|pkg| (pkg.id.clone(), pkg))
            .collect();

        let package = packages.get("valid-package-1").unwrap();
        let description = package.get(FieldName::Description)?;
        assert_eq!(description, "First Line\n Second Line\n  Third Line");

        let package = packages.get("valid-package-2").unwrap();
        let description = package.get(FieldName::Description)?;
        assert_eq!(description, "First Line");

        Ok(())
    }

    #[test]
    fn no_permissions_database() -> Result<()> {
        let database = env::temp_dir().join("twackup-no-permissions");
        let mut file = File::create(&database)?;
        file.write("This contents will never be read".as_bytes())?;
        fs::set_permissions(&database, fs::Permissions::from_mode(0o333))?;

        let parser = Parser::new(database.as_path());
        assert_eq!(parser.is_err(), true);
        assert_eq!(
            io::Error::last_os_error().kind(),
            io::ErrorKind::PermissionDenied
        );

        fs::remove_file(&database)?;

        Ok(())
    }

    #[tokio::test]
    async fn modern_repository() -> Result<()> {
        let database = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/sources_db/modern");

        let parser = Parser::new(database)?;

        let repositories = parser.parse::<Repository>().await?;
        let repositories: HashMap<String, Repository> = repositories
            .into_iter()
            .map(|repo| (repo.url.clone(), repo))
            .collect();

        assert_eq!(repositories.len(), 3);

        let repo = repositories.get("https://apt1.example.com/").unwrap();
        assert_eq!(repo.components.as_slice(), &["main", "orig"]);

        Ok(())
    }

    #[test]
    fn classic_repository() -> Result<()> {
        let database = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/sources_db/classic");
        let reader = BufReader::new(File::open(database)?);

        let lines = reader.lines().flatten();
        let repositories: HashMap<String, Repository> = lines
            .filter_map(|line| {
                Repository::from_one_line(line.as_str())
                    .map(|repo| (repo.url.clone(), repo))
                    .ok()
            })
            .collect();

        assert_eq!(repositories.len(), 3);

        let repo = repositories.get("https://apt1.example.com/").unwrap();
        assert_eq!(repo.distribution.as_str(), "stable");
        assert_eq!(repo.components.as_slice(), &["main", "orig"]);

        Ok(())
    }
}
