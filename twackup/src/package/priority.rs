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

use super::PackageError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Priority {
    Optional,
    Required,
    Important,
    Standard,
    Extra,
    Unknown,
}

impl Priority {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Optional => "optional",
            Self::Required => "required",
            Self::Important => "important",
            Self::Standard => "standard",
            Self::Extra => "extra",
            Self::Unknown => "unknown",
        }
    }
}

impl TryFrom<&str> for Priority {
    type Error = PackageError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "optional" => Ok(Self::Optional),
            "required" => Ok(Self::Required),
            "important" => Ok(Self::Important),
            "standard" => Ok(Self::Standard),
            "extra" => Ok(Self::Extra),
            "unknown" => Ok(Self::Unknown),
            _ => Err(PackageError::UnknownPriority(value.to_string())),
        }
    }
}
