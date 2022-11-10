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

use super::FieldName;

#[derive(thiserror::Error, Debug)]
pub enum PackageError {
    #[error("Unknown package priority: `{0}`")]
    UnknownPriority(String),

    #[error("Unknown package eflag field: `{0}`")]
    UnknownEFlag(String),

    #[error("Unknown package state: `{0}`")]
    UnknownState(String),

    #[error("Unknown package want field `{0}`")]
    UnknownWant(String),

    #[error("Field is missed: `{0}`")]
    MissingField(FieldName),

    #[error("This package is virtual")]
    VirtualPackage,
}
