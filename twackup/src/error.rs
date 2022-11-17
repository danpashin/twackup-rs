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

/// Defines common result type for all errors
pub type Result<T> = core::result::Result<T, Generic>;

/// Defines Twackup generic error
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Generic {
    /// There was some IO error
    #[error("IO: {0}")]
    Io(#[from] std::io::Error),

    /// Is needed for applying some operations that cannot be made under not-root user
    #[error("This action requires root permissions.")]
    #[cfg(feature = "ios")]
    NotRunningAsRoot,

    /// Some package parsing error
    #[error("PackageError: {0}")]
    Package(#[from] crate::package::Error),

    /// Some package parsing error
    #[error("RepoError: {0}")]
    Repo(#[from] crate::repository::Error),

    /// Is used when dpkg database lock failed
    #[error("Lock")]
    Lock,

    /// Another IO error
    #[error("Path must have file ending")]
    PathMustHaveFileEnding,

    /// Is used when time is backwards
    #[error("SystemTimeError")]
    SystemTime(#[from] std::time::SystemTimeError),
}
