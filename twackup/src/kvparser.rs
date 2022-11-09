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

pub trait Parsable {
    type Output;
    fn new(key_values: HashMap<String, String>) -> Option<Self::Output>;
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
    pub async fn parse<P: Parsable<Output = P> + 'static + Send>(&self) -> LinkedList<P> {
        let mut workers = LinkedList::new();

        let mut last_nl_pos = 0;
        let file_len = self.get_file_len();
        for pos in 0..file_len - 1 {
            if self.mmap[pos] ^ b'\n' == 0 && self.mmap[pos + 1] ^ b'\n' == 0 {
                let worker = ChunkWorker::new(self.mmap.clone(), last_nl_pos..pos);
                workers.push_back(tokio::spawn(worker.run()));
                last_nl_pos = pos;
            }
        }

        let mut models = LinkedList::new();
        for worker in workers {
            if let Ok(Some(worker_models)) = worker.await {
                models.push_back(worker_models);
            }
        }
        models
    }

    fn get_file_len(&self) -> usize {
        match self.file.metadata() {
            Ok(metadata) => metadata.len() as usize,
            Err(_) => 0,
        }
    }
}

impl ChunkWorker {
    /// Prepares environment and creates parser instance
    fn new(file: Arc<Mmap>, range: Range<usize>) -> Self {
        Self { file, range }
    }

    /// Parses chunk to model
    async fn run<P: Parsable<Output = P>>(self) -> Option<P> {
        let chunk = &self.file[self.range.start..self.range.end];
        let fields = self.parse_chunk(chunk);
        P::new(self.parse_fields(fields))
    }

    /// Converts raw chunk bytes to list of lines with multi-line syntax support
    fn parse_chunk(&self, chunk: &[u8]) -> LinkedList<String> {
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
                let prev_line = fields.pop_back().unwrap_or_else(|| "".into());
                fields.push_back(format!("{}\n{}", prev_line, line));
            } else {
                fields.push_back(line);
            }
        }

        fields
    }

    /// Parses lines to keys and values
    fn parse_fields(&self, fields: LinkedList<String>) -> HashMap<String, String> {
        let mut fields_map = HashMap::new();

        for field in fields {
            // Dpkg uses key-value syntax, so firstly, we'll find delimiter
            // Every line without delimiter is invalid and will be skipped
            if let Some(delim_pos) = field.find(':') {
                // Then we'll split line into two ones and trim the result
                // to remove linebreaks and spaces
                let (key, value) = field.split_at(delim_pos);
                fields_map.insert(
                    key.trim().to_string(),
                    value.trim_start_matches(':').trim().to_string(),
                );
            }
        }

        fields_map
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::{
        package::{Field, Package},
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
    async fn valid_database() {
        let database = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/database/valid");
        let parser = Parser::new(database).unwrap();
        let packages = parser.parse::<Package>().await;
        assert_eq!(packages.len(), 3);
    }

    #[tokio::test]
    async fn partially_valid_database() {
        let database = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/database/partially_valid"
        );
        let parser = Parser::new(database).unwrap();
        let packages = parser.parse::<Package>().await;
        assert_ne!(packages.len(), 3);
    }

    #[tokio::test]
    async fn multiline() {
        let database = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/database/multiline");
        let parser = Parser::new(database).unwrap();

        let packages = parser.parse::<Package>().await;
        let packages: HashMap<String, Package> = packages
            .into_iter()
            .map(|pkg| (pkg.id.clone(), pkg))
            .collect();

        let package = packages.get("valid-package-1").unwrap();
        let description = package.get_field(Field::Description).unwrap();
        assert_eq!(description, "First Line\n Second Line\n  Third Line");

        let package = packages.get("valid-package-2").unwrap();
        let description = package.get_field(Field::Description).unwrap();
        assert_eq!(description, "First Line");
    }

    #[test]
    fn no_permissions_database() {
        let database = env::temp_dir().join("twackup-no-permissions");
        let mut file = File::create(&database).unwrap();
        file.write("This contents will never be read".as_bytes())
            .unwrap();
        fs::set_permissions(&database, fs::Permissions::from_mode(0o333)).unwrap();

        let parser = Parser::new(database.as_path());
        assert_eq!(parser.is_err(), true);
        assert_eq!(
            io::Error::last_os_error().kind(),
            io::ErrorKind::PermissionDenied
        );

        fs::remove_file(&database).unwrap();
    }

    #[tokio::test]
    async fn modern_repository() {
        let database = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/sources_db/modern");

        let parser = Parser::new(database).unwrap();

        let repositories = parser.parse::<Repository>().await;
        let repositories: HashMap<String, Repository> = repositories
            .into_iter()
            .map(|repo| (repo.url.clone(), repo))
            .collect();

        assert_eq!(repositories.len(), 3);

        let repo = repositories.get("https://apt1.example.com/").unwrap();
        assert_eq!(repo.components.as_slice(), &["main", "orig"]);
    }

    #[test]
    fn classic_repository() {
        let database = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/sources_db/classic");
        let reader = BufReader::new(File::open(database).unwrap());

        let repositories: HashMap<String, Repository> = reader
            .lines()
            .map(|line| {
                let line = line.expect("Can't unwrap line");
                eprintln!("{}", line);
                let repo = Repository::from_one_line(line.as_str()).expect("Parsing repo failed");
                (repo.url.clone(), repo)
            })
            .collect();

        assert_eq!(repositories.len(), 3);

        let repo = repositories.get("https://apt1.example.com/").unwrap();
        assert_eq!(repo.distribution.as_str(), "stable");
        assert_eq!(repo.components.as_slice(), &["main", "orig"]);
    }
}
