use crate::Package;
use std::{
    io::{self, Write}, fs::{self, File},
    path::{Path, PathBuf},
    sync::Arc
};

mod archiver;
use archiver::TarArchive;
use indicatif::ProgressBar;

/// Creates DEB from filesystem contents
pub struct BuildWorker {
    pub package: Package,
    pub progress: Arc<ProgressBar>,
    admin_dir: String,
    destination: String,
}

impl BuildWorker {
    pub fn new(admin_dir: &String,
               pkg: &Package,
               destination: &String,
               progress: Arc<ProgressBar>
    ) -> Self {
        Self {
            package: pkg.clone(), progress,
            admin_dir: admin_dir.clone(), destination: destination.clone()
        }
    }

    /// Runs worker. Should be executed in a single thread usually
    pub fn run(&self) -> io::Result<()>  {
        // Tricky hack. Because of tar contents must be relative, we must move to root dir
        if std::env::current_dir()? != Path::new("/").to_path_buf() {
            panic!("Current dir must be /!");
        }

        // Removing all dir contents
        let dir = self.get_working_dir();
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir(&dir)?;

        // Start archiving files and getting target paths
        let data_path = self.archive_files()?;
        let metadata_path = self.archive_metadata()?;
        let deb_name = format!("{}.deb", self.package.canonical_name());
        let deb_path = Path::new(&dir).parent().unwrap().join(&deb_name);

        // Now combine all files together
        let mut builder = ar::Builder::new(File::create(deb_path)?);

        // First file is debian-binary.
        // It contains just a version of dpkg used for deb creation
        // The latest one is 2.0 so we'll use this
        // Although, creating new file costs more but I did't find way to do this better
        let binary_file_path = dir.join("debian-binary");
        let mut version_file = File::create(&binary_file_path)?;
        let _ = version_file.write("2.0\n".as_bytes());

        builder.append_file(
            "debian-binary".as_bytes(),
            &mut File::open(&binary_file_path)?
        ).unwrap();

        // Second file is control archive. It is compressed with gzip and packed with tar
        builder.append_file(
            Path::new(Path::new(&metadata_path).file_name().unwrap()).to_str().unwrap().as_bytes(),
            &mut File::open(&metadata_path)?
        ).unwrap();

        // Third - main archive with data. Compressed and package same way as control
        builder.append_file(
            Path::new(Path::new(&data_path).file_name().unwrap()).to_str().unwrap().as_bytes(),
            &mut File::open(&data_path)?
        ).unwrap();

        let _ = fs::remove_dir_all(&dir);

        return Ok(());
    }

    /// Archives package files and compresses in a single archive
    fn archive_files(&self) -> io::Result<String> {
        let files = self.package.get_installed_files(Path::new(&self.admin_dir))?;

        let working_dir = self.get_working_dir();
        // Firstly, we'll pack all files together
        let temp_file = self.get_working_dir().join("data.tar");
        let mut archiver = TarArchive::new(temp_file.as_path())?;

        for file in files {
            // We'll not append root dir to archive 'cause dpkg will unpack to root though
            if file == "/." { continue; }
            // Tricky hack. Archiver packs only relative paths. So let's add dot at start
            let path = format!(".{}", file);
            if let Err(error) = archiver.append_path(Path::new(&path)) {
                self.progress.println(format!(
                    "[{}] {}", self.package.identifier,
                    ansi_term::Colour::Yellow.paint(format!("{}", error))
                ));
            }
        }

        // Finish and compress with gzip
        archiver.finish_appending()?;
        let output_file = working_dir.join("data.tar.gz");
        archiver.compress_gzip(output_file.as_path(), 6)?;

        return Ok(output_file.to_str().unwrap().to_string());
    }

    /// Collects package metadata such as install scripts,
    /// creates control and packages all this together
    fn archive_metadata(&self) -> io::Result<String> {
        let working_dir = self.get_working_dir();
        let mut archiver = TarArchive::new(working_dir.join("control.tar").as_path())?;
        // Order in this archive doesn't matter. So we'll add control at first
        archiver.append_new_file(Path::new("control"), &self.package.create_control().as_bytes())?;

        // Then add every matching metadata file in dpkg database dir
        let files = fs::read_dir(Path::new(&self.admin_dir).join("info"))?;
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
                    .collect::<String>()
                    .to_lowercase();
                // And skip this two files
                // First one contains package files list
                // Second - md5 sums for every package file. Maybe it shouldn't be rejected but i don't sure
                if ext == "list" || ext == "md5sums" { continue; }

                let abs_path =  entry.path().into_os_string().into_string().unwrap();
                // Tricky hack again!
                let rel_path = format!(".{}",abs_path).to_string();
                let path = Path::new(&rel_path);

                if let Err(error) = archiver.append_path_with_name(path, Path::new(&ext)) {
                    self.progress.println(format!(
                        "[{}] {}", self.package.identifier,
                        ansi_term::Colour::Yellow.paint(format!("{}", error))
                    ));
                }
            }
        }

        // Finish and compress with gzip
        archiver.finish_appending()?;
        let output_file = working_dir.join("control.tar.gz");
        archiver.compress_gzip(output_file.as_path(), 6)?;

        return Ok(output_file.to_str().unwrap().to_string());
    }

    fn get_working_dir(&self) -> PathBuf {
        return Path::new(&self.destination).join(self.package.canonical_name());
    }
}
