
mod kvparser;
mod package;
mod repository;
mod builder;
mod cli;
mod process;

#[cfg(test)]
mod tests;

const ADMIN_DIR: &'static str = "/var/lib/dpkg";
const TARGET_DIR: &'static str = "/var/mobile/Documents/twackup";

fn main() {
    cli::run();
}
