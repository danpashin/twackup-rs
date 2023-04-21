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

mod error;

pub(crate) use error::{Result, SerdeError};
use serde::{de::DeserializeOwned, Serialize};
use std::io::{Read, Write};

/// Data format used for export and import commands
#[derive(clap::Parser, clap::ValueEnum, PartialEq, Eq, Clone)]
pub(crate) enum Format {
    Json,
    Toml,
    Yaml,
}

impl Format {
    pub(crate) fn ser_to_writer<T: Serialize, W: Write>(
        &self,
        mut writer: W,
        value: &T,
    ) -> Result<()> {
        match self {
            Self::Json => serde_json::to_writer(writer, value)?,
            Self::Toml => {
                let result = toml::to_string(value)?;
                writer.write_all(result.as_bytes())?;
            }
            Self::Yaml => serde_yaml::to_writer(writer, value)?,
        }

        Ok(())
    }

    pub(crate) fn de_from_reader<T: DeserializeOwned, R: Read>(&self, mut reader: R) -> Result<T> {
        Ok(match self {
            Self::Json => serde_json::from_reader(reader)?,
            Self::Toml => unsafe {
                let mut data = String::new();
                reader.read_to_end(data.as_mut_vec())?;
                toml::from_str(&data)?
            },
            Self::Yaml => serde_yaml::from_reader(reader)?,
        })
    }
}
