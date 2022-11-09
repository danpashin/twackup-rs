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

use std::{
    fs::{self, File},
    io,
    os::unix::io::AsRawFd,
    path::{Path, PathBuf},
};

pub(crate) struct Lock {
    path: PathBuf,
    file: File,
}

impl Lock {
    pub(crate) fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref().join("lock");
        let file = File::create(&path)?;
        flock(&file, libc::LOCK_EX)?;

        Ok(Self { path, file })
    }
}

impl Drop for Lock {
    fn drop(&mut self) {
        let _ = flock(&self.file, libc::LOCK_UN);
        let _ = fs::remove_file(&self.path);
    }
}

fn flock(file: &File, flag: libc::c_int) -> std::io::Result<()> {
    let ret = unsafe { libc::flock(file.as_raw_fd(), flag) };
    if ret < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}
