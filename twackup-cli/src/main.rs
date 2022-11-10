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

mod commands;
mod context;
mod error;
mod logger;
mod progress_bar;
mod serde;

use clap::Parser;
use commands::{CliCommand, Command};
use context::Context;
use error::Result;
use std::fs;

const ADMIN_DIR: &str = "/var/lib/dpkg";
const TARGET_DIR: &str = "/var/mobile/Documents/twackup";
const LICENSE_PATH: &str = "/usr/share/doc/ru.danpashin.twackup/LICENSE";

const ROOT_WARN_MESSAGE: &str =
    "You seem not to be a root user. It is highly recommended to use root, \
    in other case some operations can fail.";

#[derive(Parser)]
#[clap(about, version)]
pub(crate) struct Options {
    #[clap(subcommand)]
    subcmd: Command,
}

/// Starts parsing CLI arguments and runs actions for them
#[tokio::main]
async fn main() -> Result<()> {
    logger::Logger::init();
    let context = Context::new();

    let options = Options::parse();
    match options.subcmd {
        Command::List(cmd) => cmd.run(context).await,
        Command::Leaves(cmd) => cmd.run(context).await,
        Command::Build(cmd) => cmd.run(context).await,

        #[cfg(feature = "ios")]
        Command::Export(cmd) => cmd.run(context).await,

        #[cfg(feature = "ios")]
        Command::Import(cmd) => cmd.run(context).await,

        Command::ShowLicense => {
            let license = fs::read_to_string(LICENSE_PATH)?;
            println!("\n{}\n", license);

            Ok(())
        }
    }
}
