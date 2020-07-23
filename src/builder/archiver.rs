extern crate tar;
use tar::Builder;
use std::{
    io,
    path::{Path, PathBuf},
    fs::File
};

pub struct Archiver {
    output_file: PathBuf,
    builder: Builder<File>
}

impl Archiver {
    pub fn new(output_file: &Path) -> io::Result<Self> {
        return Ok(Self {
            output_file: output_file.to_path_buf(),
            builder: Builder::new(File::create(output_file)?)
        });
    }

    pub fn append_path(&mut self, path: &Path) -> io::Result<()> {
        return self.builder.append_path(path);
    }

    pub fn append_path_with_name(&mut self, path: &Path, name: &Path) -> io::Result<()> {
        return self.builder.append_path_with_name(path, name);
    }

    pub fn get_mut(&mut self) -> &mut Builder<File> {
        return &mut self.builder;
    }

    pub fn finish(&mut self) -> io::Result<()> {
        return self.builder.finish();
    }
}
