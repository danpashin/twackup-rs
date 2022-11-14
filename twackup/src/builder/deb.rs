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

use async_compression::tokio::write::GzipEncoder;
use async_compression::Level;
use std::{
    borrow::BorrowMut,
    io::{self},
    path::{Path, PathBuf},
    time::SystemTime,
};
use tokio::io::AsyncWriteExt;
use tokio_tar::Builder as Tar;

pub type DebianTarArchive = TarArchive<GzipEncoder<Vec<u8>>>;

pub struct Deb {
    output: PathBuf,
    control: DebianTarArchive,
    data: DebianTarArchive,
}

pub struct TarArchive<W: tokio::io::AsyncWrite + Unpin + Send + Sync + 'static> {
    builder: Tar<W>,
}

impl Deb {
    /// Constructs debian archive instance
    ///
    /// # Errors
    /// Returns IO error if temp dir is not writable
    pub fn new<O: AsRef<Path>>(output: O, compression: u32) -> Self {
        let control_file = GzipEncoder::with_quality(vec![], Level::Precise(compression));
        let data_file = GzipEncoder::with_quality(vec![], Level::Precise(compression));

        Self {
            output: output.as_ref().to_path_buf(),
            control: TarArchive::new(control_file),
            data: TarArchive::new(data_file),
        }
    }

    pub fn data_mut_ref(&mut self) -> &mut DebianTarArchive {
        self.data.borrow_mut()
    }

    pub fn control_mut_ref(&mut self) -> &mut DebianTarArchive {
        self.control.borrow_mut()
    }

    /// Construct debian package
    ///
    /// # Errors
    /// Returns IO error if temp dir is not writable
    pub async fn build(self) -> io::Result<()> {
        let mut builder = ar::Builder::new(std::fs::File::create(&self.output)?);

        let mtime = current_timestamp();

        let mut append_data = |name: Vec<u8>, data: &[u8]| {
            let mut header = ar::Header::new(name, data.len() as u64);
            header.set_mode(0o100_644); // o=rw,g=r,o=r
            header.set_mtime(mtime); // modify time
            header.set_uid(0); // root
            header.set_gid(0); // root
            builder.append(&header, data)
        };

        let version = "2.0\n".as_bytes();
        append_data(b"debian-binary".to_vec(), version)?;

        let mut control_encoder = self.control.builder.into_inner().await?;
        control_encoder.shutdown().await?;

        let control = control_encoder.into_inner();
        append_data(b"control.tar.gz".to_vec(), control.as_slice())?;

        let mut data_encoder = self.data.builder.into_inner().await?;
        data_encoder.shutdown().await?;

        let control = data_encoder.into_inner();
        append_data(b"data.tar.gz".to_vec(), control.as_slice())?;

        Ok(())
    }
}

impl<W: tokio::io::AsyncWrite + Unpin + Send + Sync> TarArchive<W> {
    pub fn new(writer: W) -> Self {
        let mut builder = Tar::new(writer);
        builder.follow_symlinks(false);
        Self { builder }
    }

    pub fn get_mut(&mut self) -> &mut Tar<W> {
        &mut self.builder
    }

    /// Appends non-existing on the filesystem file to archive
    ///
    /// # Errors
    /// Returns error if file couldn't be added to archive
    pub async fn append_new_file<P: AsRef<Path>>(
        &mut self,
        path: P,
        contents: &[u8],
    ) -> io::Result<()> {
        let mut header = tokio_tar::Header::new_old();
        header.set_mode(0o100_644); // o=rw,g=r,o=r
        header.set_uid(0);
        header.set_gid(0);
        header.set_size(contents.len() as u64);
        header.set_mtime(current_timestamp()); // modify time

        self.builder
            .append_data(&mut header, path, contents)
            .await
            .expect("append failed");

        Ok(())
    }
}

/// Returns UNIX timestamp in seconds
#[inline]
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
