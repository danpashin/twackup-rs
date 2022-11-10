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

use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum FieldName {
    // main info fields
    Package,
    Name,
    Version,
    Architecture,
    MultiArch,
    State,
    Section,
    Description,
    Depiction,
    Author,
    Maintainer,
    Homepage,
    Tag,
    Essential,

    // database status
    Status,
    InstalledSize,

    // relation with other packages
    Priority,
    Depends,
    PreDepends,
    Recommends,
    Suggests,
    Conflicts,
    Breaks,
    Provides,
    Replaces,

    Custom(String),
}

impl FieldName {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Package => "Package",
            Self::Name => "Name",
            Self::Version => "Version",
            Self::Architecture => "Architecture",
            Self::State => "State",
            Self::Section => "Section",
            Self::Priority => "Priority",
            Self::Depends => "Depends",
            Self::PreDepends => "Pre-Depends",
            Self::Description => "Description",
            Self::Status => "Status",
            Self::Depiction => "Depiction",
            Self::Maintainer => "Maintainer",
            Self::Homepage => "Homepage",
            Self::Recommends => "Recommends",
            Self::Suggests => "Suggests",
            Self::Conflicts => "Conflicts",
            Self::Breaks => "Conflicts",
            Self::Provides => "Provides",
            Self::Replaces => "Replaces",
            Self::InstalledSize => "Installed-Size",
            Self::Tag => "Tag",
            Self::MultiArch => "Multi-Arch",
            Self::Author => "Author",
            Self::Essential => "Essential",
            Self::Custom(str) => str.as_str(),
        }
    }
}

impl FromStr for FieldName {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        Ok(match str {
            "Package" => Self::Package,
            "Name" => Self::Name,
            "Version" => Self::Version,
            "Architecture" => Self::Architecture,
            "State" => Self::State,
            "Section" => Self::Section,
            "Priority" => Self::Priority,
            "Depends" => Self::Depends,
            "Pre-Depends" => Self::PreDepends,
            "Description" => Self::Description,
            "Status" => Self::Status,
            "Depiction" => Self::Depiction,
            "Maintainer" => Self::Maintainer,
            "Homepage" => Self::Homepage,
            "Recommends" => Self::Recommends,
            "Suggests" => Self::Suggests,
            "Conflicts" => Self::Conflicts,
            "Breaks" => Self::Breaks,
            "Provides" => Self::Provides,
            "Replaces" => Self::Replaces,
            "Installed-Size" => Self::InstalledSize,
            "Tag" => Self::Tag,
            "Multi-Arch" => Self::MultiArch,
            "Author" => Self::Author,
            "Essential" => Self::Essential,
            _ => Self::Custom(str.to_string()),
        })
    }
}

impl AsRef<FieldName> for FieldName {
    fn as_ref(&self) -> &FieldName {
        self
    }
}
