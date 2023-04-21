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

mod build;
mod leaves;
mod list;

#[cfg(feature = "ios")]
mod backup;

use crate::{error::Result, paths};
use std::{
    collections::{BTreeMap, LinkedList},
    path::PathBuf,
};
use twackup::{package::Package, Dpkg, PackagesSort};

#[async_trait::async_trait]
pub(crate) trait CliCommand {
    async fn run(&self) -> Result<()>;
}

#[derive(clap::Parser)]
struct GlobalOptions {
    /// Use custom dpkg directory.
    /// This option is used for detecting installed packages
    #[arg(long, default_value = paths::dpkg_admin_dir(), value_parser)]
    #[arg(help_heading = "GLOBAL OPTIONS", global = true)]
    admin_dir: PathBuf,
}

impl GlobalOptions {
    pub(crate) async fn packages(
        &self,
        leaves_only: bool,
        sort: PackagesSort,
    ) -> Result<BTreeMap<String, Package>> {
        let dpkg = Dpkg::new(&self.admin_dir, true);
        Ok(dpkg.packages(leaves_only, sort).await?)
    }

    pub(crate) async fn unsorted_packages(&self, leaves_only: bool) -> Result<LinkedList<Package>> {
        let dpkg = Dpkg::new(&self.admin_dir, true);
        Ok(dpkg.unsorted_packages(leaves_only).await?)
    }
}

#[derive(clap::Parser)]
#[clap(version)]
pub(crate) enum Command {
    /// Just prints installed packages to stdout.
    /// Skips "virtual" packages mostly used by all iOS package managers.
    #[clap(disable_version_flag = true)]
    List(list::List),

    /// Detects packages that are not dependencies of others and prints them to stdout
    ///
    /// If you know homebrew, you should know similar command. This does the same thing.
    ///
    #[clap(disable_version_flag = true)]
    Leaves(leaves::Leaves),

    /// Collects package from files in the filesystem and packages them to DEB.
    /// Skips "virtual" packages mostly used by all iOS package managers.
    ///
    /// Note, this command can fail in finding some files
    /// (e.g. when they were moved by post-installation or another script),
    /// so it can't be used for "backing up" all packages you have.
    /// For backups, please, use export and import commands.
    #[clap(disable_version_flag = true)]
    Build(build::Build),

    /// Exports packages and repositories to file.
    /// Skips "virtual" packages mostly used by all iOS package managers.
    ///
    /// Can be used for backing up data for to restore in another jailbreak
    /// or after restoring system itself.
    #[cfg(feature = "ios")]
    #[clap(disable_version_flag = true)]
    Export(backup::export::Export),

    /// Performs importing packages and repositories from file created by export command.
    ///
    /// Useful when you want to restore packages from your old device or another jailbreak.
    #[cfg(feature = "ios")]
    #[clap(disable_version_flag = true)]
    Import(backup::import::Import),

    /// Shows license under Twackup is being distributed
    #[clap(aliases = &["w", "c"])]
    ShowLicense,
}
