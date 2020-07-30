use std::{
    sync::{Arc, Mutex},
    env, io, fs::{self, File},
    collections::HashMap,
    os::unix::fs::PermissionsExt
};

use crate::{package::*, parser::*};
use std::io::Write;

#[test]
fn parser_valid_database() {
    let database = env::current_dir().unwrap().join("assets/database/valid");
    let parser = Parser::new(database.as_path()).unwrap_or_else(|error| {
        panic!("Failed to open {}. {}", database.display().to_string(), error);
    });

    let packages_counter = Arc::new(Mutex::new(0));
    let handler_counter = Arc::clone(&packages_counter);
    parser.parse(move |_| -> () {
        let mut counter = handler_counter.lock().unwrap();
        *counter = *counter + 1;
    });

    let real_count = *packages_counter.lock().unwrap();
    assert_eq!(real_count, 3);
}

#[test]
fn parser_partially_valid_database() {
    let database = env::current_dir().unwrap().join("assets/database/partially_valid");
    let parser = Parser::new(database.as_path()).unwrap_or_else(|error| {
        panic!("Failed to open {}. {}", database.display().to_string(), error);
    });

    let packages_counter = Arc::new(Mutex::new(0));
    let handler_counter = Arc::clone(&packages_counter);
    parser.parse(move |_| -> () {
        let mut counter = handler_counter.lock().unwrap();
        *counter = *counter + 1;
    });

    let real_count = *packages_counter.lock().unwrap();
    assert_ne!(real_count, 3);
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

    let package = Package::new(&package_info).unwrap();

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

    let package = Package::new(&package_info).unwrap();

    let path = env::current_dir().unwrap().join("assets/packages");
    let files = package.get_installed_files(path.as_path());
    assert_eq!(files.is_err(), true);
}
