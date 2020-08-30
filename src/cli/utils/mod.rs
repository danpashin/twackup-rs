use ansi_term::{Colour, ANSIString};
use std::path::Path;
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

pub fn get_packages<P: AsRef<Path>>(admin_dir: P, leaves_only: bool) -> Vec<Package> {
    let status_file = admin_dir.as_ref().join("status");
    let parser = Parser::new(status_file).expect("Failed to open database");

    let packages = parser.parse::<Package>().into_iter()
        .filter(|pkg| {
            !(pkg.identifier.starts_with("gsc.") || pkg.identifier.starts_with("cy+"))
        }).collect();

    if !leaves_only {
        return packages;
    }

    let mut leaves_indexes = Vec::with_capacity(packages.len());
    for (index, package) in packages.iter().enumerate() {
        // Skip package if it is system
        if package.section == Section::System || package.priority == Priority::Required {
            continue;
        }
        // Skip this package if it is the dependency of other
        let mut is_dependency = false;
        for pkg in packages.iter() {
            if package.is_dependency_of(pkg) {
                is_dependency = true;
                break;
            }
        }
        // Save index to filter packages in further
        if !is_dependency {
            leaves_indexes.push(index);
        }
    }

    packages.into_iter().enumerate()
        .filter(|(index, _)| { leaves_indexes.contains(index) })
        .map(|(_, pkg)| pkg).collect()
}

#[inline]
pub fn non_root_warn_msg() -> ANSIString<'static> {
    Colour::Yellow.paint(
        "You seem not to be a root user. It is highly recommended to use root, \
         in other case some operations can fail."
    )
}

/// Returns true if the `Uid` represents privileged user - root. (If it equals zero.)
#[inline]
pub fn is_root() -> bool {
    nix::unistd::getuid().is_root()
}
