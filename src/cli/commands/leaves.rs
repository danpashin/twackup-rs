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

use super::{CliCommand, Context};
use crate::{cli::ADMIN_DIR, error::Result};
use std::path::PathBuf;

#[derive(clap::Parser)]
pub(crate) struct Leaves {
    /// Use custom dpkg <directory>.
    /// This option is used for detecting installed packages
    #[clap(long, default_value = ADMIN_DIR, value_parser)]
    admindir: PathBuf,
}

#[async_trait::async_trait]
impl CliCommand for Leaves {
    async fn run(&self, context: Context) -> Result<()> {
        let packages = context.packages(&self.admindir, true).await?;

        for (_, package) in packages {
            let section_sym = package.section.color().paint("▶︎");
            println!("{} {} - {}", section_sym, package.name, package.id);
        }

        Ok(())
    }
}
