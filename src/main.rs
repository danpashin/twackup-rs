use clap::Clap;
use ansi_term::Colour;
use threadpool::ThreadPool;
use gethostname::gethostname;
use chrono::Local;
use std::{
    sync::{Arc, Mutex},
    vec::Vec,
    collections::LinkedList,
    fs, io,
    time::Instant,
    path::{Path, PathBuf},
};

mod parser;
mod package;
mod builder;
use crate::{package::*, parser::*, builder::*};

#[cfg(test)]
mod tests;

const ADMIN_DIR: &str = "/var/lib/dpkg";
const TARGET_DIR: &str = "/var/mobile/Documents/twackup";
const DEFAULT_ARCHIVE_NAME: &str = "%host%_%date%.tar.gz";

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
struct CLIOptions {
    #[clap(subcommand)]
    subcmd: CLICommand,
}

#[derive(Clap)]
#[clap(version)]
enum CLICommand {
    /// Prints installed packages to stdout
    List(ListCommand),
    /// Prints packages that are not dependencies of others to stdout
    Leaves(LeavesCommand),
    /// Creates DEB from the already installed package(s)
    Build(BuildCommand)
}

#[derive(Clap)]
#[clap(version)]
struct ListCommand {
    /// Use custom dpkg <directory>
    #[clap(long, default_value=ADMIN_DIR, parse(from_os_str))]
    admindir: PathBuf,
}

#[derive(Clap)]
#[clap(version)]
struct LeavesCommand {
    /// Use custom dpkg <directory>
    #[clap(long, default_value=ADMIN_DIR, parse(from_os_str))]
    admindir: PathBuf,
}

#[derive(Clap)]
#[clap(version, after_help="
Beware, this command doesn't guarantee to copy all files to the final DEB! \
Some files can be skipped because of being renamed or removed in the installation process.
If you see yellow warnings, it means the final deb will miss some contents \
and may not work properly anymore.
")]
struct BuildCommand {
    /// By default twackup rebuilds only that packages which are not dependencies of others.
    /// This flag disables this restriction - command will rebuild all found packages.
    #[clap(short, long)]
    all: bool,

    /// Use custom dpkg <directory>
    #[clap(long, default_value=ADMIN_DIR, parse(from_os_str))]
    admindir: PathBuf,

    /// Package identifier or number from the list command.
    /// This argument can have multiple values separated by space ' '.
    packages: Vec<String>,

    /// Use custom destination <directory>.
    #[clap(long, short, default_value=TARGET_DIR, parse(from_os_str))]
    destination: PathBuf,

    /// Packs all rebuilded DEB's to single archive
    #[clap(long)]
    archive: bool,

    /// Name of archive if --archive is set. Supports only .tar.gz archives for now.
    #[clap(long, default_value=DEFAULT_ARCHIVE_NAME)]
    archive_name: String,

    /// Removes all DEB's after adding to archive. Makes sense only if --archive is set.
    #[clap(short="R", long)]
    remove_after: bool,
}


fn main() {
    let options = CLIOptions::parse();
    match options.subcmd {
        CLICommand::List(cmd) => cmd.list(),
        CLICommand::Leaves(cmd) => cmd.list(),
        CLICommand::Build(cmd) => cmd.run(),
    }
}

fn section_color(section: &Section)-> Colour {
    match section {
        Section::System => Colour::Fixed(9), // bright red
        Section::Tweaks => Colour::Fixed(11), // bright yellow
        Section::Utilities | Section::Packaging => Colour::Fixed(14), // bright cyan
        Section::Development => Colour::Fixed(130), // more like orange with pink
        Section::Themes => Colour::Fixed(12), // bright blue
        Section::TerminalSupport => Colour::Fixed(10), // bright green
        Section::Networking => Colour::Fixed(112), // bright green with some cyan
        Section::Archiving => Colour::Fixed(216),  // peach?
        Section::TextEditors => Colour::Fixed(162), // between red and magenta. Raspberry?
        _ => Colour::Fixed(8) // bright grey
    }
}

fn get_packages(admin_dir: &PathBuf, leaves_only: bool) -> Vec<Arc<Package>> {
    let status_file = admin_dir.join("status");
    let parser = Parser::new(status_file.as_path()).expect("Failed to open database");

    let pkgs: LinkedList<Arc<Package>> = LinkedList::new();
    let packages = Arc::new(Mutex::new(pkgs));

    // Create mutexed pointer for multithreading parser
    let handler_pkgs = Arc::clone(&packages);
    parser.parse(move |pkg| -> () {
        let identifier = &pkg.identifier;
        // Ignore all virtual dependencies for Cydia and all uninstalled packages
        if identifier.starts_with("gsc") || identifier.starts_with("cy+")
            || pkg.state != State::Install
        { return; }

        let mut pkgs = handler_pkgs.lock().unwrap();
        pkgs.push_back(Arc::new(pkg));
    });

    let unfiltered = packages.lock().unwrap();

    let mut filtered: Vec<Arc<Package>> = Vec::with_capacity(unfiltered.len());
    for package in unfiltered.iter() {
        if leaves_only {
            // Drop this package if it is the dependency of other
            let mut is_dependency = false;
            for pkg in unfiltered.iter() {
                if package.is_dependency_of(pkg) {
                    is_dependency = true;
                    break;
                }
            }
            if !is_dependency {
                filtered.push(Arc::clone(package));
            }
        } else {
            filtered.push(Arc::clone(package));
        }
    }

    return filtered;
}

