use clap::Clap;
use std::path::PathBuf;

use crate::ADMIN_DIR;
use super::{CLICommand, utils::{get_packages, section_color}};

#[derive(Clap)]
#[clap(version)]
pub struct List {
    /// Use custom dpkg <directory>.
    /// This option is used for detecting installed packages
    #[clap(long, default_value=ADMIN_DIR, parse(from_os_str))]
    admindir: PathBuf,
}

impl CLICommand for List {
    fn run(&self) {
        let mut packages = get_packages(&self.admindir, false);
        packages.sort_by(|a, b| {
            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        });

        let mut counter: usize = 0;
        for package in packages.iter() {
            counter += 1;
            let section_sym = section_color(&package.section).paint("▶︎");
            println!("{:3}: {} {} - {}", counter, section_sym, package.name, package.identifier);
        }
    }
}
