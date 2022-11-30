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

#![allow(unused_variables)]

use crate::package::Package;
use std::path::Path;

pub enum MessageLevel {
    Debug,
    Info,
    Warning,
    Error,
}

/// Allow users to see progress
pub trait Progress {
    /// Prints message. Should be probably removed.
    fn print_message<M: AsRef<str>>(&self, message: M, level: MessageLevel);

    /// Sets current progress message
    fn started_processing(&self, package: &Package);

    /// Will be called when total progress is incrementing
    fn finished_processing<P: AsRef<Path>>(&self, package: &Package, deb_path: P);

    /// For cleanup and finish
    fn finished_all(&self);
}
