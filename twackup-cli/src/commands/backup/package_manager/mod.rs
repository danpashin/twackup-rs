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

use crate::{commands::backup::RepoGroup, Result};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};
use twackup::{repository::Repository, Parser};

pub(crate) mod cydia;
pub(crate) mod sileo;
pub(crate) mod zebra;

pub(crate) enum PackageManager {
    Cydia(cydia::Cydia),
    Zebra(zebra::Zebra),
    Sileo(sileo::Sileo),
}

impl PackageManager {
    #[inline]
    pub(crate) fn supported() -> &'static [Self] {
        const SUPPORTED: &[PackageManager] = &[
            PackageManager::Cydia(cydia::Cydia::new()),
            PackageManager::Zebra(zebra::Zebra::new()),
            PackageManager::Sileo(sileo::Sileo::new()),
        ];
        SUPPORTED
    }

    #[inline]
    pub(crate) async fn repositories(&self) -> Result<Vec<Repository>> {
        match self {
            PackageManager::Cydia(val) => val.parse_repositories(),
            PackageManager::Zebra(val) => val.parse_repositories(),
            PackageManager::Sileo(val) => val.parse_repositories().await,
        }
    }

    #[inline]
    pub(crate) fn post_import(&self, repo_group: &RepoGroup) -> Result<()> {
        match self {
            Self::Cydia(val) => val.post_import(repo_group),
            Self::Zebra(val) => val.post_import(repo_group),
            Self::Sileo(val) => val.post_import(repo_group),
        }
    }

    #[inline]
    pub(crate) fn is_modern(&self) -> bool {
        match self {
            Self::Cydia(_) | Self::Zebra(_) => false,
            Self::Sileo(_) => true,
        }
    }

    #[inline]
    pub(crate) fn name(&self) -> &str {
        match self {
            PackageManager::Cydia(val) => val.exec_name(),
            PackageManager::Zebra(val) => val.exec_name(),
            PackageManager::Sileo(val) => val.exec_name(),
        }
    }

    #[inline]
    pub(crate) fn sources_path(&self) -> PathBuf {
        match self {
            PackageManager::Cydia(val) => val.repos_file_path(),
            PackageManager::Zebra(val) => val.repos_file_path(),
            PackageManager::Sileo(val) => val.repos_file_path(),
        }
    }
}

pub(crate) trait Hooks {
    fn post_import(&self, repo_group: &RepoGroup) -> Result<()>;
}

pub(crate) trait PackageManagerDescription {
    fn exec_name(&self) -> &str;
    fn repos_file_path(&self) -> PathBuf;
}

pub(crate) trait OldStylePackageManager
where
    Self: PackageManagerDescription,
{
    fn parse_repositories(&self) -> Result<Vec<Repository>> {
        let file = File::open(self.repos_file_path())?;

        let mut repos = Vec::new();
        for line in BufReader::new(file).lines().flatten() {
            match Repository::from_one_line(line.as_str()) {
                Ok(repo) => repos.push(repo),
                Err(error) => log::warn!("[{}] {:?}", self.exec_name(), error),
            }
        }

        Ok(repos)
    }
}

#[async_trait::async_trait]
pub(crate) trait NewStylePackageManager
where
    Self: PackageManagerDescription,
{
    async fn parse_repositories(&self) -> Result<Vec<Repository>> {
        let parser = Parser::new(self.repos_file_path())?;
        Ok(parser.parse().await.into_iter().collect())
    }
}
