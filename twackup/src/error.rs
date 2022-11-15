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

pub type Result<T> = core::result::Result<T, Generic>;

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Generic {
    #[error("IO: {0}")]
    Io(#[from] std::io::Error),

    #[error("This action requires root permissions.")]
    NotRunningAsRoot,

    #[error("PackageError: {0}")]
    Package(#[from] crate::package::Error),

    #[error("RepoError: {0}")]
    Repo(#[from] crate::repository::RepoError),

    #[error("Lock")]
    Lock,

    #[error("Path must have file ending")]
    PathMustHaveFileEnding,

    #[error("SystemTimeError")]
    SystemTime(#[from] std::time::SystemTimeError),
}
