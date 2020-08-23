use clap::Clap;

mod list;
mod leaves;
mod build;
mod utils;

#[cfg(any(target_os = "ios", debug_assertions))]
mod ios_backup;

trait CLICommand {
    fn run(&self);
}

#[derive(Clap)]
#[clap(about, version, after_help="
Hello there! This is twackup - the most advanced, safe and fast tool for rebuilding your tweaks \
back to DEB's.
But be careful! It doesn't download new DEB from somewhere, it passes through all system and \
collects all files it finds to a single DEB. Therefore it's highly recommended to run this tool \
as root - lower probability it couldn't open and/or copy some files.

All commands will never ever list or backup \"virtual\" packages - different dependencies which \
package managers use to define your OS version or device.
")]
struct Options {
    #[clap(subcommand)]
    subcmd: Command,
}

#[derive(Clap)]
#[clap(version)]
enum Command {
    /// Prints installed packages to stdout
    List(list::List),
    /// Detects packages that are not dependencies of others and prints them to stdout
    Leaves(leaves::Leaves),
    /// Creates DEB from the already installed package(s)
    Build(build::Build),

    /// Exports packages and repositories to backup file
    #[cfg(any(target_os = "ios", debug_assertions))]
    Export(ios_backup::Export),

    /// Performs importing packages and repositories from backup file
    #[cfg(any(target_os = "ios", debug_assertions))]
    Import(ios_backup::Import),
}

/// Starts parsing CLI arguments and runs actions for them
pub fn run() {
    let options = Options::parse();
    match options.subcmd {
        Command::List(cmd) => cmd.run(),
        Command::Leaves(cmd) => cmd.run(),
        Command::Build(cmd) => cmd.run(),

        #[cfg(any(target_os = "ios", debug_assertions))]
        Command::Export(cmd) => cmd.run(),

        #[cfg(any(target_os = "ios", debug_assertions))]
        Command::Import(cmd) => cmd.run(),
    }
}
