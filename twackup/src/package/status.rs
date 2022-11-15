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

use super::Error;
use std::fmt::{Display, Formatter};
use twackup_derive::StrEnumWithError;

/// Wrapper for dpkg current package state
#[derive(Clone, Debug, PartialEq, Eq, StrEnumWithError)]
#[twackup(convert_all = "lower")]
pub enum SelectionState {
    /// The package selection is unknown. A package that is also
    /// in a not-installed state, and with an ok flag will be
    /// forgotten in the next database store.
    Unknown,
    /// The package is selected for installation
    Install,
    /// Such a package is not handled by dpkg
    Hold,
    /// The package is marked for deinstallation.
    /// The configuration will be kept
    DeInstall,
    /// The package is marked for deinstallation.
    /// The configuration will be removed, too
    Purge,
}

impl Display for SelectionState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Wrapper for dpkg-set flags for package
#[derive(Clone, Debug, PartialEq, Eq, StrEnumWithError)]
#[twackup(convert_all = "lower")]
pub enum Flags {
    /// A package marked ok is in a known state,
    /// but might need further processing.
    Ok,
    /// A package marked reinstreq is broken and requires installation
    #[twackup(rename = "reinstreq")]
    ReInstallRequest,
}

impl Display for Flags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Wrapper for dpkg-set current package state
#[derive(Clone, Debug, PartialEq, Eq, StrEnumWithError)]
#[twackup(convert_all = "kebab")]
pub enum State {
    /// The package is not installed on a system
    NotInstalled,
    /// Only a packages configuration files exist on a system
    ConfigFiles,
    /// An installation of a package was started, but not finished
    HalfInstalled,
    /// The package is unpacked but not configured
    Unpacked,
    /// The package is unpacked and its configuration was started, but not finished
    HalfConfigured,
    /// The package awaits trigger processing by another package
    TriggersAwaited,
    /// The package has been triggered
    TriggersPending,
    /// The package is correctly unpacked and configured
    Installed,
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Wrapper of the dpkg Status database field
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Status {
    /// Current dpkg selection state of this package.
    /// Typically, should be  **Install**
    selection_state: SelectionState,
    /// Package different flags. Should always be **Ok**
    flags: Flags,
    /// Current installation state of this package.
    /// Typically, should be  **Installed**
    state: State,
}

impl TryFrom<&str> for Status {
    type Error = Error;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        let components: Vec<_> = string.split_whitespace().collect();
        if components.len() != 3 {
            return Err(Error::UnknownState(string.to_owned()));
        }

        Ok(Self {
            selection_state: components[0]
                .try_into()
                .map_err(|error: &str| Error::UnknownSelectionState(error.to_string()))?,
            flags: components[1]
                .try_into()
                .map_err(|error: &str| Error::UnknownFlag(error.to_string()))?,
            state: components[2]
                .try_into()
                .map_err(|error: &str| Error::UnknownState(error.to_string()))?,
        })
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.selection_state, self.flags, self.state)
    }
}
