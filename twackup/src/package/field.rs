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

use std::fmt::{Display, Formatter};
use twackup_derive::StrEnumWithDefault;

#[derive(Clone, Debug, Eq, PartialEq, Hash, StrEnumWithDefault)]
#[twackup(convert_all = "train")]
#[non_exhaustive]
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

impl AsRef<Field> for Field {
    fn as_ref(&self) -> &Field {
        self
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
