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

/// Describes field type
#[derive(Clone, Debug, Eq, PartialEq, Hash, StrEnumWithDefault)]
#[twackup(convert_all = "train")]
#[non_exhaustive]
pub enum Field {
    /// The value of this field determines the package name, and
    /// is used to generate file names by most installation tools
    Package,
    /// Typically, this is the original package's version number
    /// in whatever form the program's author uses. It may also
    /// include a Debian revision number (for non-native packages).
    Version,
    /// The architecture specifies which type of hardware this
    /// package was compiled for.
    Architecture,
    /// This field is used to indicate how this package should
    /// behave on a multi-arch installations.
    MultiArch,
    /// This is a general field that gives the package a category
    /// based on the software that it installs
    Section,
    /// The format for the package description is a short brief
    /// summary on the first line (after the Description field).
    /// The following lines should be used as a longer, more
    /// detailed description. Each line of the long description
    /// must be preceded by a space, and blank lines in the long
    /// description must contain a single ‘.’ following the
    /// preceding space
    Description,
    /// author of the software that was packaged
    Author,
    /// The person who created the package
    Maintainer,
    /// The upstream project home page url
    Homepage,
    /// Denotes a package that is required for proper operation of the system
    Essential,

    /// Current installation status
    Status,
    /// The approximate total size of the package's installed
    /// files, in KiB units
    InstalledSize,

    /// Sets the importance of this package in relation to the
    /// system as a whole.
    Priority,
    /// List of packages that are required for this package to
    /// provide a non-trivial amount of functionality.
    Depends,
    /// List of packages that must be installed and configured
    /// before this one can be installed
    PreDepends,
    /// Lists packages that would be found together with this one
    /// in all but unusual installations
    Recommends,
    /// Lists packages that are related to this one and can
    /// perhaps enhance its usefulness, but without which
    /// installing this package is perfectly reasonable
    Suggests,
    /// Lists packages that this one breaks, for example by
    /// exposing bugs when the named packages rely on this one
    Breaks,
    /// Lists packages that conflict with this one, for example by
    /// containing files with the same names
    Conflicts,
    /// This is a list of virtual packages that this one provides
    Provides,
    /// List of packages files from which this one replaces
    Replaces,

    /// User-readable name of the package
    Name,
    /// Meta-info as os support and etc...
    Tag,
    /// Like homepage
    Depiction,

    /// Other field that is not in the list
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
