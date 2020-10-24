use clap::Clap;

mod list;
mod leaves;
mod build;
mod utils;

#[cfg(any(target_os = "ios", debug_assertions))]
mod backup;

const ADMIN_DIR: &'static str = "/var/lib/dpkg";
const TARGET_DIR: &'static str = "/var/mobile/Documents/twackup";

trait CLICommand {
    fn run(&self);
}

#[derive(Clap)]
#[clap(about, version)]
struct Options {
    #[clap(subcommand)]
    subcmd: Command,
}

#[derive(Clap)]
#[clap(version)]
enum Command {
    /// Just prints installed packages to stdout.
    /// Skips "virtual" packages mostly used by all iOS package managers.
    List(list::List),

    /// Detects packages that are not dependencies of others and prints them to stdout
    ///
    /// If you know homebrew, you should know similar command. This does the same thing.
    ///
    Leaves(leaves::Leaves),

    /// Collects package from files in the filesystem and packages them to DEB.
    /// Skips "virtual" packages mostly used by all iOS package managers.
    ///
    /// Note, this command can fail in finding some files
    /// (e.g. when they were moved by post-installation or another script),
    /// so it can't be used for "backing up" all packages you have.
    /// For backups, please, use export and import commands.
    Build(build::Build),

    /// Exports packages and repositories to file.
    /// Skips "virtual" packages mostly used by all iOS package managers.
    ///
    /// Can be used for backing up data for to restore in another jailbreak
    /// or after restoring system itself.
    #[cfg(any(target_os = "ios", debug_assertions))]
    Export(backup::export::Export),

    /// Performs importing packages and repositories from file created by export command.
    ///
    /// Useful when you want to restore packages from your old device or another jailbreak.
    #[cfg(any(target_os = "ios", debug_assertions))]
    Import(backup::import::Import),
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
