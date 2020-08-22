
mod kvparser;
mod package;
mod builder;
mod cli;

#[cfg(test)]
mod tests;

const ADMIN_DIR: &str = "/var/lib/dpkg";
const TARGET_DIR: &str = "/var/mobile/Documents/twackup";
const DEFAULT_ARCHIVE_NAME: &str = "%host%_%date%.tar.gz";

fn main() {
    cli::run();
}
