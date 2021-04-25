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

use std::path::PathBuf;

use super::{
    utils::{get_packages, section_color},
    CliCommand, ADMIN_DIR,
};

#[derive(clap::Parser)]
#[clap(version)]
pub struct List {
    /// Use custom dpkg <directory>.
    /// This option is used for detecting installed packages
    #[clap(long, default_value = ADMIN_DIR, parse(from_os_str))]
    admindir: PathBuf,
}

#[async_trait::async_trait]
impl CliCommand for List {
    async fn run(&self) {
        let mut packages = get_packages(&self.admindir, false).await;
        packages.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

        for (position, package) in packages.into_iter().enumerate() {
            let section_sym = section_color(&package.section).paint("▶︎");
            println!(
                "{:3}: {} {} - {}",
                position + 1,
                section_sym,
                package.name,
                package.id
            );
        }
    }
}
