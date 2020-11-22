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

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Field {
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

impl Field {
    pub fn as_str(&self) -> &str {
        match self {
            Field::Package => "Package",
            Field::Name => "Name",
            Field::Version => "Version",
            Field::Architecture => "Architecture",
            Field::State => "State",
            Field::Section => "Section",
            Field::Priority => "Priority",
            Field::Depends => "Depends",
            Field::PreDepends => "Pre-Depends",
            Field::Description => "Description",
            Field::Status => "Status",
            Field::Depiction => "Depiction",
            Field::Maintainer => "Maintainer",
            Field::Homepage => "Homepage",
            Field::Recommends => "Recommends",
            Field::Suggests => "Suggests",
            Field::Conflicts => "Conflicts",
            Field::Breaks => "Conflicts",
            Field::Provides => "Provides",
            Field::Replaces => "Replaces",
            Field::InstalledSize => "Installed-Size",
            Field::Tag => "Tag",
            Field::MultiArch => "Multi-Arch",
            Field::Author => "Author",
            Field::Essential => "Essential",
            Field::Custom(str) => str.as_str(),
        }
    }
}

impl FromStr for Field {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        Ok(match str {
            "Package" => Field::Package,
            "Name" => Field::Name,
            "Version" => Field::Version,
            "Architecture" => Field::Architecture,
            "State" => Field::State,
            "Section" => Field::Section,
            "Priority" => Field::Priority,
            "Depends" => Field::Depends,
            "Pre-Depends" => Field::PreDepends,
            "Description" => Field::Description,
            "Status" => Field::Status,
            "Depiction" =>Field::Depiction,
            "Maintainer" => Field::Maintainer,
            "Homepage" => Field::Homepage,
            "Recommends" => Field::Recommends,
            "Suggests" => Field::Suggests,
            "Conflicts" => Field::Conflicts,
            "Breaks" => Field::Breaks,
            "Provides" => Field::Provides,
            "Replaces" => Field::Replaces,
            "Installed-Size" => Field::InstalledSize,
            "Tag" => Field::Tag,
            "Multi-Arch" => Field::MultiArch,
            "Author" => Field::Author,
            "Essential" => Field::Essential,
            _ => Field::Custom(str.to_string()),
        })
    }
}
