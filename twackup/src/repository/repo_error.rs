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

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum RepoError {
    #[error("Missed field `{0}`")]
    MissingField(String),

    #[error("Category {0} is invalid")]
    InvalidCategory(String),

    #[error("Repo line {0} is invalid")]
    InvalidRepoLine(String),
}
