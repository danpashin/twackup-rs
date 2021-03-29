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
    output: PathBuf,
    control: DebTarArchive,
    data: DebTarArchive,
    control_path: PathBuf,
    data_path: PathBuf,
}

pub struct TarArchive<W: Write> {
    builder: Builder<W>
}

impl Deb {
    pub fn new<T: AsRef<Path>, O: AsRef<Path>>(
        temp_dir: T, output: O, compression: u32,
    ) -> io::Result<Self> {
        let control_path = temp_dir.as_ref().join("control.tar.gz");
        let data_path = temp_dir.as_ref().join("data.tar.gz");

        let control_file = GzEncoder::new(
            File::create(&control_path)?,
            Compression::new(compression),
        );

        let data_file = GzEncoder::new(
            File::create(&data_path)?,
            Compression::new(compression),
        );

        Ok(Self {
            output: output.as_ref().to_path_buf(),
            control: TarArchive::new(control_file),
            data: TarArchive::new(data_file),
            control_path,
            data_path,
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
        let version = "2.0\n".as_bytes();
        let mut header = ar::Header::new(b"debian-binary".to_vec(), version.len() as u64);
        header.set_mode(0o100644); // o=rw,g=r,o=r
        header.set_mtime(current_timestamp()); // modify time
        builder.append(&header, version)?;

        // Second file is control archive. It is compressed with gzip and packed with tar
        let data = self.prepare_path(&self.control_path, "control.tar.gz")?;
        builder.append(&data.0, data.1)?;

        // Third - main archive with data. Compressed and package same way as control
        let data = self.prepare_path(&self.data_path, "data.tar.gz")?;
        builder.append(&data.0, data.1)?;

        Ok(())
    }

    fn prepare_path<P: AsRef<Path>>(&self, path: P, name: &str) -> io::Result<(ar::Header, File)> {
        let file = File::open(path)?;
        let metadata = file.metadata()?;
        let mut header = ar::Header::from_metadata(name.as_bytes().to_vec(), &metadata);
        header.set_mode(0o100644); // o=rw,g=r,o=r
        header.set_uid(0); // root
        header.set_gid(0); // root

        Ok((header, file))
    }
}

impl<W: Write> TarArchive<W> {
    pub fn new(writer: W) -> Self {
        let mut builder = Builder::new(writer);
        builder.follow_symlinks(false);
        Self { builder }
    }

    pub fn get_mut(&mut self) -> &mut Builder<W> { &mut self.builder }

    /// Appends non-existing on the filesystem file to archive
    pub fn append_new_file<P: AsRef<Path>>(&mut self, path: P, contents: &[u8]) -> io::Result<()> {
        let mut header = tar::Header::new_old();
        header.set_mode(0o100644); // o=rw,g=r,o=r
        header.set_uid(0);
        header.set_gid(0);
        header.set_size(contents.len() as u64);
        header.set_mtime(current_timestamp()); // modify time
        header.set_cksum();

        self.builder.append_data(&mut header, path, contents)
    }
}

/// Returns UNIX timestamp in seconds
#[inline]
fn current_timestamp() -> u64 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
}
