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

use super::RepoError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Category {
    /// Used for distributing binaries only
    Binary,
    /// This is used for distributing sources only
    Source,
    /// Supported only in DEB822 format
    Both,
}

impl Category {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Binary => "deb",
            Self::Source => "deb-src",
            Self::Both => "deb deb-src",
        }
    }
}

impl TryFrom<&str> for Category {
    type Error = RepoError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "deb" => Ok(Self::Binary),
            "deb-src" => Ok(Self::Source),
            "deb deb-src" | "deb-src deb" => Ok(Self::Both),
            _ => Err(RepoError::MissingField(s.to_string())),
        }
    }
}
