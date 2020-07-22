use clap::Clap;
use ansi_term::{Colour, Color};
use num_cpus;
use threadpool::ThreadPool;
use std::{
    sync::{Arc, Mutex},
    vec::Vec,
    collections::LinkedList,
    fs,
    process::exit,
};

mod cli_error;
mod parser;
mod package;
mod builder;
use crate::{package::*, parser::*, builder::*};


const ADMIN_DIR: &str = "/var/lib/dpkg";

/// Simple utility that helps you to rebuild all your packages to DEB's
#[derive(Clap)]
#[clap(version = env!("CARGO_PKG_VERSION"))]
struct CLIOptions {
    #[clap(subcommand)]
    subcmd: CLICommand,
}

#[derive(Clap)]
enum CLICommand {
    /// This command prints installed (or all) packages to stdout
    List(ListCommand),
    /// This command prints only unique packages from installed
    Leaves(LeavesCommand),
    Build(BuildCommand)
}

#[derive(Clap)]
struct ListCommand {
    /// Lists all found packages instead of installed only
    #[clap(short, long)]
    all: bool,

    /// Use custom dpkg <directory> instead of default
    #[clap(long, default_value=ADMIN_DIR)]
    admindir: String,
}

#[derive(Clap)]
struct BuildCommand {
    /// Builds all found packages instead of installed only
    #[clap(short, long)]
    all: bool,

    /// Use custom dpkg <directory> instead of default
    #[clap(long, default_value=ADMIN_DIR)]
    admindir: String,

    #[clap(long, default_value="/var/mobile/Documents/twackup")]
    destination: String,
}

#[derive(Clap)]
struct LeavesCommand {
    /// Use custom dpkg <directory> instead of default
    #[clap(long, default_value=ADMIN_DIR)]
    admindir: String,
}


fn main() {
    let options = CLIOptions::parse();
    match options.subcmd {
        CLICommand::List(cmd) => cmd.list(),
        CLICommand::Leaves(cmd) => cmd.list(),
        CLICommand::Build(cmd) => cmd.build(),
        _ => eprintln!("This feature is not implemented yet :(")
    }
}
fn section_color(section: &String)-> Colour {
    match section.to_lowercase().as_str() {
        "system" => Color::Red,
        "tweaks" => Color::Yellow,
        "utilities" => Color::Green,
        "themes" => Color::Cyan,
        "terminal_support" => Color::Green,
        _ => Color::White
    }
}

fn get_packages(admin_dir: &String, get_all: bool) -> Vec<Package> {
    let status_file = format!("{}/status", admin_dir);
    let parser = Parser::new(status_file.as_str())
        .unwrap_or_else(|error| {
            eprintln!("Failed to open {}. {}", status_file, error);
            exit(1);
        });

    let pkgs: LinkedList<Package> = LinkedList::new();
    let packages = Arc::new(Mutex::new(pkgs));

    let handler_pkgs = Arc::clone(&packages);
    parser.parse(move |pkg| -> () {
        if !get_all {
            let identifier = &pkg.identifier;
            if identifier.starts_with("gsc")
                || identifier.starts_with("cy+")
                || pkg.state != State::Install {
                return;
            }
        }

        let mut pkgs = handler_pkgs.lock().unwrap();
        pkgs.push_back(pkg);
    });

    let packages_vec: Vec<Package> = packages.lock().unwrap().to_owned().into_iter().collect();
    return packages_vec;
}

impl ListCommand {
    fn list(&self) {
        let mut packages = get_packages(&self.admindir, self.all);
        packages.sort_by(|a, b| {
            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        });

        let mut counter = 0;
        for package in packages.iter() {
            counter += 1;
            let section_sym = section_color(&package.section).paint("▶︎");
            println!("{:3}: {} {} - {}",
                     counter, section_sym, package.name, package.identifier);
        }
    }
}

impl LeavesCommand {
    fn list(&self) {
        let mut packages = get_packages(&self.admindir, false);
        packages.sort_by(|a, b| {
            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        });

        let mut counter = 0;
        for package in packages.iter() {
            let mut is_dependency = false;
            for pkg in packages.iter() {
                is_dependency = pkg.depends.contains(&package.identifier);
                is_dependency = is_dependency || pkg.predepends.contains(&package.identifier);
                if is_dependency {
                    break;
                }
            }

            if !is_dependency {
                counter += 1;
                let section_sym = section_color(&package.section).paint("▶︎");
                println!("{:3}: {} {} - {}",
                         counter, section_sym, package.name, package.identifier);
            }
        }
    }
}

impl BuildCommand {
    fn build(&self) {
        self.create_dir_if_needed();
        let packages = get_packages(&self.admindir, self.all);
        let threadpool = ThreadPool::new(num_cpus::get());

        for package in packages {
            let builder = BuildWorker::new(&self.admindir, &package, &self.destination);
            threadpool.execute(move || {
                if let Err(error) = builder.run() {
                    eprintln!("Error while building {}. {}", builder.package.name, error);
                }
            });
        }

        threadpool.join();
    }

    fn create_dir_if_needed(&self) {
        if let Ok(metadata) = fs::metadata(&self.destination) {
            if !metadata.is_dir() {
                if let Err(error) = fs::remove_file(&self.destination) {
                    eprintln!("Failed to remove {}. {}", self.destination, error);
                    exit(1);
                }
            }

            return;
        }

        if let Err(error) = fs::create_dir_all(&self.destination) {
            eprintln!("Failed to create {}. {}", self.destination, error);
            exit(1);
        }
    }

}
