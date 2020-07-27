use std::{
    path::PathBuf,
    io::{self, Write}, fs::File,
    borrow::BorrowMut
};
use super::archiver::*;

pub struct Deb {
    temp_dir: PathBuf,
    output: PathBuf,
    control: TarArchive,
    data: TarArchive,
    control_path: PathBuf,
    data_path: PathBuf
}

impl Deb {
    pub fn new(temp_dir: &PathBuf, output: &PathBuf) -> io::Result<Self> {
        let control_path  = temp_dir.join("control.tar");
        let data_path = temp_dir.join("data.tar");

        Ok(Self {
            temp_dir: temp_dir.clone(),
            output: output.clone(),
            control: TarArchive::new(control_path.as_path())?,
            data: TarArchive::new(data_path.as_path())?,
            control_path, data_path
        })
    }

    pub fn data_mut_ref(&mut self) -> &mut TarArchive { self.data.borrow_mut() }

    pub fn control_mut_ref(&mut self) -> &mut TarArchive { self.control.borrow_mut() }

    pub fn package(&mut self) -> io::Result<()> {
        self.control.finish_appending()?;
        self.data.finish_appending()?;

        let control_compressed = self.temp_dir.join("control.tar.gz");
        compress_gzip(&self.control_path, &control_compressed, 6)?;

        let data_compressed = self.temp_dir.join("data.tar.gz");
        compress_gzip(&self.data_path, &data_compressed, 6)?;

        // Now combine all files together
        let mut builder = ar::Builder::new(File::create(&self.output)?);

        // First file is debian-binary.
        // It contains just a version of dpkg used for deb creation
        // The latest one is 2.0 so we'll use this
        // Although, creating new file costs more but I did't find way to do this better
        let binary_file_path = self.temp_dir.join("debian-binary");
        let mut version_file = File::create(&binary_file_path)?;
        let _ = version_file.write("2.0\n".as_bytes());

        builder.append_file(
            "debian-binary".as_bytes(),
            &mut File::open(&binary_file_path)?
        ).unwrap();

        // Second file is control archive. It is compressed with gzip and packed with tar
        builder.append_file(
            "control.tar.gz".as_bytes(),
            &mut File::open(control_compressed)?
        ).unwrap();

        // Third - main archive with data. Compressed and package same way as control
        builder.append_file(
            "data.tar.gz".as_bytes(),
            &mut File::open(data_compressed)?
        ).unwrap();

        return Ok(());
    }
}
