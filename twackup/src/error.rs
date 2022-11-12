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

pub type Result<T> = std::result::Result<T, Generic>;

#[derive(thiserror::Error, Debug)]
pub enum Generic {
    #[error("IO: {0}")]
    Io(#[from] std::io::Error),

    #[error("This action requires root permissions.")]
    NotRunningAsRoot,

    #[cfg(feature = "with_serde")]
    #[error("PlistError({0})")]
    Plist(#[from] plist::Error),

    #[error("PackageError: {0}")]
    PackageError(#[from] crate::package::Error),

    #[error("RepoError: {0}")]
    RepoError(#[from] crate::repository::RepoError),

    #[error("LockError")]
    LockError,
}
