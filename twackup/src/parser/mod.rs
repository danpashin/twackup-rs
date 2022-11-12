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

mod iterators;

use crate::parser::iterators::UnOwnedLine;
use memmap2::Mmap;
use std::{
    collections::{HashMap, LinkedList},
    fs::File,
    io::{self},
    marker::Send,
    path::Path,
    ptr::slice_from_raw_parts,
};

/// Common trait for any struct that can be parsed in key-value mode
pub trait Parsable: Send + Sized {
    type Error: Send;

    /// Should process lines and return result
    ///
    /// # Errors
    /// If return error, it will be logged as warning
    fn new(key_values: HashMap<String, String>) -> Result<Self, Self::Error>;
}

pub struct Parser {
    mmap: Mmap,
}

struct ChunkWorker {
    ptr: usize,
    length: usize,
}

impl Parser {
    /// Prepares environment and creates parser instance
    ///
    /// # Errors
    /// Will return error when user has no permissions to read
    /// or file is empty
    pub fn new<P: AsRef<Path>>(file_path: P) -> io::Result<Self> {
        let file = File::open(file_path)?;
        let mmap = unsafe { Mmap::map(&file) }?;

        Ok(Self { mmap })
    }

    /// This method will parse file with key-value syntax on separate lines.
    pub async fn parse<P: Parsable + 'static>(&self) -> LinkedList<P> {
        let mut workers = LinkedList::new();

        for chunk in UnOwnedLine::double_line(&self.mmap[..]) {
            let worker = ChunkWorker::new(chunk.as_ptr() as usize, chunk.len());
            workers.push_back(tokio::spawn(async { worker.run::<P>() }));
        }

        let mut models = LinkedList::new();
        for worker in workers {
            match worker.await {
                Ok(Ok(worker_models)) => models.push_back(worker_models),
                Ok(Err(_)) => {}
                Err(error) => log::warn!("Worker join error: {:?}", error),
            }
        }

        models
    }
}

impl ChunkWorker {
    /// Prepares environment and creates parser instance
    fn new(ptr: usize, length: usize) -> Self {
        Self { ptr, length }
    }

    /// Parses chunk to model
    fn run<P: Parsable>(self) -> Result<P, P::Error> {
        // Fucking hacks. This is the only way to pass slice without ARC
        let chunk = unsafe { &*slice_from_raw_parts(self.ptr as *const u8, self.length) };

        let fields = self.parse_chunk(chunk);
        P::new(fields)
    }

    /// Converts raw chunk bytes to list of lines with multi-line syntax support
    fn parse_chunk(&self, chunk: &[u8]) -> HashMap<String, String> {
        let mut fields: LinkedList<Vec<_>> = LinkedList::new();

        let line_iter = UnOwnedLine::single_line(chunk).flat_map(std::str::from_utf8);

        // Now we'll process each line of chunk
        for line in line_iter {
            // If line is empty (but it shouldn't) - skip
            if line.is_empty() {
                continue;
            }

            // Keys can have multi-line syntax starting with single space
            // So we'll process them and concat with previous line in list
            if line.starts_with(' ') {
                let mut prev_lines = fields.pop_back().unwrap_or_default();

                prev_lines.push(line);
                fields.push_back(prev_lines);
            } else {
                fields.push_back(vec![line]);
            }
        }

        fields
            .iter()
            .filter_map(|field_lines| {
                // Find delimiter in first line
                let (key, first_val) = field_lines.first()?.split_once(':')?;
                let key = key.to_string();

                // Count total length to effectively allocate space
                let total_len = field_lines
                    .iter()
                    .skip(1)
                    .fold(0, |sum, line| sum + line.len() + 1);
                let total_len = total_len + first_val.len();

                // Create copy for the first line
                let mut value = String::with_capacity(total_len);
                value.push_str(first_val.trim_start());

                // And for other lines
                let value = field_lines.iter().skip(1).enumerate().fold(
                    value,
                    |mut value, (index, line)| {
                        value.push('\n');
                        // If this is the last line - trim it from the end
                        if index == field_lines.len() - 1 {
                            value.push_str(line.trim_end());
                        } else {
                            value.push_str(line);
                        }
                        value
                    },
                );

                Some((key, value))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::{
        error::Result,
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
    async fn valid_database() -> Result<()> {
        let database = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/database/valid");
        let parser = Parser::new(database)?;
        let packages = parser.parse::<Package>().await;
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
        let packages = parser.parse::<Package>().await;
        assert_ne!(packages.len(), 3);

        Ok(())
    }

    #[tokio::test]
    async fn multiline() -> Result<()> {
        let database = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/database/multiline");
        let parser = Parser::new(database)?;

        let packages = parser.parse::<Package>().await;
        let packages: HashMap<String, Package> = packages
            .into_iter()
            .map(|pkg| (pkg.id.clone(), pkg))
            .collect();

        let package = packages.get("valid-package-1").unwrap();
        let description = package.get(Field::Description)?;
        assert_eq!(description, "First Line\n Second Line\n  Third Line");

        assert!(packages.get("invalid-package-1").is_none());

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

        let repositories = parser.parse::<Repository>().await;
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
