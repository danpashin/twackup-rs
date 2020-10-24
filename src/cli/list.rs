use clap::Clap;
use std::path::PathBuf;

use super::{
    ADMIN_DIR, CLICommand, utils::{get_packages, section_color}
};

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

        for (position, package) in packages.into_iter().enumerate() {
            let section_sym = section_color(&package.section).paint("▶︎");
            println!("{:3}: {} {} - {}", position + 1, section_sym, package.name, package.id);
        }
    }
}
