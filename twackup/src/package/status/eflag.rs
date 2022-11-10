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

use crate::package::PackageError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EFlag {
    Ok,
    ReInstallRequest,
}

impl EFlag {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ok => "ok",
            Self::ReInstallRequest => "reinstreq",
        }
    }
}

impl TryFrom<&str> for EFlag {
    type Error = PackageError;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        match string {
            "ok" => Ok(Self::Ok),
            "reinstreq" => Ok(Self::ReInstallRequest),
            _ => Err(PackageError::UnknownEFlag(string.to_string())),
        }
    }
}