impl ListCommand {
    fn list(&self) {
        let mut packages = get_packages(&self.admindir, false);
        packages.sort_by(|a, b| {
            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        });

        let mut counter: usize = 0;
        for package in packages.iter() {
            counter += 1;
            let section_sym = section_color(&package.section).paint("▶︎");
            println!("{:3}: {} {} - {}", counter, section_sym, package.name, package.identifier);
        }
    }
}

impl LeavesCommand {
    fn list(&self) {
        let mut packages = get_packages(&self.admindir, true);
        packages.sort_by(|a, b| {
            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        });

        for package in packages.iter() {
            let section_sym = section_color(&package.section).paint("▶︎");
            println!("{} {} - {}", section_sym, package.name, package.identifier);
        }
    }
}

impl BuildCommand {
    fn run(&self) {
        if !self.packages.is_empty() {
            self.build_user_specified();
        } else if !self.all {
            eprint!("No packages specified. Rebuild all? [Y/N] [default N] ");

            let mut buffer = String::new();
            let _ = io::stdin().read_line(&mut buffer);
            if buffer.trim().to_lowercase() == "y" {
                self.build(get_packages(&self.admindir, true));
            } else {
                eprintln!("Ok, cancelling...");
            }
        } else {
            self.build(get_packages(&self.admindir, false));
        }
    }

    fn build_user_specified(&self) {
        let mut all_packages = get_packages(&self.admindir, false);
        all_packages.sort_by(|a, b| {
            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        });

        let mut to_build: Vec<Arc<Package>> = Vec::with_capacity(self.packages.len());

        for package_id in self.packages.iter() {
            if let Ok(human_pos) = package_id.parse::<usize>() {
                let position = human_pos - 1;
                match all_packages.iter().skip(position).next() {
                    Some(pkg) => to_build.push(pkg.clone()),
                    None => {
                        match all_packages.iter().find(|pkg| pkg.identifier == *package_id) {
                            Some(pkg)=> to_build.push(pkg.clone()),
                            None => eprintln!("Can't find any package with name or index {}", package_id)
                        }
                    }
                }
            } else {
                match all_packages.iter().find(|pkg| pkg.identifier == *package_id) {
                    Some(pkg)=> to_build.push(pkg.clone()),
                    None => eprintln!("Can't find any package with name or index {}", package_id)
                }
            }
        }

        self.build(to_build);
    }

    fn build(&self, packages: Vec<Arc<Package>>) {
        let started = Instant::now();
        self.create_dir_if_needed();
        let threadpool = ThreadPool::default();

        let all_count = packages.len();
        let pb = indicatif::ProgressBar::new(all_count as u64);
        pb.set_style(
            indicatif::ProgressStyle::default_bar()
            .template("{pos}/{len} [{wide_bar:.cyan/blue}] {msg}")
            .progress_chars("##-")
        );
        let progress_bar = Arc::new(pb);

        if !nix::unistd::getuid().is_root() {
            progress_bar.println(Colour::Yellow.paint(
                "You seem not to be a root user. It is highly recommended to use root, \
                in other case building some packages can fail."
            ).to_string());
        }

        let archive = self.create_archive_if_needed();
        let archive_ptr = Arc::new(Mutex::new(archive));

        for package in packages.iter() {
            let builder = BuildWorker::new(
                &self.admindir, package, &self.destination, Arc::clone(&progress_bar)
            );
            let b_archive_ptr = Arc::clone(&archive_ptr);
            let perform_archive = self.archive;
            let remove_deb = self.remove_after;
            threadpool.execute(move || {
                builder.progress.set_message(
                    format!("Processing {}", builder.package.name).as_str()
                );
                let status = builder.run();
                builder.progress.inc(1);
                if let Err(error) = status {
                    builder.progress.println(Colour::Red.paint(
                        format!("Building {} error. {}", builder.package.name, error)
                    ).to_string());
                } else {
                    builder.progress.set_message(
                        format!("Done {}", builder.package.name).as_str()
                    );

                    if perform_archive {
                        let mut archive = b_archive_ptr.lock().unwrap();
                        let file = status.unwrap();
                        let abs_file = Path::new(".").join(file.file_name().unwrap());
                        let result = archive.as_mut().unwrap().get_mut()
                            .append_path_with_name(&file, &abs_file);
                        if result.is_ok() && remove_deb {
                            let _ = fs::remove_file(file);
                        }
                    }
                }
            });
        }

        threadpool.join();
        progress_bar.finish_and_clear();
        println!(
            "Processed {} packages in {}",
            all_count, indicatif::HumanDuration(started.elapsed())
        );
    }

    fn create_archive_if_needed(&self) -> Option<builder::deb::DebTarArchive> {
        if !self.archive {
            return None;
        }

        let filename = if self.archive_name == DEFAULT_ARCHIVE_NAME {
            format!("{}_{}.tar.gz", gethostname().to_str().unwrap(), Local::now().format("%v_%T"))
        } else {
            self.archive_name.clone()
        };

        let filepath = self.destination.join(&filename);
        let file = fs::File::create(filepath).expect("Can't open archive file");
        let compression = flate2::Compression::default();
        let encoder = flate2::write::GzEncoder::new(file, compression);

        return Some(builder::deb::TarArchive::new(encoder));
    }

    fn create_dir_if_needed(&self) {
        if let Ok(metadata) = fs::metadata(&self.destination) {
            if !metadata.is_dir() {
                fs::remove_file(&self.destination).expect("Failed to remove working dir");
            }

            return;
        }

        fs::create_dir_all(&self.destination).expect("Failed to create working dir");
    }
}
