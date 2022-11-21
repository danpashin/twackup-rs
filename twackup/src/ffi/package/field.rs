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

use crate::package::Field;
use safer_ffi::derive_ReprC;

/// Describes field type
#[derive_ReprC]
#[repr(u8)]
#[non_exhaustive]
pub enum TwPackageField {
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
}

impl From<TwPackageField> for Field {
    fn from(value: TwPackageField) -> Self {
        match value {
            TwPackageField::Package => Self::Package,
            TwPackageField::Version => Self::Version,
            TwPackageField::Architecture => Self::Architecture,
            TwPackageField::MultiArch => Self::MultiArch,
            TwPackageField::Section => Self::Section,
            TwPackageField::Description => Self::Description,
            TwPackageField::Author => Self::Author,
            TwPackageField::Maintainer => Self::Maintainer,
            TwPackageField::Homepage => Self::Homepage,
            TwPackageField::Essential => Self::Essential,
            TwPackageField::Status => Self::Status,
            TwPackageField::InstalledSize => Self::InstalledSize,
            TwPackageField::Priority => Self::Priority,
            TwPackageField::Depends => Self::Depends,
            TwPackageField::PreDepends => Self::PreDepends,
            TwPackageField::Recommends => Self::Recommends,
            TwPackageField::Suggests => Self::Suggests,
            TwPackageField::Breaks => Self::Breaks,
            TwPackageField::Conflicts => Self::Conflicts,
            TwPackageField::Provides => Self::Provides,
            TwPackageField::Replaces => Self::Replaces,
            TwPackageField::Name => Self::Name,
            TwPackageField::Tag => Self::Tag,
            TwPackageField::Depiction => Self::Depiction,
        }
    }
}
