use std::{
    fs::File, io::{self, BufRead},
    path::{Path, PathBuf},
    collections::{LinkedList, HashMap},
    thread,
    marker::Send,
};
use memmap::Mmap;
use deque::{Stealer, Stolen};

pub trait Parsable {
    type Output;
    fn new(key_values: HashMap<String, String>) -> Option<Self::Output>;
}

pub struct Parser {
    file_path: PathBuf,
    file: Mmap,
}

enum ChunkWorkerState {
    Process(usize, usize),
    Quit,
}

struct ChunkWorker {
    file: Mmap,
    stealer: Stealer<ChunkWorkerState>,
}

impl Parser {
    /// Prepares environment and creates parser instance
    ///
    /// Will return error when user has no permissions to read
    /// or file is empty
    pub fn new<P: AsRef<Path>>(file_path: P) -> io::Result<Self> {
        let file = unsafe { Mmap::map(&File::open(file_path.as_ref())?) }?;
        let file_path = file_path.as_ref().to_path_buf();
        Ok(Self { file_path, file })
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
    pub fn parse<P: Parsable<Output = P> + 'static + Send>(&self) -> Vec<P> {
        let mut workers = Vec::new();
        let (workq, stealer) = deque::new();
        for _ in 0..num_cpus::get() {
            let worker = ChunkWorker::new(&self.file_path, stealer.clone());
            workers.push(thread::spawn(move || worker.run()));
        }

        let file_len = self.file_path.metadata().expect("Can't get file").len() as usize;
        let mut last_nl_pos = 0;
        for pos in 0..file_len - 1 {
            if self.file[pos] ^ b'\n' == 0 && self.file[pos + 1] ^ b'\n' == 0 {
                workq.push(ChunkWorkerState::Process(last_nl_pos, pos));
                last_nl_pos = pos;
            }
        }

        for _ in 0..workers.len() {
            workq.push(ChunkWorkerState::Quit);
        }

        let mut models = Vec::with_capacity(workers.len());
        for worker in workers {
            models.extend(worker.join().expect("Can't join thread"));
        }

        return models;
    }
}

impl ChunkWorker {
    /// Prepares environment and creates parser instance
    fn new<P: AsRef<Path>>(file_path: P, stealer: Stealer<ChunkWorkerState>) -> Self {
        let file = unsafe { Mmap::map(&File::open(file_path).unwrap()).unwrap() };
        Self { file, stealer }
    }

    /// Parses chunk to model
    fn run<P: Parsable<Output = P>>(&self) -> Vec<P> {
        let mut models = Vec::new();
        loop {
            match self.stealer.steal() {
                Stolen::Empty | Stolen::Abort => continue,
                Stolen::Data(ChunkWorkerState::Quit) => break,
                Stolen::Data(ChunkWorkerState::Process(start, end)) => {
                    let fields = self.parse_chunk(&self.file[start..end]);
                    if let Some(model) = P::new(self.parse_fields(fields)) {
                        models.push(model);
                    }
                }
            }
        }

        return models;
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

            // Keys can have multi-line syntax starting with single space
            // So we'll process them and concat with previous line in list
            if unwrapped_line.starts_with(" ") && !fields.is_empty() {
                let prev_line = fields.pop_back().unwrap();
                fields.push_back(format!("{}\n{}", prev_line, unwrapped_line));
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
            // Dpkg uses key-value syntax, so firstly, we'll find delimiter
            // Every line without delimiter is invalid and will be skipped
            if let Some(delim_pos) = field.find(':') {
                // Then we'll split line into two ones and trim the result
                // to remove linebreaks and spaces
                let (key, value) = field.split_at(delim_pos);
                fields_map.insert(
                    key.trim().to_string(),
                    value.trim_start_matches(':').trim().to_string()
                );
            }
        }

        return fields_map;
    }
}
