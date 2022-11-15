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

use std::time::{Duration, Instant};

pub(crate) struct Context {
    pub(crate) start_time: Instant,
    pub(crate) is_root: bool,
}

impl Context {
    pub(crate) fn new() -> Self {
        Self {
            start_time: Instant::now(),
            is_root: libproc::libproc::proc_pid::am_root(),
        }
    }

    pub(crate) fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }
}
