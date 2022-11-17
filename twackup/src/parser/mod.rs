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
    /// Error which will be used to contain model errors
    type Error: Send;

    /// Should process lines and return result
    ///
    /// # Errors
    /// If return error, it will be logged as warning
    fn new(key_values: HashMap<String, String>) -> Result<Self, Self::Error>;
}

/// Parser is a Twackup module that parses file line by line
/// in a simple key-value way to any Rust struct
/// that implements [Parsable] trait.
///
/// Moreover, it supports multi-line syntax, so file can look like this
/// ```txt
/// Line1: Value1
/// Line2: Value2
///   Continuation for line 2
/// ```
/// And parser will parse this without errors!
///
/// # Example usage
///
/// ```no_run
/// use twackup::{Parser, Result, package::Package};
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let dpkg_database = "/var/lib/dpkg/status";
///     let parser = Parser::new(dpkg_database)?;
///
///     let packages = parser.parse::<Package>().await;
///     for package in packages {
///         println!("Package {}", package.id);
///     }
///
///     Ok(())
/// }
/// ```
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
    #[inline]
    pub fn new<P: AsRef<Path>>(file_path: P) -> io::Result<Self> {
        let file = File::open(file_path)?;
        let mmap = unsafe { Mmap::map(&file) }?;

        Ok(Self { mmap })
    }

    /// This method will parse file with key-value syntax on separate lines.
    pub async fn parse<P: Parsable + 'static>(&self) -> LinkedList<P> {
        let mut workers = LinkedList::new();

        for chunk in UnOwnedLine::double_line(&self.mmap) {
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
    #[inline]
    const fn new(ptr: usize, length: usize) -> Self {
        Self { ptr, length }
    }

    /// Parses chunk to model
    #[inline]
    fn run<P: Parsable>(self) -> Result<P, P::Error> {
        let fields = self.parse_chunk();
        P::new(fields)
    }

    /// Converts raw chunk bytes to list of lines with multi-line syntax support
    fn parse_chunk(&self) -> HashMap<String, String> {
        // Fucking hacks. This is the only way to pass slice without ARC
        let chunk = unsafe { &*slice_from_raw_parts(self.ptr as *const u8, self.length) };

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
                let key = key.to_owned();

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
