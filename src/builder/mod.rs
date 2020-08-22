use crate::package::Package;
use std::{
    io, fs,
    path::{Path, PathBuf},
    sync::Arc
};

use indicatif::ProgressBar;
pub mod deb;
use deb::*;

const DEB_COMPRESSION_LEVEL: u32 = 6;

/// Creates DEB from filesystem contents
pub struct BuildWorker {
    pub package: Package,
    pub progress: Arc<ProgressBar>,
    admin_dir: PathBuf,
    destination: PathBuf,
    working_dir: PathBuf
}

impl BuildWorker {
    pub fn new(admin_dir: &Path,
               pkg: &Package,
               destination: &Path,
               progress: Arc<ProgressBar>
    ) -> Self {
        Self {
            package: pkg.clone(), progress,
            admin_dir: admin_dir.to_path_buf(),
            destination: destination.to_path_buf(),
            working_dir: destination.join(pkg.canonical_name())
        }
    }

    /// Runs worker. Should be executed in a single thread usually
    pub fn run(&self) -> io::Result<PathBuf>  {
        // Removing all dir contents
        let _ = fs::remove_dir_all(&self.working_dir);
        fs::create_dir(&self.working_dir)?;

        let deb_name = format!("{}.deb", self.package.canonical_name());
        let deb_path = self.destination.join(deb_name);

        let mut deb = Deb::new(&self.working_dir, &deb_path, DEB_COMPRESSION_LEVEL)?;
        self.archive_files(deb.data_mut_ref())?;
        self.archive_metadata(deb.control_mut_ref())?;
        deb.package()?;

        let _ = fs::remove_dir_all(&self.working_dir);

        return Ok(deb_path);
    }

    /// Archives package files and compresses in a single archive
    fn archive_files(&self, archiver: &mut DebTarArchive) -> io::Result<()> {
        let files = self.package.get_installed_files(&self.admin_dir)?;

        for file in files {
            // We'll not append root dir to archive because dpkg will unpack to root though
            if file == "/." { continue; }
            // Tricky hack. Archiver packs only relative paths. So let's add dot at start
            let res = archiver.get_mut().append_path_with_name(&file, format!(".{}", file));
            if let Err(error) = res {
                self.progress.println(format!(
                    "[{}] {}", self.package.identifier,
                    ansi_term::Colour::Yellow.paint(format!("{}", error))
                ));
            }
        }

        return Ok(());
    }

    /// Collects package metadata such as install scripts,
    /// creates control and packages all this together
    fn archive_metadata(&self, archiver: &mut DebTarArchive) -> io::Result<()> {
        // Order in this archive doesn't matter. So we'll add control at first
        archiver.append_new_file("control", &self.package.to_control().as_bytes())?;

        // Then add every matching metadata file in dpkg database dir
        let files = fs::read_dir(self.admin_dir.join("info"))?;
        for entry in files {
            if let Ok(entry) = entry {
                let file_name = entry.file_name().into_string().unwrap();

                // Firstly, reject every file not starting with package id
                if !file_name.starts_with(&self.package.identifier) { continue; }
                let id_len = self.package.identifier.len();
                // Then reject every file without dot after package id
                if file_name.chars().skip(id_len).take(1).next().unwrap_or('\0') != '.' {
                    continue;
                }
                let ext = file_name
                    .chars().skip(id_len + 1)
                    .collect::<String>();
                // And skip this two files
                // First one contains package files list
                // Second - md5 sums for every package file. Maybe it shouldn't be rejected
                // but i don't sure
                if ext == "list" || ext == "md5sums" { continue; }

                // Tricky hack. Archiver packs only relative paths. So let's add dot at start
                let abs_path =  entry.path().into_os_string().into_string().unwrap();
                let rel_path = format!("./{}", ext);

                let res = archiver.get_mut().append_path_with_name(abs_path, rel_path);
                if let Err(error) = res {
                    self.progress.println(format!(
                        "[{}] {}", self.package.identifier,
                        ansi_term::Colour::Yellow.paint(format!("{}", error))
                    ));
                }
            }
        }

        return Ok(());
    }
}
