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

//! Package module represents some repository info
//! that was parsed from dpkg database

use crate::Parsable;
use std::{collections::HashMap, string::ToString};
use twackup_derive::StrEnumWithDefault;

/// Different repo errors
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Some field is missing
    #[error("Missed field `{0}`")]
    MissingField(String),

    /// repo category is unknown
    #[error("Category {0} is invalid")]
    InvalidCategory(String),

    /// Something is wrong with one-line repo line
    #[error("Repo line {0} is invalid")]
    InvalidRepoLine(String),
}

/// Repo category type
#[derive(Clone, Debug, StrEnumWithDefault)]
#[cfg_attr(feature = "with-serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum Category {
    /// Used for distributing binaries only
    #[twackup(rename = "binary")]
    Binary,
    /// This is used for distributing sources only
    #[twackup(rename = "deb-src")]
    Source,
    /// Other category types
    Other(String),
}

/// Wrapper for default repo structure
#[derive(Clone, Debug)]
#[cfg_attr(feature = "with-serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub struct Repository {
    /// specifies type of repo packages - Binary or Source
    pub category: Category,

    /// Specifies the root of the archive
    pub url: String,

    /// Specifies a subdirectory in $ARCHIVE_ROOT/dists
    pub distribution: String,

    /// A whitespace separated list of areas.
    /// May also include be prefixed by parts of the path following
    /// the directory beneath dists, if the Release file is not in a directory
    /// directly beneath dists/
    pub components: Vec<String>,
}

impl Parsable for Repository {
    type Error = Error;

    /// Performs parsing repo model in DEB822 format
    /// #### Doesn't support options
    fn new(mut fields: HashMap<String, String>) -> Result<Self, Self::Error> {
        let mut fetch_field = |field: &str| -> Result<String, Error> {
            fields
                .remove(field)
                .ok_or_else(|| Error::MissingField(field.to_owned()))
        };

        Ok(Self {
            category: Category::from(fetch_field("Types")?.as_str()),
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
    pub fn from_one_line(line: &str) -> Result<Self, Error> {
        let components: Vec<&str> = line.split_ascii_whitespace().collect();
        // type, uri and suite are required, so break if they don't exist
        if components.len() < 3 {
            return Err(Error::InvalidRepoLine(line.to_owned()));
        }

        Ok(Self {
            category: Category::from(components[0]),
            url: components[1].to_owned(),
            distribution: components[2].to_owned(),
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
    #[inline]
    pub fn to_one_line(&self) -> String {
        format!(
            "{} {} {} {}",
            self.category.as_str(),
            self.url,
            self.distribution,
            self.components.join(" ").trim_end()
        )
    }

    /// Performs fields formatting in multiple-lines style. (Also known as DEB822 Style)
    /// #### Doesn't support options
    #[must_use]
    #[inline]
    pub fn to_deb822(&self) -> String {
        format!(
            "Types: {}\nURIs: {}\nSuites: {}\nComponents: {}",
            self.category.as_str(),
            self.url,
            self.distribution,
            self.components.join(" ").trim_end()
        )
    }

    /// Performs fields formatting for Cydia plist key
    #[must_use]
    #[cfg(feature = "ios")]
    pub fn to_cydia_key(&self) -> String {
        format!(
            "{}:{}:{}",
            self.category.as_str(),
            self.url,
            self.distribution
        )
    }

    /// Performs constructing repo to Apple's plist format
    #[must_use]
    #[cfg(feature = "with-serde")]
    pub fn to_dict(&self) -> plist::Dictionary {
        let mut dict = plist::Dictionary::new();
        dict.insert(
            "Distribution".to_owned(),
            plist::Value::String(self.distribution.clone()),
        );
        dict.insert("URI".to_owned(), plist::Value::String(self.url.clone()));
        dict.insert(
            "Type".to_owned(),
            plist::Value::String(self.category.as_str().to_owned()),
        );
        dict.insert(
            "Sections".to_owned(),
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

#[cfg(test)]
mod tests {
    use super::Repository;
    use crate::{parser::Parser, Result};
    use std::{
        collections::HashMap,
        fs::File,
        io::{BufRead, BufReader},
    };

    #[tokio::test]
    async fn modern_repository() -> Result<()> {
        let database = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/sources_db/modern");

        let parser = Parser::new(database)?;

        let repositories = parser.parse::<Repository>().await;
        let repositories: HashMap<String, Repository> = repositories
            .into_iter()
            .map(|repo| (repo.url.clone(), repo))
            .collect();

        assert_eq!(repositories.len(), 3);

        let repo = repositories.get("https://apt1.example.com/").unwrap();
        assert_eq!(repo.components.as_slice(), &["main", "orig"]);

        Ok(())
    }

    #[test]
    fn classic_repository() -> Result<()> {
        let database = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/sources_db/classic");
        let reader = BufReader::new(File::open(database)?);

        let lines = reader.lines().flatten();
        let repositories: HashMap<String, Repository> = lines
            .map(|line| {
                Repository::from_one_line(line.as_str())
                    .map(|repo| (repo.url.clone(), repo))
                    .unwrap()
            })
            .collect();

        assert_eq!(repositories.len(), 3);

        let repo = repositories.get("https://apt1.example.com/").unwrap();
        assert_eq!(repo.distribution.as_str(), "stable");
        assert_eq!(repo.components.as_slice(), &["main", "orig"]);

        Ok(())
    }
}
