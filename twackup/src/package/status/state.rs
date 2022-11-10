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

use crate::error::Error;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum State {
    NotInstalled,
    ConfigFiles,
    HalfInstalled,
    Unpacked,
    HalfConfigured,
    TriggersAwaited,
    TriggersPending,
    Installed,
}

impl State {
    pub fn as_str(&self) -> &str {
        match self {
            Self::NotInstalled => "not-installed",
            Self::ConfigFiles => "config-files",
            Self::HalfInstalled => "half-installed",
            Self::Unpacked => "unpacked",
            Self::HalfConfigured => "half-configured",
            Self::TriggersAwaited => "triggers-awaited",
            Self::TriggersPending => "triggers-pending",
            Self::Installed => "installed",
        }
    }
}

impl TryFrom<&str> for State {
    type Error = Error;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        match string {
            "not-installed" => Ok(Self::NotInstalled),
            "config-files" => Ok(Self::ConfigFiles),
            "half-installed" => Ok(Self::HalfInstalled),
            "unpacked" => Ok(Self::Unpacked),
            "half-configured" => Ok(Self::HalfConfigured),
            "triggers-awaited" => Ok(Self::TriggersAwaited),
            "triggers-pending" => Ok(Self::TriggersPending),
            "installed" => Ok(Self::Installed),
            _ => Err(Error::UnknownState(string.to_string())),
        }
    }
}
