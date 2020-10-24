
mod kvparser;
mod package;
mod repository;
mod builder;
mod cli;
mod process;

#[cfg(test)]
mod tests;

fn main() {
    cli::run();
}
