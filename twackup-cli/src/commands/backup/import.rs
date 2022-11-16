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

use super::{ExportData, RepoGroup, RepoGroupFormat};
use crate::{commands::CliCommand, error::Result, process, serializer::Format};
use libproc::libproc::proc_pid::am_root;
use std::{
    fs::File as StdFile,
    io,
    process::{Command, Stdio},
};
use tokio::fs::{self, File};
use tokio::io::{AsyncWriteExt, BufWriter};
use twackup::GenericError;

#[derive(clap::Parser)]
pub(crate) struct Import {
    /// Use another input format
    /// (e.g. when it was processed with third-party parser like jq)
    #[arg(short, long, value_enum, default_value = "json")]
    format: Format,

    /// Input file, stdin if equal to '-'
    #[arg(name = "file")]
    input: String,
}

#[async_trait::async_trait]
impl CliCommand for Import {
    async fn run(&self) -> Result<()> {
        if !am_root() {
            Err(GenericError::NotRunningAsRoot)?;
        }

        let data = self.deserialize_input()?;

        if let Some(repositories) = data.repositories {
            for repo_group in &repositories {
                if let Err(error) = self.import_repo_group(repo_group).await {
                    log::error!(
                        "Can't import sources for {}. {:?}",
                        repo_group.executable,
                        error
                    );
                }
            }

            let executables = repositories.iter().map(|src| src.executable.as_str());
            process::send_signal_to_multiple(executables, process::Signal::Kill);
        }

        if let Some(packages) = data.packages {
            log::info!("Importing packages...");
            Self::run_apt(&[
                "update",
                "--allow-unauthenticated",
                "--allow-insecure-repositories",
                "-o",
                "Acquire::Languages=none",
            ])?;

            let install_args = ["install", "-y", "--allow-unauthenticated", "--fix-missing"];
            let install_args: Vec<_> = install_args
                .into_iter()
                .chain(packages.iter().map(String::as_str))
                .collect();
            Self::run_apt(&install_args)?;
        }

        Ok(())
    }
}

impl Import {
    #[inline]
    fn deserialize_input(&self) -> Result<ExportData> {
        Ok(match self.input.as_str() {
            "-" => self.format.de_from_reader(io::stdin())?,
            _ => self.format.de_from_reader(StdFile::open(&self.input)?)?,
        })
    }

    async fn import_repo_group(&self, repo_group: &RepoGroup) -> Result<()> {
        log::info!(
            "Importing {} source(s) for {}",
            repo_group.sources.len(),
            repo_group.executable
        );
        let mut writer = BufWriter::new(File::create(&repo_group.path).await?);
        for source in &repo_group.sources {
            match repo_group.format {
                RepoGroupFormat::Classic => {
                    writer.write_all(source.to_one_line().as_bytes()).await?;
                }
                RepoGroupFormat::Modern => {
                    writer.write_all(source.to_deb822().as_bytes()).await?;
                    writer.write_all(b"\n").await?;
                }
            };
            writer.write_all(b"\n").await?;
        }
        writer.flush().await?;

        log::info!(
            "Triggering post-import hooks for {}...",
            repo_group.executable
        );

        match repo_group.executable.as_str() {
            "Zebra" => {
                let path = "/var/mobile/Library/Application Support/xyz.willy.Zebra/zebra.db";
                fs::remove_file(path).await?;
            }
            "Cydia" => Self::cydia_post_import_hook(repo_group)?,
            _ => {}
        }

        Ok(())
    }

    fn cydia_post_import_hook(repo_group: &RepoGroup) -> Result<()> {
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

    fn run_apt(args: &[&str]) -> Result<()> {
        let apt_cmd = Command::new("apt")
            .args(args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;

        assert!(
            apt_cmd.success(),
            "Apt exited with status: {:?}. See stderr for more info.",
            apt_cmd
        );

        Ok(())
    }
}
