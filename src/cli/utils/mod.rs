use crate::package::Section;
use ansi_term::Colour;
use std::{
    sync::Arc,
    path::PathBuf,
    collections::LinkedList,
};
use crate::{package::*, kvparser::Parser};

pub fn section_color(section: &Section)-> Colour {
    match section {
        Section::System => Colour::Fixed(9), // bright red
        Section::Tweaks => Colour::Fixed(11), // bright yellow
        Section::Utilities | Section::Packaging => Colour::Fixed(14), // bright cyan
        Section::Development => Colour::Fixed(130), // more like orange with pink
        Section::Themes => Colour::Fixed(12), // bright blue
        Section::TerminalSupport => Colour::Fixed(10), // bright green
        Section::Networking => Colour::Fixed(112), // bright green with some cyan
        Section::Archiving => Colour::Fixed(216),  // peach?
        Section::TextEditors => Colour::Fixed(162), // between red and magenta. Raspberry?
        _ => Colour::Fixed(8) // bright grey
    }
}

pub fn get_packages(admin_dir: &PathBuf, leaves_only: bool) -> Vec<Arc<Package>> {
    let status_file = admin_dir.join("status");
    let parser = Parser::new(status_file.as_path()).expect("Failed to open database");

    let unfiltered = parser.parse::<Package>().drain(..)
        .filter(|pkg| {
            !(pkg.identifier.starts_with("gsc.") || pkg.identifier.starts_with("cy+"))
        }).collect::<LinkedList<Arc<Package>>>();

    let mut filtered: Vec<Arc<Package>> = Vec::with_capacity(unfiltered.len());
    for package in unfiltered.iter() {
        if leaves_only {
            // Skip package if it is system
            if package.section == Section::System || package.priority == Priority::Required {
                continue;
            }
            // Skip this package if it is the dependency of other
            let mut is_dependency = false;
            for pkg in unfiltered.iter() {
                if package.is_dependency_of(pkg) {
                    is_dependency = true;
                    break;
                }
            }
            if !is_dependency {
                filtered.push(Arc::clone(package));
            }
        } else {
            filtered.push(Arc::clone(package));
        }
    }

    return filtered;
}
