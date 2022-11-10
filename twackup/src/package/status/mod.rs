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

pub mod eflag;
pub mod state;
pub mod want;

pub use self::{eflag::EFlag, state::State, want::Want};
use super::PackageError;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Status {
    want: Want,
    e_flag: EFlag,
    state: State,
}

impl TryFrom<&str> for Status {
    type Error = PackageError;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        let mut components = string.split_whitespace();
        let want = components
            .next()
            .ok_or_else(|| PackageError::UnknownState(string.to_string()))?;
        let eflag = components
            .next()
            .ok_or_else(|| PackageError::UnknownState(string.to_string()))?;
        let status = components
            .next()
            .ok_or_else(|| PackageError::UnknownState(string.to_string()))?;

        Ok(Self {
            want: want.try_into()?,
            e_flag: eflag.try_into()?,
            state: status.try_into()?,
        })
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.want, self.e_flag, self.state)
    }
}
