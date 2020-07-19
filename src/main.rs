use clap::Clap;
use ansi_term::{Colour, Color};
use std::{
    sync::{Arc, Mutex},
    vec::Vec
};

mod cli_error;
mod parser;
use crate::parser::{*, package::*};

const DPKG_STATUS_FILE: &str = "/var/lib/dpkg/status";

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
}

#[derive(Clap)]
struct ListCommand {
    /// Lists all found packages instead of installed only
    #[clap(short, long)]
    all: bool,

    /// Sets a custom database file
    #[clap(short, long, default_value=DPKG_STATUS_FILE)]
    database: String,
}

#[derive(Clap)]
struct BuildCommand {
    /// Sets a custom database file
    #[clap(short, long, default_value=DPKG_STATUS_FILE)]
    database: String,
}

#[derive(Clap)]
struct LeavesCommand {
    /// Sets a custom database file
    #[clap(short, long, default_value=DPKG_STATUS_FILE)]
    database: String,
}


fn main() {
    let options = CLIOptions::parse();
    match options.subcmd {
        CLICommand::List(cmd) => cmd.list(),
        CLICommand::Leaves(cmd) => cmd.list(),
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

fn get_packages(file: &str, get_all: bool) -> Vec<Package> {
    let parser = Parser::new(file)
        .unwrap_or_else(|error| panic!("Failed to open {}. {}", file, error));

    let pkgs: Vec<Package> = Vec::new();
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
        pkgs.push(pkg);
    });

    return packages.lock().unwrap().to_owned();
}

impl ListCommand {
    fn list(&self) {
        let mut packages = get_packages(self.database.as_str(), self.all);
        packages.sort_by(|a, b| {
            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        });

        let mut counter = 0;
        for package in packages.iter() {
            counter += 1;
            let name = section_color(&package.section).bold().paint(&package.name);
            println!("{:3}: {} - {}", counter, name, package.identifier);
        }
    }
}

impl LeavesCommand {
    fn list(&self) {
        let mut packages = get_packages(self.database.as_str(), false);
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
                let name = section_color(&package.section).bold().paint(&package.name);
                println!("{:3}: {} - {}", counter, name, package.identifier);
            }
        }
    }
}
