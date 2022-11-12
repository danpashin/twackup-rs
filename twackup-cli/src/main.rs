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
#![deny(rust_2018_idioms, clippy::pedantic)]
#![warn(
    clippy::single_match_else,
    clippy::linkedlist,
    clippy::unused_self,
    clippy::cast_possible_wrap
)]

mod commands;
mod context;
mod error;
mod logger;
mod process;
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

#[derive(Parser)]
#[clap(about, version)]
pub(crate) struct Options {
    #[clap(subcommand)]
    subcmd: Command,
}

#[tokio::main]
async fn main() {
    logger::Logger::init();
    if let Err(error) = _run().await {
        log::error!("{}", error);
    }
}

/// Starts parsing CLI arguments and runs actions for them
async fn _run() -> Result<()> {
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
