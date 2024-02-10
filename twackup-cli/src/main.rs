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

#![deny(
    rust_2018_idioms,
    clippy::pedantic,
    unreachable_pub,
    clippy::string_lit_as_bytes,
    clippy::deref_by_slicing,
    clippy::filetype_is_file,
    clippy::let_underscore_must_use,
    clippy::redundant_feature_names
)]
#![warn(clippy::single_match_else, clippy::unused_self)]
#![allow(clippy::struct_excessive_bools)]

mod commands;
mod error;
mod logger;
mod paths;
mod process;
mod progress_bar;
mod serializer;

use clap::Parser;
use commands::{CliCommand, Command};
use error::Result;
use std::fs;
use std::time::Instant;

const fn long_version_message() -> &'static str {
    concat!(
        env!("CARGO_PKG_VERSION"),
        " (Debug: ",
        env!("VERGEN_CARGO_DEBUG"),
        ")",
        "\n\nBuild on ",
        env!("VERGEN_BUILD_TIMESTAMP"),
        "\nGit commit: ",
        env!("VERGEN_GIT_DESCRIBE"),
    )
}

#[derive(Parser)]
#[clap(about, version, long_version = long_version_message())]
pub(crate) struct Options {
    #[clap(subcommand)]
    sub_cmd: Command,
}

#[tokio::main]
async fn main() {
    logger::Logger::init();

    let start_time = Instant::now();
    match _run().await {
        Ok(()) => log::info!(
            "command performed in {}",
            indicatif::HumanDuration(start_time.elapsed())
        ),
        Err(error) => log::error!("{}", error),
    }
}

/// Starts parsing CLI arguments and runs actions for them
async fn _run() -> Result<()> {
    let options = Options::parse();
    match options.sub_cmd {
        Command::List(cmd) => cmd.run().await,
        Command::Leaves(cmd) => cmd.run().await,
        Command::Build(cmd) => cmd.run().await,

        #[cfg(feature = "ios")]
        Command::Export(cmd) => cmd.run().await,

        #[cfg(feature = "ios")]
        Command::Import(cmd) => cmd.run().await,

        Command::ShowLicense => {
            let license = fs::read_to_string(paths::LICENSE_PATH)?;
            eprintln!("\n{license}\n");

            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Options;
    use clap::CommandFactory;

    #[test]
    fn cli_generic() {
        Options::command().debug_assert();
    }
}
