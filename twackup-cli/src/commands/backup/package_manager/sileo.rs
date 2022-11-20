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

use super::{Hooks, NewStylePackageManager, PackageManagerDescription};
use crate::{commands::backup::RepoGroup, Result};
use std::path::PathBuf;

pub(crate) struct Sileo {
    pub(crate) binary_name: &'static str,
    pub(crate) sources: &'static str,
}

impl Sileo {
    pub(crate) const fn new() -> Self {
        Self {
            binary_name: "Sileo",
            sources: "/etc/apt/sources.list.d/sileo.sources",
        }
    }
}

impl Hooks for Sileo {
    fn post_import(&self, _repo_group: &RepoGroup) -> Result<()> {
        Ok(())
    }
}

impl PackageManagerDescription for Sileo {
    fn exec_name(&self) -> &str {
        self.binary_name
    }

    fn repos_file_path(&self) -> PathBuf {
        PathBuf::from(self.sources)
    }
}

impl NewStylePackageManager for Sileo {}
