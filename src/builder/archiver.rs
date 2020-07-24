extern crate tar;
use tar::Builder;
use flate2::{Compression, write::GzEncoder};
use std::{
    io::{self, Read, Write},
    path::{Path, PathBuf},
    fs::File,
    time::SystemTime,
};

/// Just a wrapper for tar and flate2 libs
pub struct TarArchive {
    output_file: PathBuf,
    builder: Builder<File>
}

impl TarArchive {
    pub fn new(output_file: &Path) -> io::Result<Self> {
        return Ok(Self {
            output_file: output_file.to_path_buf(),
            builder: Builder::new(File::create(output_file)?)
        });
    }

    /// Adds a file on the local filesystem to this archive.
    pub fn append_path(&mut self, path: &Path) -> io::Result<()> {
        return self.builder.append_path(path);
    }

    /// Adds a file on the local filesystem to this archive under another name.
    pub fn append_path_with_name(&mut self, path: &Path, name: &Path) -> io::Result<()> {
        return self.builder.append_path_with_name(path, name);
    }

    /// Finish writing this archive, emitting the termination sections.
    pub fn finish_appending(&mut self) -> io::Result<()> {
        return self.builder.finish();
    }

    /// Appends non-existing on the filesystem file to archive
    pub fn append_new_file(&mut self, path: &Path, contents: &[u8]) -> io::Result<()> {
        let cur_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH).unwrap()
            .as_secs();

        let mut header = tar::Header::new_old();
        header.set_mode(0o644); // o=rw,g=r
        header.set_uid(0);
        header.set_gid(0);
        header.set_size(contents.len() as u64);
        header.set_mtime(cur_time); // modify time
        header.set_cksum();

        return self.builder.append_data(&mut header, path, contents);
    }

    /// Compresses output file with gzip
    pub fn compress_gzip(&mut self, path: &Path, compression: u32)-> io::Result<()> {
        debug_assert!(compression <= 9, "Compression level must be below or equal to 9");

        let mut uncompressed = File::open(&self.output_file)?;
        let compressed = File::create(path)?;
        // Allocate 1 MB of memory in heap
        let mut buffer = vec![0; 1024];

        let mut encoder = GzEncoder::new(compressed, Compression::new(compression));

        while uncompressed.read_exact(&mut buffer).is_ok() {
            encoder.write(buffer.as_slice()).unwrap();
        }

        encoder.finish()?;

        return Ok(());
    }
}
