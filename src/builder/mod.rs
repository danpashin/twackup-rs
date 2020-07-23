use crate::Package;
use std::{
    io, fs::{self, File},
    path::{Path, PathBuf},
};

mod archiver;
use archiver::TarArchive;
use std::io::Write;

pub struct BuildWorker {
    pub package: Package,
    admin_dir: String,
    destination: String
}

impl BuildWorker {
    pub fn new(admin_dir: &String, pkg: &Package, destination: &String) -> Self {
        return Self {
            package: pkg.clone(),
            admin_dir: admin_dir.clone(),
            destination: destination.clone()
        }
    }

    pub fn run(&self) -> io::Result<()>  {
        if std::env::current_dir()? != Path::new("/").to_path_buf() {
            panic!("Current dir must be root!");
        }
        let dir = self.get_working_dir();
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir(&dir)?;

        let data_path = self.archive_files()?;
        let metadata_path = self.archive_metadata()?;
        let deb_name = format!("{}.deb", self.package.canonical_name());
        let deb_path = Path::new(&dir).parent().unwrap().join(&deb_name);

        let mut builder = ar::Builder::new(File::create(deb_path)?);

        let binary_file_path = dir.join("debian-binary");
        let mut version_file = File::create(&binary_file_path)?;
        let _ = version_file.write("2.0\n".as_bytes());

        builder.append_file(
            "debian-binary".as_bytes(),
            &mut File::open(&binary_file_path)?
        ).unwrap();

        builder.append_file(
            Path::new(Path::new(&metadata_path).file_name().unwrap()).to_str().unwrap().as_bytes(),
            &mut File::open(&metadata_path)?
        ).unwrap();

        builder.append_file(
            Path::new(Path::new(&data_path).file_name().unwrap()).to_str().unwrap().as_bytes(),
            &mut File::open(&data_path)?
        ).unwrap();

        let _ = fs::remove_dir_all(&dir);

        return Ok(());
    }

    fn archive_files(&self) -> io::Result<String> {
        let files = self.package.get_installed_files(&self.admin_dir)?;

        let working_dir = self.get_working_dir();
        let temp_file = Path::new(&working_dir).join("data.tar");
        let mut archiver = TarArchive::new(temp_file.as_path())?;

        for file in files {
            if file == "/." {
                continue;
            }
            let path = format!(".{}", file);
            if let Err(error) = archiver.append_path(Path::new(&path)) {
                eprintln!("Error while archiving file for package {}. {}",
                          self.package.identifier,
                          error);
            }
        }

        archiver.finish_appending()?;
        let output_file = Path::new(&working_dir).join("data.tar.gz");
        archiver.compress_gzip(output_file.as_path(), 6)?;

        return Ok(output_file.to_str().unwrap().to_string());
    }

    fn archive_metadata(&self) -> io::Result<String> {
        let working_dir = self.get_working_dir();
        let mut archiver = TarArchive::new(working_dir.join("control.tar").as_path())?;
        archiver.append_new_file(Path::new("control"), &self.package.create_control().as_bytes())?;

        let files = fs::read_dir(Path::new(&self.admin_dir).join("info"))?;
        for entry in files {
            if let Ok(entry) = entry {
                let file_name = entry.file_name().into_string().unwrap();

                if !file_name.starts_with(&self.package.identifier) { continue; }
                let id_len = self.package.identifier.len();
                if file_name.chars().skip(id_len).take(1).next().unwrap_or('\0') != '.' {
                    continue;
                }
                let ext = file_name
                    .chars().skip(id_len + 1)
                    .collect::<String>()
                    .to_lowercase();
                if ext == "list" || ext == "md5sums" { continue; }

                let abs_path =  entry.path().into_os_string().into_string().unwrap();
                let rel_path = format!(".{}",abs_path).to_string();
                let path = Path::new(&rel_path);

                if let Err(error) = archiver.append_path_with_name(path, Path::new(&ext)) {
                    eprintln!("Error while archiving control for package {}. {}",
                              self.package.identifier,
                              error);
                }
            }
        }

        archiver.finish_appending()?;

        let output_file = Path::new(&working_dir).join("control.tar.gz");
        archiver.compress_gzip(output_file.as_path(), 6)?;

        return Ok(output_file.to_str().unwrap().to_string());
    }

    fn get_working_dir(&self) -> PathBuf {
        return Path::new(&self.destination).join(self.package.canonical_name());
    }
}
