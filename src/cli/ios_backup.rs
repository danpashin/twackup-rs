use std::{
    path::PathBuf,
    collections::{HashMap, LinkedList},
    io, fs::File,
};
use clap::Clap;

use crate::ADMIN_DIR;
use super::{CLICommand, utils::get_packages};

#[derive(Clap, PartialEq)]
enum ExportFormat {
    Json,
    Toml,
    Yaml,
}

#[derive(Clap, PartialEq)]
enum ExportData {
    Packages,
    Repositories,
    All,
}

#[derive(Clap)]
pub struct Export {
    /// Use custom dpkg <directory>
    #[clap(long, default_value=ADMIN_DIR, parse(from_os_str))]
    admindir: PathBuf,

    /// Use custom output format
    #[clap(long, arg_enum, default_value="json")]
    format: ExportFormat,

    /// Data to export
    #[clap(long, arg_enum, default_value="all")]
    data: ExportData,

    /// Output file, stdout if not present
    #[clap(short, long)]
    output: Option<PathBuf>,
}

impl ExportFormat {
    fn as_str(&self) -> &str {
        match self {
            Self::Json => "json",
            Self::Toml => "toml",
            Self::Yaml => "yaml",
        }
    }
}

impl CLICommand for Export {
    fn run(&self) {
        let mut data = HashMap::new();
        match self.data {
            ExportData::Packages => data.insert("packages", self.get_packages()),
            ExportData::Repositories => data.insert("repositories", self.get_repositories()),
            ExportData::All => {
                data.insert("packages", self.get_packages());
                data.insert("repositories", self.get_repositories())
            }
        };

        let format = serde_any::guess_format_from_extension(self.format.as_str())
            .expect("Unsupported format");

        if let Some(path) = &self.output {
            let file = File::create(path).expect("Can't open fd for writing");
            serde_any::to_writer(file, &data, format).unwrap();
        } else {
            serde_any::to_writer(io::stdout(), &data, format).unwrap();
            println!();
        }
    }
}

impl Export {
    fn get_packages(&self) -> LinkedList<String> {
        get_packages(&self.admindir, true).iter().map(|pkg| {
            pkg.identifier.clone()
        }).collect()
    }

    fn get_repositories(&self) -> LinkedList<String> {
        LinkedList::new()
    }
}
