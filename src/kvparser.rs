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
