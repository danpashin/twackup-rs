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

use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct Paths(pub PathBuf);

impl Paths {
    #[inline]
    pub fn new<P: AsRef<Path>>(dpkg_dir: P) -> Self {
        Self(dpkg_dir.as_ref().to_path_buf())
    }

    #[must_use]
    #[inline]
    pub fn status_file(&self) -> PathBuf {
        self.0.join("status")
    }

    #[must_use]
    #[inline]
    pub fn info_dir(&self) -> PathBuf {
        self.0.join("info")
    }

    #[must_use]
    #[inline]
    pub fn lock_file(&self) -> PathBuf {
        self.0.join("lock")
    }
}

impl AsRef<PathBuf> for Paths {
    fn as_ref(&self) -> &PathBuf {
        &self.0
    }
}

impl From<&Path> for Paths {
    fn from(value: &Path) -> Self {
        Self::new(value)
    }
}

impl From<&Paths> for Paths {
    fn from(value: &Paths) -> Self {
        value.clone()
    }
}
impl From<&PathBuf> for Paths {
    fn from(value: &PathBuf) -> Self {
        Self::new(value)
    }
}
