use tar::Builder;
use flate2::{Compression, write::GzEncoder};
use std::{
    path::{Path, PathBuf},
    io::{self, Write}, fs::File,
    borrow::BorrowMut,
    time::SystemTime,
};

pub type DebTarArchive = TarArchive<GzEncoder<File>>;

pub struct Deb {
    temp_dir: PathBuf,
    output: PathBuf,
    control: DebTarArchive,
    data: DebTarArchive,
    control_path: PathBuf,
    data_path: PathBuf
}

pub struct TarArchive<W: Write> {
    builder: Builder<W>
}

impl Deb {
    pub fn new(temp_dir: &PathBuf, output: &PathBuf, compression: u32) -> io::Result<Self> {
        let control_path  = temp_dir.join("control.tar.gz");
        let data_path = temp_dir.join("data.tar.gz");

        let control_file = GzEncoder::new(
            File::create(&control_path)?,
            Compression::new(compression)
        );

        let data_file = GzEncoder::new(
            File::create(&data_path)?,
            Compression::new(compression)
        );

        Ok(Self {
            temp_dir: temp_dir.clone(),
            output: output.clone(),
            control: TarArchive::new(control_file),
            data: TarArchive::new(data_file),
            control_path, data_path
        })
    }

    pub fn data_mut_ref(&mut self) -> &mut DebTarArchive { self.data.borrow_mut() }

    pub fn control_mut_ref(&mut self) -> &mut DebTarArchive { self.control.borrow_mut() }

    pub fn package(&mut self) -> io::Result<()> {
        self.control.builder.finish()?;
        self.control.builder.get_mut().try_finish()?;
        self.data.builder.finish()?;
        self.data.builder.get_mut().try_finish()?;

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
        )?;

        // Second file is control archive. It is compressed with gzip and packed with tar
        builder.append_file(
            "control.tar.gz".as_bytes(),
            &mut File::open(&self.control_path)?
        )?;

        // Third - main archive with data. Compressed and package same way as control
        builder.append_file(
            "data.tar.gz".as_bytes(),
            &mut File::open(&self.data_path)?
        )?;

        return Ok(());
    }
}

impl<W: Write> TarArchive<W> {
    pub fn new(writer: W) -> Self {
        Self { builder: Builder::new(writer) }
    }

    pub fn get_mut(&mut self) -> &mut Builder<W> { &mut self.builder }

    /// Appends non-existing on the filesystem file to archive
    pub fn append_new_file<P: AsRef<Path>>(&mut self, path: P, contents: &[u8]) -> io::Result<()> {
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
}
