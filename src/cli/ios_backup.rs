use std::{
    path::PathBuf,
    collections::{HashMap, LinkedList},
    io, fs::File,
};
use clap::Clap;
use serde::{Deserialize, Serialize};

use crate::{ADMIN_DIR, repository::Repository};
use super::{CLICommand, utils::{get_packages, get_repos}};

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

#[derive(Serialize, Deserialize)]
struct DataLayout {
    packages: LinkedList<String>,
    repositories: HashMap<String, Vec<Repository>>,
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
        let data = match self.data {
            ExportData::Packages => DataLayout {
                packages: self.get_packages(), repositories: HashMap::new()
            },
            ExportData::Repositories => DataLayout {
                packages: LinkedList::new(), repositories: get_repos()
            },
            ExportData::All => DataLayout {
                packages: self.get_packages(), repositories: get_repos()
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
}
