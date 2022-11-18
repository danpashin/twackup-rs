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

use crate::package::Section;
use safer_ffi::derive_ReprC;

#[derive_ReprC]
#[repr(u8)]
#[non_exhaustive]
pub enum TwPackageSection {
    /// Other not listed package sections
    Other = 0,
    /// Different archiving utils
    Archiving,
    /// Developers header files and etc.
    Development,
    /// Utils for use with network
    Networking,
    /// Other archiving utils
    Packaging,
    /// System packages
    System,
    /// Terminal
    TerminalSupport,
    /// Text editors
    TextEditors,
    /// Themes
    Themes,
    /// Tweaks
    Tweaks,
    /// Different utilities
    Utilities,
}

impl From<&Section> for TwPackageSection {
    fn from(value: &Section) -> Self {
        match value {
            Section::Archiving => Self::Archiving,
            Section::Development => Self::Development,
            Section::Networking => Self::Networking,
            Section::Packaging => Self::Packaging,
            Section::System => Self::System,
            Section::TerminalSupport => Self::TerminalSupport,
            Section::TextEditors => Self::TextEditors,
            Section::Themes => Self::Themes,
            Section::Tweaks => Self::Tweaks,
            Section::Utilities => Self::Utilities,
            Section::Other(_) => Self::Other,
        }
    }
}
