use crate::*;
use std::{
    io::{self, BufWriter, Write}, fs::{self, File},
};

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
        let dir = self.get_working_dir();
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir(&dir)?;

        self.copy_files()?;
        self.copy_metadata()?;

        return Ok(());
    }

    fn copy_files(&self) -> io::Result<()> {
        let dir = self.get_working_dir();
        let files = self.package.get_installed_files(&self.admin_dir)?;
        for file in files {
            if file == "/." {
                continue;
            }

            let metadata = fs::metadata(&file);
            if let Err(error) = metadata {
                eprintln!("{} - {}", &file, error);
                continue;
            }

            let destination = format!("{}/{}", &dir, &file);
            if metadata.unwrap().is_dir() {
                if let Err(error) = fs::create_dir(&destination) {
                    eprintln!("{} - {}", &file, error);
                }
            } else {
                if let Err(error) = fs::copy(&file, &destination) {
                    eprintln!("{} - {}", &file, error);
                }
            }
        }

        return Ok(());
    }

    fn copy_metadata(&self) -> io::Result<()> {
        let working_dir = self.get_working_dir();
        let debian_dir = format!("{}/DEBIAN", working_dir);
        fs::create_dir(&debian_dir)?;

        let control_file = File::create(format!("{}/control", &debian_dir))?;
        let control_contents = self.package.create_control();
        BufWriter::new(control_file).write_all(control_contents.as_bytes())?;

        let files = fs::read_dir(format!("{}/info", self.admin_dir))?;
        for entry in files {
            if let Ok(entry) = entry {
                let file_name = entry.file_name().into_string().unwrap();

                if !file_name.starts_with(&self.package.identifier) { continue; }
                let id_len = self.package.identifier.len();
                if file_name.chars().skip(id_len).take(1).next().unwrap_or('\0') != '.' {
                    continue;
                }
                let ext = file_name.chars().skip(id_len + 1).collect::<String>().to_lowercase();
                if ext == "list" || ext == "md5sums" { continue; }

                let source = entry.path().into_os_string().into_string().unwrap();
                let destination = format!("{}/{}", debian_dir, file_name);
                if let Err(error) = fs::copy(&source, &destination) {
                    eprintln!("{} - {}", source, error);
                }
            }
        }

        return Ok(());
    }

    fn get_working_dir(&self) -> String {
        return format!("{}/{}", &self.destination, self.package.canonical_name());
    }

}
