pub mod cli_error;
pub mod parser;
use parser::*;
use clap::Clap;
use std::{
    sync::{Arc, Mutex},
    vec::Vec
};
use crate::parser::package::*;

const DPKG_STATUS_FILE: &str = "/var/lib/dpkg/status";

#[derive(Clap)]
struct CLIOptions {
    #[clap(subcommand)]
    subcmd: CLICommand,
}

#[derive(Clap)]
enum CLICommand {
    List
}

fn main() {
    let options = CLIOptions::parse();
    match options.subcmd {
        CLICommand::List => list_packages(DPKG_STATUS_FILE),
    }
}

fn list_packages(file: &str) {
    let mut packages = get_actual_packages(file);
    packages.sort_by(|a, b| {
        a.name.to_lowercase().cmp(&b.name.to_lowercase())
    });

    let mut counter = 0;
    for package in packages.iter() {
        counter += 1;
        println!("{:3}: {} - {}", counter, package.name, package.identifier);
    }
}

fn get_actual_packages(file: &str) -> Vec<Package> {
    let parser = Parser::new(file)
        .unwrap_or_else(|error| panic!("Failed to open {}. {}", file, error));

    let pkgs: Vec<Package> = Vec::new();
    let packages = Arc::new(Mutex::new(pkgs));

    let handler_pkgs = Arc::clone(&packages);
    parser.parse(move |pkg| -> () {
        let identifier = &pkg.identifier;
        if identifier.starts_with("gsc") || identifier.starts_with("cy+") || pkg.state != State::Install {
            return;
        }

        let mut pkgs = handler_pkgs.lock().unwrap();
        pkgs.push(pkg);
    });

    return packages.lock().unwrap().to_owned();
}
