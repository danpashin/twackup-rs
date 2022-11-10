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

use super::Context;
use crate::error::Result;

#[async_trait::async_trait]
pub(crate) trait CliCommand {
    async fn run(&self, context: Context) -> Result<()>;
}

#[derive(clap::Parser)]
#[clap(version)]
pub(crate) enum Command {
    /// Just prints installed packages to stdout.
    /// Skips "virtual" packages mostly used by all iOS package managers.
    List(list::List),

    /// Detects packages that are not dependencies of others and prints them to stdout
    ///
    /// If you know homebrew, you should know similar command. This does the same thing.
    ///
    Leaves(leaves::Leaves),

    /// Collects package from files in the filesystem and packages them to DEB.
    /// Skips "virtual" packages mostly used by all iOS package managers.
    ///
    /// Note, this command can fail in finding some files
    /// (e.g. when they were moved by post-installation or another script),
    /// so it can't be used for "backing up" all packages you have.
    /// For backups, please, use export and import commands.
    Build(build::Build),

    /// Exports packages and repositories to file.
    /// Skips "virtual" packages mostly used by all iOS package managers.
    ///
    /// Can be used for backing up data for to restore in another jailbreak
    /// or after restoring system itself.
    #[cfg(feature = "ios")]
    Export(backup::export::Export),

    /// Performs importing packages and repositories from file created by export command.
    ///
    /// Useful when you want to restore packages from your old device or another jailbreak.
    #[cfg(feature = "ios")]
    Import(backup::import::Import),

    /// Shows license under Twackup is being distributed
    #[clap(aliases = &["w", "c"])]
    ShowLicense,
}
