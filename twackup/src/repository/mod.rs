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

mod category;
mod repo_error;

pub use self::{category::Category, repo_error::RepoError};
use crate::parser::Parsable;
use std::{collections::HashMap, string::ToString};

#[cfg(feature = "with_serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "with_serde", derive(Serialize, Deserialize))]
pub struct Repository {
    /// specifies type of repo packages - Binary or Source
    pub category: Category,

    /// Specifies the root of the archive
    pub url: String,

    /// Specifies a subdirectory in $ARCHIVE_ROOT/dists
    pub distribution: String,

    pub components: Vec<String>,
}

impl Parsable for Repository {
    type Error = RepoError;

    /// Performs parsing repo model in DEB822 format
    /// #### Doesn't support options
    fn new(fields: HashMap<String, String>) -> Result<Self, Self::Error> {
        let mut fields = fields;
        let mut fetch_field = |field: &str| -> Result<String, RepoError> {
            fields
                .remove(field)
                .ok_or_else(|| RepoError::MissingField(field.to_string()))
        };

        Ok(Self {
            category: Category::try_from(fetch_field("Types")?.as_str())?,
            url: fetch_field("URIs")?,
            distribution: fetch_field("Suites")?,
            components: fetch_field("Components")
                .map(|components| {
                    components
                        .split_ascii_whitespace()
                        .map(ToString::to_string)
                        .collect()
                })
                .unwrap_or_default(),
        })
    }
}

impl Repository {
    /// Performs parsing model from one-line style.
    /// **Doesn't support options as they aren't used in iOS.**
    ///
    /// # Errors
    /// Return error if line doesn't consist of three or more components
    pub fn from_one_line(line: &str) -> Result<Self, RepoError> {
        let components: Vec<&str> = line.split_ascii_whitespace().collect();
        // type, uri and suite are required, so break if they don't exist
        if components.len() < 3 {
            return Err(RepoError::InvalidRepoLine(line.to_string()));
        }

        Ok(Self {
            category: Category::try_from(components[0])?,
            url: components[1].to_string(),
            distribution: components[2].to_string(),
            components: components
                .into_iter()
                .skip(3)
                .map(ToString::to_string)
                .collect(),
        })
    }

    /// Performs fields formatting in the one-line style.
    /// #### Doesn't support options
    #[must_use]
    pub fn to_one_line(&self) -> String {
        format!(
            "{} {} {} {}",
            self.category.as_str(),
            self.url,
            self.distribution,
            self.components.join(" ")
        )
        .trim()
        .to_string()
    }

    /// Performs fields formatting in multiple-lines style. (Also known as DEB822 Style)
    /// #### Doesn't support options
    #[must_use]
    pub fn to_deb822(&self) -> String {
        format!(
            "Types: {}\nURIs: {}\nSuites: {}\nComponents: {}",
            self.category.as_str(),
            self.url,
            self.distribution,
            self.components.join(" ")
        )
        .trim()
        .to_string()
    }

    #[must_use]
    pub fn to_cydia_key(&self) -> String {
        format!(
            "{}:{}:{}",
            self.category.as_str(),
            self.url,
            self.distribution
        )
    }

    #[must_use]
    #[cfg(feature = "with_serde")]
    pub fn to_dict(&self) -> plist::Dictionary {
        let mut dict = plist::Dictionary::new();
        dict.insert(
            "Distribution".to_string(),
            plist::Value::String(self.distribution.clone()),
        );
        dict.insert("URI".to_string(), plist::Value::String(self.url.clone()));
        dict.insert(
            "Type".to_string(),
            plist::Value::String(self.category.as_str().to_string()),
        );
        dict.insert(
            "Sections".to_string(),
            plist::Value::Array(
                self.components
                    .iter()
                    .map(|val| plist::Value::String(val.clone()))
                    .collect(),
            ),
        );

        dict
    }
}
