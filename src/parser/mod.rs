// pub mod crate::package;
use std::{
    fs::File, io::BufRead,
    str,
    collections::{LinkedList, HashMap},
};
use memmap::Mmap;
use crate::{cli_error::CliError, package::*};

extern crate num_cpus;
extern crate threadpool;

const NEWLINE_CHAR: u8 = 0xA;

pub struct Parser {
    file_path: String,
    file: File,
    thread_pool: threadpool::ThreadPool
}

struct ChunkParser {
    file_path: String,
    start: usize,
    end: usize
}

impl Parser {
    /// Prepares environment and creates parser instance
    pub fn new(file_path: &str) -> Result<Parser, CliError> {
        return Ok(Parser {
            file_path: file_path.to_string(),
            // If file is not found or user has no rigths, this method will throw an error
            file: File::open(file_path)?,
            // Thread pool will grab all processor cores
            thread_pool: threadpool::ThreadPool::new(num_cpus::get())
        });
    }

    /// This method will parse file with key-value syntax on separate lines
    /// and call handler for each found block
    ///
    /// ### File should have following syntax. Every wrong line will be skipped.
    ///
    /// ```
    /// Package: com.example.my.package
    /// Name: My Package
    ///
    /// Package: com.example.my.other.package
    /// Name: My Other Package
    /// ```
    pub fn parse<F>(&self, handler: F)
    where
        F: Fn(Package) + Send + Sync + 'static,
    {
        let mut last_is_nl = true;
        let mut last_nl_pos = 0;
        let mut cur_position = 0;

        let safe_handler = std::sync::Arc::new(handler);

        // Load file in memory with mmap kernel feature
        let fmmap = unsafe { Mmap::map(&self.file).unwrap()  };
        // And iterate for all bytes in file
        for byte in fmmap.iter() {
            cur_position += 1;
            // When double new line is detected
            let nl = byte ^ NEWLINE_CHAR == 0;
            if nl && last_is_nl {
                // Create ARC pointer for handler, get package (chunk) start/end positions
                let th_handler = std::sync::Arc::clone(&safe_handler);
                let parser = ChunkParser::new(
                    self.file_path.clone(),
                    last_nl_pos.clone(),
                    cur_position.clone()
                );

                // And execute parser in another thread with calling atomic handler
                self.thread_pool.execute(move || {
                    if let Some(pkg) = parser.parse() {
                        th_handler(pkg);
                    }
                });
                last_nl_pos = cur_position.clone();
            }
            last_is_nl = nl;
        }

        self.thread_pool.join();
    }
}

impl ChunkParser {
    /// Prepares environment and creates parser instance
    fn new(file_path: String, start: usize, end: usize) -> ChunkParser {
        return ChunkParser { file_path, start, end };
    }

    /// Parses chunk to package model
    fn parse(&self) -> Option<Package> {
        // Open file and load it in memory with mmap kernel feature
        let file = File::open(&self.file_path).unwrap();
        let mmap = unsafe { Mmap::map(&file).unwrap()  };
        // Load package (chunk) in buffer. This usually allocates 1 MB of memory
        let chunk = &mmap[self.start..self.end];

        // First, we'll get all lines (with these which can be multi-line
        let fields = self.parse_chunk(chunk);

        // Now process each line individually
        let fields_map = self.parse_fields(fields);

        return Some(Package::new(&fields_map)?);
    }

    /// Converts raw chunk bytes to list of lines with multi-line syntax support
    fn parse_chunk(&self, chunk: &[u8]) -> LinkedList<String> {
        let mut fields = LinkedList::new();

        // Now we'll process each line of chunk
        for line in chunk.lines() {
            let unwrapped_line = line.unwrap();
            // If line is empty (but it shouldn't) - skip
            if unwrapped_line.is_empty() {
                continue;
            }

            // Keys can have multi-line syntax starting with two or four spaces
            // So we'll process them and concat with previous line in list
            if unwrapped_line.starts_with(" ") && !fields.is_empty() {
                let prev_line = fields.pop_back().unwrap();
                fields.push_back(format!("{}{}", prev_line, unwrapped_line).to_string());
            } else {
                fields.push_back(unwrapped_line);
            }
        }

        return fields;
    }

    /// Parses lines to keys and values
    fn parse_fields(&self, fields: LinkedList<String>) -> HashMap<String, String> {
        let mut fields_map = HashMap::new();

        for field in fields {
            // Dpkg uses key-value syntax, so firsly, we'll find delimeter
            // Every line without delimiter is invalid and will be skipped
            let delim_pos = field.find(':');
            if delim_pos.is_some() {
                // Then we'll split line into two ones and trim the result
                // to remove linebreaks and spaces
                let (key, value) = field.split_at(delim_pos.unwrap());
                fields_map.insert(
                    key.trim().to_string(),
                    value.trim_start_matches(':').trim().to_string()
                );
            }
        }

        return fields_map;
    }
}
