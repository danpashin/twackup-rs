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

pub(crate) type Result<T> = std::result::Result<T, SerdeError>;

#[derive(thiserror::Error, Debug)]
pub(crate) enum SerdeError {
    #[error("Io({0})")]
    Io(#[from] std::io::Error),

    #[error("JSON({0})")]
    Json(#[from] serde_json::Error),

    #[error("TomlSerialize({0})")]
    TomlSerialize(#[from] toml::ser::Error),

    #[error("TomlDeserialize({0})")]
    TomlDeserialize(#[from] toml::de::Error),

    #[error("Yaml({0})")]
    Yaml(#[from] serde_yaml::Error),
}
