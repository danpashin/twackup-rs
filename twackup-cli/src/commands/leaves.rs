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

use super::CliCommand;
use crate::{commands::GlobalOptions, error::Result};
use twackup::PackagesSort;

#[derive(clap::Parser)]
pub(crate) struct Leaves {
    #[clap(flatten)]
    global_options: GlobalOptions,
}

#[async_trait::async_trait]
impl CliCommand for Leaves {
    async fn run(&self) -> Result<()> {
        let packages = self
            .global_options
            .packages(true, PackagesSort::Name)
            .await?;

        for (_, package) in packages {
            let section_sym = package.section.color().apply_to("▶︎");
            println!("{} {} - {}", section_sym, package.human_name(), package.id);
        }

        Ok(())
    }
}
