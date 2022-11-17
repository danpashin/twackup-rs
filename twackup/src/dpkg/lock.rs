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

use super::paths::Paths;
use std::{
    fs::{self, File},
    io,
    os::unix::io::AsRawFd,
    path::PathBuf,
};

pub(crate) struct Lock {
    path: PathBuf,
    file: File,
}

impl Lock {
    pub(crate) fn new(paths: &Paths) -> io::Result<Self> {
        let path = paths.lock_file();
        let file = File::create(&path)?;
        flock(&file, libc::LOCK_EX)?;

        Ok(Self { path, file })
    }
}

impl Drop for Lock {
    fn drop(&mut self) {
        flock(&self.file, libc::LOCK_UN).ok();
        fs::remove_file(&self.path).ok();
    }
}

fn flock(file: &File, flag: libc::c_int) -> io::Result<()> {
    let ret = unsafe { libc::flock(file.as_raw_fd(), flag) };
    if ret < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}
