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

use twackup_derive::StrEnumWithError;

/// Wrapper on package priority
#[derive(Clone, Debug, PartialEq, Eq, StrEnumWithError)]
#[twackup(convert_all = "lower")]
#[non_exhaustive]
pub enum Priority {
    /// Package is optional for installation and removal
    Optional,
    /// Package is required for system to work
    Required,
    /// Package is important for installed system
    Important,
    /// Package is shipped with default priority
    Standard,
    /// Package is important for installed system
    Extra,
}
