use crate::package::{SelectionState, State, Status, StatusFlags};
use safer_ffi::derive_ReprC;
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

/// dpkg current package state
#[derive_ReprC]
#[repr(u8)]
pub enum TwPackageSelectionState {
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

impl From<SelectionState> for TwPackageSelectionState {
    fn from(value: SelectionState) -> Self {
        match value {
            SelectionState::Unknown => TwPackageSelectionState::Unknown,
            SelectionState::Install => TwPackageSelectionState::Install,
            SelectionState::Hold => TwPackageSelectionState::Hold,
            SelectionState::DeInstall => TwPackageSelectionState::DeInstall,
            SelectionState::Purge => TwPackageSelectionState::Purge,
        }
    }
}

/// dpkg-set flags for package
#[derive_ReprC]
#[repr(u8)]
pub enum TwPackageStatusFlag {
    /// A package marked ok is in a known state,
    /// but might need further processing.
    Ok,
    /// A package marked reinstreq is broken and requires installation
    ReInstallRequest,
}

impl From<StatusFlags> for TwPackageStatusFlag {
    fn from(value: StatusFlags) -> Self {
        match value {
            StatusFlags::Ok => TwPackageStatusFlag::Ok,
            StatusFlags::ReInstallRequest => TwPackageStatusFlag::ReInstallRequest,
        }
    }
}

/// dpkg-set current package state
#[derive_ReprC]
#[repr(u8)]
pub enum TwPackageCurrentState {
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

impl From<State> for TwPackageCurrentState {
    fn from(value: State) -> Self {
        match value {
            State::NotInstalled => TwPackageCurrentState::NotInstalled,
            State::ConfigFiles => TwPackageCurrentState::ConfigFiles,
            State::HalfInstalled => TwPackageCurrentState::HalfInstalled,
            State::Unpacked => TwPackageCurrentState::Unpacked,
            State::HalfConfigured => TwPackageCurrentState::HalfConfigured,
            State::TriggersAwaited => TwPackageCurrentState::TriggersAwaited,
            State::TriggersPending => TwPackageCurrentState::TriggersPending,
            State::Installed => TwPackageCurrentState::Installed,
        }
    }
}

#[derive_ReprC]
#[repr(C)]
pub struct TwPackageState {
    selection_state: TwPackageSelectionState,
    status_flag: TwPackageStatusFlag,
    current_state: TwPackageCurrentState,
}

impl From<Status> for TwPackageState {
    fn from(value: Status) -> Self {
        Self {
            selection_state: value.selection_state.into(),
            status_flag: value.flags.into(),
            current_state: value.state.into(),
        }
    }
}
