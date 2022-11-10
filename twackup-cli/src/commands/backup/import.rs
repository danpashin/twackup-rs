/*
 * Copyright 2020 DanP
 *
 * This file is part of Twackup
 *
 * Twackup is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Twackup is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Twackup. If not, see <http://www.gnu.org/licenses/>.
 */

use super::{DataLayout, RepoGroup, RepoGroupFormat};
use crate::{commands::CliCommand, context::Context, error::Result, serde::Format};
use std::{
    fs::File,
    io::{self, BufWriter, Write},
    process::{Command, Stdio},
};
use twackup::{error::GenericError, process};

#[derive(clap::Parser)]
pub(crate) struct Import {
    /// Use another input format
    /// (e.g. when it was processed with third-party parser like jq)
    #[clap(short, long, value_enum, default_value = "json")]
    format: Format,

    /// Input file, stdin if equal to '-'
    #[clap(name = "file")]
    input: String,
}

#[async_trait::async_trait]
impl CliCommand for Import {
    async fn run(&self, context: Context) -> Result<()> {
        if !context.is_root() {
            Err(GenericError::NotRunningAsRoot)?;
        }

        let data = self.deserialize_input().expect("Can't deserialize input");

        if let Some(repositories) = data.repositories {
            for repo_group in repositories.iter() {
                if let Err(error) = self.import_repo_group(repo_group) {
                    eprint!(
                        "Can't import sources for {}. {:?}",
                        repo_group.executable, error
                    );
                }
            }

            let executables = repositories.iter().map(|src| src.executable.as_str());
            process::send_signal_to_multiple(executables, process::Signal::Kill);
        }

        if let Some(packages) = data.packages {
            log::info!("Importing packages...");
            self.run_apt(vec![
                "update",
                "--allow-unauthenticated",
                "--allow-insecure-repositories",
                "-o",
                "Acquire::Languages=none",
            ])
            .expect("Failed to run update subcommand");

            let mut install_args =
                vec!["install", "-y", "--allow-unauthenticated", "--fix-missing"];
            install_args.extend(packages.iter().map(|pkg| pkg.as_str()));
            self.run_apt(install_args)
                .expect("Failed to run install subcommand");
        }

        Ok(())
    }
}

impl Import {
    #[inline]
    fn deserialize_input(&self) -> Result<DataLayout> {
        Ok(match self.input.as_str() {
            "-" => self.format.de_from_reader(io::stdin())?,
            _ => self.format.de_from_reader(File::open(&self.input)?)?,
        })
    }

    fn import_repo_group(&self, repo_group: &RepoGroup) -> Result<()> {
        log::info!(
            "Importing {} source(s) for {}",
            repo_group.sources.len(),
            repo_group.executable
        );
        let mut writer = BufWriter::new(File::create(&repo_group.path)?);
        for source in repo_group.sources.iter() {
            match repo_group.format {
                RepoGroupFormat::Classic => writer.write(source.to_one_line().as_bytes())?,
                RepoGroupFormat::Modern => {
                    writer.write_all(source.to_deb822().as_bytes())?;
                    writer.write(b"\n")?
                }
            };
            writer.write_all(b"\n")?;
            writer.flush()?;
        }

        log::info!(
            "Triggering post-import hooks for {}...",
            repo_group.executable
        );
        let hook_res: Result<_> = match repo_group.executable.as_str() {
            "Zebra" => std::fs::remove_file(
                "/var/mobile/Library/Application Support/xyz.willy.Zebra/zebra.db",
            )
            .map_err(|error| error.into()),
            "Cydia" => self.cydia_post_import_hook(repo_group),
            _ => Ok(()),
        };

        if let Err(error) = hook_res {
            eprintln!("Post-import hook completed with error: {:?}", error);
        }

        Ok(())
    }

    fn cydia_post_import_hook(&self, repo_group: &RepoGroup) -> Result<()> {
        let prefs_path = "/var/mobile/Library/Preferences/com.saurik.Cydia.plist";
        let mut prefs = plist::Value::from_file(prefs_path)?;

        let prefs_dict = prefs
            .as_dictionary_mut()
            .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidInput))?;

        let sources: plist::Dictionary = repo_group
            .sources
            .iter()
            .map(|src| (src.to_cydia_key(), plist::Value::Dictionary(src.to_dict())))
            .collect();

        prefs_dict.insert(
            "CydiaSources".to_string(),
            plist::Value::Dictionary(sources),
        );

        Ok(prefs.to_file_binary(prefs_path)?)
    }

    fn run_apt(&self, args: Vec<&str>) -> Option<()> {
        let apt_cmd = Command::new("apt")
            .args(args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .expect("apt command failed to start");

        if !apt_cmd.success() {
            log::error!(
                "Apt exited with status: {}. See stderr for more info.",
                apt_cmd
            );
            return None;
        }

        Some(())
    }
}
