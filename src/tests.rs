use std::{
    env, io::{self, Write, BufReader, BufRead}, fs::{self, File},
    collections::HashMap,
    os::unix::fs::PermissionsExt,
};

use crate::{package::*, kvparser::*, repository::*};

#[test]
fn parser_valid_database() {
    let database = env::current_dir().unwrap().join("assets/database/valid");
    let parser = Parser::new(database.as_path()).unwrap();
    let packages = parser.parse::<Package>();
    assert_eq!(packages.len(), 3);
}

#[test]
fn parser_partially_valid_database() {
    let database = env::current_dir().unwrap().join("assets/database/partially_valid");
    let parser = Parser::new(database.as_path()).unwrap();
    let packages = parser.parse::<Package>();
    assert_ne!(packages.len(), 3);
}

#[test]
fn parser_multiline() {
    let database = env::current_dir().unwrap().join("assets/database/multiline");
    let parser = Parser::new(database.as_path()).unwrap();

    let packages: HashMap<String, Package> = parser.parse::<Package>().iter().map(|pkg| {
        (pkg.identifier.clone(), pkg.as_ref().clone())
    }).collect();

    let first_package = packages.get("valid-package-1").unwrap();
    assert_eq!(first_package.description().unwrap().as_str(), "First Line\n Second Line\n  Third Line");

    let second_pkg = packages.get("valid-package-2").unwrap();
    assert_eq!(second_pkg.description().unwrap().as_str(), "First Line");
}

#[test]
fn parser_no_permissions_database() {
    let database = env::temp_dir().join("twackup-no-permissions");
    let mut file = File::create(&database).unwrap();
    file.write("This contents will never be read".as_bytes()).unwrap();
    fs::set_permissions(&database, fs::Permissions::from_mode(0o333)).unwrap();

    let parser = Parser::new(database.as_path());
    assert_eq!(parser.is_err(), true);
    assert_eq!(io::Error::last_os_error().kind(), io::ErrorKind::PermissionDenied);

    fs::remove_file(&database).unwrap();
}

#[test]
fn valid_package_get_files() {
    let mut package_info: HashMap<String, String> = HashMap::new();
    package_info.insert("Package".to_string(), "valid-package".to_string());
    package_info.insert("Version".to_string(), "1.0.0".to_string());
    package_info.insert("Architecture".to_string(), "all".to_string());

    let package = Package::new(package_info).unwrap();

    let path = env::current_dir().unwrap().join("assets/packages");
    let files = package.get_installed_files(path.as_path());
    assert_eq!(files.is_err(), false);
    assert_eq!(files.unwrap().len(), 3);
}

#[test]
fn non_valid_package_get_files() {
    let mut package_info = HashMap::new();
    package_info.insert("Package".to_string(), "non-valid-package".to_string());
    package_info.insert("Version".to_string(), "1.0.0".to_string());
    package_info.insert("Architecture".to_string(), "all".to_string());

    let package = Package::new(package_info).unwrap();

    let path = env::current_dir().unwrap().join("assets/packages");
    let files = package.get_installed_files(path.as_path());
    assert_eq!(files.is_err(), true);
}

#[test]
fn parser_modern_repository() {
    let database = env::current_dir().unwrap().join("assets/sources_db/modern");
    let parser = Parser::new(database.as_path()).unwrap();
    let repositories: HashMap<String, Repository> = parser.parse::<Repository>().iter().map(|repo| {
        (repo.address.clone(), repo.as_ref().clone())
    }).collect();

    assert_eq!(repositories.len(), 3);

    let repo = repositories.get("https://apt1.example.com/").unwrap();
    assert_eq!(repo.components.as_str(), "main orig");
}

#[test]
fn parser_classic_repository() {
    let database = env::current_dir().unwrap().join("assets/sources_db/classic");
    let reader = BufReader::new(File::open(database).unwrap());

    let repositories: HashMap<String, Repository> = reader.lines().map(|line| {
        let line = line.expect("Can't unwrap line");
        eprintln!("{}", line);
        let repo = Repository::from_oneline(line.as_str()).expect("Parsing repo failed");
        (repo.address.clone(), repo)
    }).collect();

    assert_eq!(repositories.len(), 3);

    let repo = repositories.get("https://apt1.example.com/").unwrap();
    assert_eq!(repo.distribution.as_str(), "stable");
    assert_eq!(repo.components.as_str(), "main orig");
}
