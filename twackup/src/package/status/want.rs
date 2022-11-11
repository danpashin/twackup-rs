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

use crate::package::Error;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Want {
    Unknown,
    Install,
    Hold,
    DeInstall,
    Purge,
}

impl Want {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Unknown => "unknown",
            Self::Install => "install",
            Self::Hold => "hold",
            Self::DeInstall => "deinstall",
            Self::Purge => "purge",
        }
    }
}

impl TryFrom<&str> for Want {
    type Error = Error;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        match string {
            "unknown" => Ok(Self::Unknown),
            "install" => Ok(Self::Install),
            "hold" => Ok(Self::Hold),
            "deinstall" => Ok(Self::DeInstall),
            "purge" => Ok(Self::Purge),
            _ => Err(Error::UnknownState(string.to_string())),
        }
    }
}

impl Display for Want {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
