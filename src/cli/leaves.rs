use clap::Clap;
use std::path::PathBuf;

use crate::ADMIN_DIR;
use super::{CLICommand, utils::{get_packages, section_color}};

#[derive(Clap)]
pub struct Leaves {
    /// Use custom dpkg <directory>
    #[clap(long, default_value=ADMIN_DIR, parse(from_os_str))]
    admindir: PathBuf,
}

impl CLICommand for Leaves {
    fn run(&self) {
        let mut packages = get_packages(&self.admindir, true);
        packages.sort_by(|a, b| {
            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        });

        for package in packages.iter() {
            let section_sym = section_color(&package.section).paint("▶︎");
            println!("{} {} - {}", section_sym, package.name, package.identifier);
        }
    }
}
