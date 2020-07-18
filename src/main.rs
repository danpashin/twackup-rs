pub mod cli_error;
pub mod parser;
use parser::*;

const DPKG_STATUS_FILE: &str = "/var/lib/dpkg/status";

fn main() {
    let parser = Parser::new(DPKG_STATUS_FILE)
        .unwrap_or_else(|error| panic!("Failed to open {}. {}", DPKG_STATUS_FILE, error));
    parser.parse(|pkg| -> () {
        // println!("{} - {}", pkg.name, pkg.identifier);
    });
}
