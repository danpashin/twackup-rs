use std::{
    path::PathBuf,
    collections::LinkedList,
    io::{self, BufReader, BufRead, BufWriter, Write},
    fs::File,
};
use clap::Clap;
use serde::{Deserialize, Serialize};

use crate::{ADMIN_DIR, repository::Repository, kvparser::Parser, process};
use super::{CLICommand, utils::get_packages};

const MODERN_MANAGERS: &[(&str, &str)] = &[
    ("Sileo", "/etc/apt/sources.list.d/sileo.sources")
];

const CLASSIC_MANAGERS: &[(&str, &str)] = &[
    ("Cydia", "/var/mobile/Library/Caches/com.saurik.Cydia/sources.list"),
    ("Zebra", "/var/mobile/Library/Application Support/xyz.willy.Zebra/sources.list"),
];

#[derive(Clap, PartialEq)]
enum DataFormat {
    Json,
    Toml,
    Yaml,
}

#[derive(Clap, PartialEq)]
enum DataType {
    Packages,
    Repositories,
    All,
}

#[derive(PartialEq, Serialize, Deserialize)]
enum RepoGroupKind {
    Modern,
    Classic,
}

#[derive(Serialize, Deserialize)]
struct RepoGroup {
    kind: RepoGroupKind,
    path: String,
    executable: String,
    sources: LinkedList<Repository>,
}

#[derive(Serialize, Deserialize)]
struct DataLayout {
    packages: Option<LinkedList<String>>,
    repositories: Option<LinkedList<RepoGroup>>,
}

#[derive(Clap)]
pub struct Export {
    /// Use custom dpkg <directory>
    #[clap(long, default_value=ADMIN_DIR, parse(from_os_str))]
    admindir: PathBuf,

    /// Use custom output format
    #[clap(short, long, arg_enum, default_value="json")]
    format: DataFormat,

    /// Data to export
    #[clap(short, long, arg_enum, default_value="all")]
    data: DataType,

    /// Output file, stdout if not present
    #[clap(short, long)]
    output: Option<PathBuf>,
}

#[derive(Clap)]
pub struct Import {
    /// Use custom input format
    #[clap(short, long, arg_enum, default_value="json")]
    format: DataFormat,

    /// Input file, stdin if equal to '-'
    #[clap(name="file")]
    input: String,
}

impl DataFormat {
    fn as_str(&self) -> &str {
        match self {
            Self::Json => "json",
            Self::Toml => "toml",
            Self::Yaml => "yaml",
        }
    }

    fn to_serde(&self) -> serde_any::Format {
        match self {
            Self::Json => serde_any::Format::Json,
            Self::Toml => serde_any::Format::Toml,
            Self::Yaml => serde_any::Format::Yaml,
        }
    }
}

impl CLICommand for Export {
    fn run(&self) {
        let data = match self.data {
            DataType::Packages => DataLayout {
                packages: Some(self.get_packages()), repositories: None
            },
            DataType::Repositories => DataLayout {
                packages: None, repositories: Some(self.get_repos())
            },
            DataType::All => DataLayout {
                packages: Some(self.get_packages()), repositories: Some(self.get_repos())
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

    fn get_repos(&self) -> LinkedList<RepoGroup> {
        let mut sources = LinkedList::new();

        for (name, path) in MODERN_MANAGERS {
            if let Ok(parser) = Parser::new(path) {
                let repos = parser.parse::<Repository>().iter().map(|repo| {
                    repo.as_ref().clone()
                }).collect();
                sources.push_back(RepoGroup {
                    kind: RepoGroupKind::Modern, path: path.to_string(),
                    executable: name.to_string(), sources: repos
                });
            }
        }

        for (name, path) in CLASSIC_MANAGERS {
            if let Ok(file) = File::open(path) {
                let mut repos = LinkedList::new();
                for line in BufReader::new(file).lines() {
                    if let Ok(line) = line {
                        if let Some(repo) = Repository::from_one_line(line.as_str()) {
                            repos.push_back(repo);
                        }
                    }
                }
                sources.push_back(RepoGroup {
                    kind: RepoGroupKind::Classic, path: path.to_string(),
                    executable: name.to_string(), sources: repos
                });
            }
        }

        return sources;
    }
}

impl CLICommand for Import {
    fn run(&self) {
        let data = self.try_deserializing_file().expect("Can't deserialize file");

        if let Some(repositories) = data.repositories {
            for repo_group in repositories.iter() {
                if let Err(error) = self.import_repo_group(&repo_group) {
                    eprint!("Can't import sources for {}. {}", repo_group.executable, error);
                }
            }

            let executables = repositories.iter().map(|src| {
                src.executable.clone()
            }).collect();
            process::send_signal_to_multiple(executables, process::Signal::Kill);
        }
    }
}

impl Import {
    fn try_deserializing_file(&self) -> Result<DataLayout, serde_any::error::Error> {
        let format =  self.format.to_serde();
        match self.input.as_str() {
            "-" => serde_any::from_reader(io::stdin(), format),
            _ => serde_any::from_reader(File::open(&self.input)?, format)
        }
    }

    fn import_repo_group(&self, repo_group: &RepoGroup) -> io::Result<()> {
        let mut writer = BufWriter::new(File::create(&repo_group.path)?);
        for source in repo_group.sources.iter() {
            eprintln!("Importing {} for {}", source.url, repo_group.executable);
            match repo_group.kind {
                RepoGroupKind::Classic => writer.write(source.to_one_line().as_bytes())?,
                RepoGroupKind::Modern => {
                    writer.write(source.to_deb822().as_bytes())?;
                    writer.write(b"\n")?
                }
            };
            writer.write(b"\n")?;
            writer.flush()?;
        }

        eprintln!("Triggering post-import hooks for {}...", repo_group.executable);
        let hook_res = match repo_group.executable.as_str() {
            "Zebra" => std::fs::remove_file(
                "/var/mobile/Library/Application Support/xyz.willy.Zebra/zebra.db"
            ),
            "Cydia" =>  self.cydia_post_import_hook(repo_group),
            _ => Ok(())
        };
        if let Err(error) = hook_res {
            eprintln!("Post-import hook completed with error: {}", error);
        }

        Ok(())
    }

    fn cydia_post_import_hook(&self, repo_group: &RepoGroup) -> io::Result<()> {
        let prefs_path = "/var/mobile/Library/Preferences/com.saurik.Cydia.plist";
        let prefs = plist::Value::from_file(prefs_path);
        if let Err(_) = prefs {
            return Err(io::Error::from(io::ErrorKind::InvalidInput));
        }
        let mut prefs = prefs.unwrap();

        let prefs_dict = prefs.as_dictionary_mut();
        if let None = prefs_dict {
            return Err(io::Error::from(io::ErrorKind::InvalidInput));
        }

        let sources: plist::Dictionary = repo_group.sources.iter().map(|src| {
            (src.to_cydia_key(), plist::Value::Dictionary(src.to_dict()))
        }).collect();

        prefs_dict.unwrap().insert("CydiaSources".to_string(), plist::Value::Dictionary(sources));

        if let Err(_) = prefs.to_file_binary(prefs_path) {
            return Err(io::Error::from(io::ErrorKind::WriteZero));
        }

        Ok(())
    }
}
