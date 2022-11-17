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

//! This module represents some traits used for allowing
//! user to see packages build progress.
//!
//! Should be probably refactored.

#![allow(unused_variables)]

/// Allow users to see progress
pub trait Progress {
    /// Should construct self for total packages/items
    fn new(total: u64) -> Self;

    /// Will be called when total progress is incrementing
    fn increment(&self, delta: u64) {}

    /// For cleanup and finish
    fn finish(&self) {}

    /// Prints message. Should be probably removed.
    fn print<M: AsRef<str>>(&self, message: M) {}

    /// Prints or logs warning message
    fn print_warning<M: AsRef<str>>(&self, message: M) {}

    /// Prints or logs error message
    fn print_error<M: AsRef<str>>(&self, message: M) {}

    /// Sets current progress message
    fn set_message<M: AsRef<str>>(&self, message: M) {}
}
