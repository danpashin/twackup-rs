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

#![allow(missing_docs)]

pub mod builder;
pub mod c_dpkg;
pub mod package;

use self::{
    c_dpkg::{TwDpkg, TwPackagesSort},
    package::{container::TwPackageRef, field::TwPackageField, TwPackage},
};
use crate::ffi::builder::TwPackagesRebuildResult;
use crate::Dpkg;
use builder::progress::TwProgressFunctions;
use safer_ffi::{
    derive_ReprC, ffi_export,
    prelude::c_slice,
    prelude::{char_p, repr_c},
    slice::Ref,
};

#[derive_ReprC]
#[repr(i8)]
pub enum TwResult {
    Ok,
    Error = -1,
}

/// Initialises dpkg database parser
///
/// \param[in] dpkg_dir Path to dpkg directory
/// \param[in] lock If dpkg database dir must be locked for parsing packages
///
#[ffi_export]
fn tw_init(dpkg_dir: char_p::Ref<'_>, lock: bool) -> repr_c::Box<TwDpkg> {
    let dpkg = Dpkg::new(dpkg_dir.to_str(), lock);
    repr_c::Box::new(TwDpkg::new(dpkg))
}

/// Deallocates dpkg instance
///
/// \param[in] dpkg Instance to be deallocated
///
#[ffi_export]
fn tw_free(dpkg: repr_c::Box<TwDpkg>) {
    drop(dpkg);
}

/// Fetches packages from dpkg database
///
/// \param[in] dpkg Dpkg instance
/// \param[in] leaves_only If parser should return leaves packages or not
/// \param[in] sort Sort type. Select TW_PACKAGES_SORT_UNSORTED if no sort is needed
///
#[ffi_export]
fn tw_get_packages(
    dpkg: &TwDpkg,
    leaves_only: bool,
    sort: TwPackagesSort,
) -> c_slice::Box<TwPackage> {
    dpkg.get_packages(leaves_only, sort)
}

/// Returns package section description
///
/// \param[in] package package instance
/// from which section description should be fetched
///
#[ffi_export]
fn tw_get_package_section_string(package: TwPackageRef) -> c_slice::Raw<u8> {
    package::get_section_string(package)
}

/// Fetches package field value
///
/// \param[in] package Package from which field value should be fetched
/// \param[in] field Field type
///
#[ffi_export]
fn tw_get_package_field(package: TwPackageRef, field: TwPackageField) -> c_slice::Raw<u8> {
    package::get_field(package, field)
}

/// Build control file string from package
///
/// \param[in] package Package from which control string should be build
///
#[ffi_export]
fn tw_package_build_control(package: TwPackageRef) -> c_slice::Box<u8> {
    package::build_control(package)
}

/// Rebuilds package to deb file.
///
/// \param[in] dpkg dpkg instance to run tasks
/// \param[in] packages packages to rebuild
/// \param[in] functions different functions used to report about progress
/// \param[in] out_dir directory to write deb files
/// \param[out] results paths and errors for every package.
/// YOU ARE RESPONSIBLE TO DEALLOCATE THIS BEFORE CALLING [tw_rebuild_packages] again
///
/// \returns Vector with errors. You MUST free result and all errors inside.
///
#[ffi_export]
fn tw_rebuild_packages(
    dpkg: &TwDpkg,
    packages: Ref<'_, TwPackage>,
    functions: TwProgressFunctions,
    out_dir: char_p::Ref<'_>,
    results: Option<&mut c_slice::Box<TwPackagesRebuildResult>>,
) -> TwResult {
    if builder::rebuild_packages(dpkg, packages, functions, out_dir, results).is_ok() {
        TwResult::Ok
    } else {
        TwResult::Error
    }
}

/// Deallocates memory allocated from *tw_rebuild_packages*
///
/// \param[in] results *tw_rebuild_packages* result
#[ffi_export]
fn tw_free_rebuild_results(results: c_slice::Box<TwPackagesRebuildResult>) {
    drop(results);
}

/// Generates FFI headers
///
/// # Parameters
/// - `output_dir` - Directory to which header file should be written
///
/// # Errors
/// Returns IO error if cannot write to `output_dir`
#[cfg(feature = "ffi-headers")]
pub fn generate_headers(output_dir: &std::path::Path) -> std::io::Result<()> {
    ::safer_ffi::headers::builder()
        .to_file(output_dir.join("twackup.h"))?
        .generate()
}
