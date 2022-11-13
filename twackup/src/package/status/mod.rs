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
use super::Error;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Status {
    want: Want,
    e_flag: EFlag,
    state: State,
}

impl TryFrom<&str> for Status {
    type Error = Error;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        let components: Vec<_> = string.split_whitespace().collect();
        if components.len() != 3 {
            return Err(Error::UnknownState(string.to_string()));
        }

        Ok(Self {
            want: components[0].try_into().map_err(Error::UnknownWant)?,
            e_flag: components[1].try_into().map_err(Error::UnknownEFlag)?,
            state: components[2].try_into().map_err(Error::UnknownState)?,
        })
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.want, self.e_flag, self.state)
    }
}
