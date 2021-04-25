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

use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::{self, BufRead, BufReader, Write},
    os::unix::fs::PermissionsExt,
};

use crate::{kvparser::*, package::*, repository::*};

fn tokio_runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .build()
        .expect("Can't build tokio runtime")
}

#[test]
fn parser_valid_database() {
    let database = env::current_dir().unwrap().join("assets/database/valid");
    let parser = Parser::new(database.as_path()).unwrap();
    let packages = tokio_runtime().block_on(parser.parse::<Package>());
    assert_eq!(packages.len(), 3);
}

#[test]
fn parser_partially_valid_database() {
    let database = env::current_dir()
        .unwrap()
        .join("assets/database/partially_valid");
    let parser = Parser::new(database.as_path()).unwrap();
    let packages = tokio_runtime().block_on(parser.parse::<Package>());
    assert_ne!(packages.len(), 3);
}

#[test]
fn parser_multiline() {
    let database = env::current_dir()
        .unwrap()
        .join("assets/database/multiline");
    let parser = Parser::new(database.as_path()).unwrap();

    let packages: HashMap<String, Package> = tokio_runtime().block_on(async {
        parser
            .parse::<Package>()
            .await
            .into_iter()
            .map(|pkg| (pkg.id.clone(), pkg))
            .collect()
    });

    let package = packages.get("valid-package-1").unwrap();
    let description = package.get_field(&Field::Description).unwrap();
    assert_eq!(
        description.as_str(),
        "First Line\n Second Line\n  Third Line"
    );

    let package = packages.get("valid-package-2").unwrap();
    let description = package.get_field(&Field::Description).unwrap();
    assert_eq!(description.as_str(), "First Line");
}

#[test]
fn parser_no_permissions_database() {
    let database = env::temp_dir().join("twackup-no-permissions");
    let mut file = File::create(&database).unwrap();
    file.write("This contents will never be read".as_bytes())
        .unwrap();
    fs::set_permissions(&database, fs::Permissions::from_mode(0o333)).unwrap();

    let parser = Parser::new(database.as_path());
    assert_eq!(parser.is_err(), true);
    assert_eq!(
        io::Error::last_os_error().kind(),
        io::ErrorKind::PermissionDenied
    );

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
    let repositories: HashMap<String, Repository> = tokio_runtime().block_on(async {
        parser
            .parse::<Repository>()
            .await
            .into_iter()
            .map(|repo| (repo.url.clone(), repo))
            .collect()
    });

    assert_eq!(repositories.len(), 3);

    let repo = repositories.get("https://apt1.example.com/").unwrap();
    assert_eq!(repo.components.as_slice(), &["main", "orig"]);
}

#[test]
fn parser_classic_repository() {
    let database = env::current_dir()
        .unwrap()
        .join("assets/sources_db/classic");
    let reader = BufReader::new(File::open(database).unwrap());

    let repositories: HashMap<String, Repository> = reader
        .lines()
        .map(|line| {
            let line = line.expect("Can't unwrap line");
            eprintln!("{}", line);
            let repo = Repository::from_one_line(line.as_str()).expect("Parsing repo failed");
            (repo.url.clone(), repo)
        })
        .collect();

    assert_eq!(repositories.len(), 3);

    let repo = repositories.get("https://apt1.example.com/").unwrap();
    assert_eq!(repo.distribution.as_str(), "stable");
    assert_eq!(repo.components.as_slice(), &["main", "orig"]);
}
