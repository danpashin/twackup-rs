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

mod arc_container;
pub mod builder;
pub mod c_dpkg;
mod logger;
pub mod package;

use self::{
    arc_container::ArcContainer,
    builder::{TwBuildParameters, TwPackagesRebuildResult},
    c_dpkg::{TwDpkg, TwPackagesSort},
    logger::{Logger, TwLogFunctions, TwMessageLevel},
    package::{field::TwPackageField, TwPackage},
};
use crate::{
    package::{Field, Package},
    Dpkg,
};
use safer_ffi::{boxed, derive_ReprC, ffi_export, prelude::c_slice, prelude::char_p};
use std::{mem, mem::ManuallyDrop, ptr::NonNull, slice};

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
fn tw_init(dpkg_dir: char_p::Ref<'_>, lock: bool) -> boxed::Box_<TwDpkg> {
    let dpkg = Dpkg::new(dpkg_dir.to_str(), lock);
    boxed::Box_::new(TwDpkg::new(dpkg))
}

/// Deallocates dpkg instance
///
/// \param[in] dpkg Instance to be deallocated
///
#[ffi_export]
fn tw_free(dpkg: boxed::Box_<TwDpkg>) {
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
    output: safer_ffi::prelude::Out<'_, NonNull<TwPackage>>,
) -> i64 {
    if let Some(packages) = dpkg.get_packages(leaves_only, sort) {
        let packages = ManuallyDrop::new(packages);

        // Since memory allocators for Rust and outer FFI callee can be different
        // we must release slice by ourselves
        //
        // To do that, slice must have valid length and layout
        // Releasing func will decrement counter on ARC, so we need to increase it here
        for package in packages.as_slice() {
            package.inner.retain();
        }

        let packages_ptr = packages.as_ptr().cast_mut();
        unsafe { output.write(NonNull::new_unchecked(packages_ptr)) };

        packages.len() as i64
    } else {
        TwResult::Error as i64
    }
}

#[ffi_export]
fn tw_free_packages(mut start: NonNull<TwPackage>, length: i64) {
    unsafe {
        let packages = slice::from_raw_parts_mut(start.as_mut(), length as usize);
        drop(Box::from_raw(packages))
    }
}

/// Returns package section description
///
/// \param[in] package package instance
/// from which section description should be fetched
///
#[ffi_export]
fn tw_package_section_str(package: ArcContainer<Package>) -> c_slice::Raw<u8> {
    // Since ArcContainer was initialized just by casting pointer, we should retain it
    // counter will be decremented automatically when container drops
    package.retain();

    let section = package.section.as_str().as_bytes();
    c_slice::Ref::from(section).into()
}

/// Fetches package field value
///
/// \param[in] package Package from which field value should be fetched
/// \param[in] field Field type
///
#[ffi_export]
fn tw_package_field_str(package: ArcContainer<Package>, field: TwPackageField) -> c_slice::Raw<u8> {
    // Since ArcContainer was initialized just by casting pointer, we should retain it
    // counter will be decremented automatically when container drops
    package.retain();

    let field: Field = field.into();
    match package.get(field) {
        Ok(value) => c_slice::Raw::from(c_slice::Ref::from(value.as_bytes())),
        Err(_) => c_slice::Raw::from(c_slice::Ref::default()),
    }
}

/// Build control file string from package
///
/// \param[in] package Package from which control string should be build
///
#[ffi_export]
fn tw_package_build_control(package: ArcContainer<Package>) -> c_slice::Box<u8> {
    // Since ArcContainer was initialized just by casting pointer, we should retain it
    // counter will be decremented automatically when container drops
    package.retain();

    let control = package.to_control();
    c_slice::Box::from(control.into_bytes().into_boxed_slice())
}

#[ffi_export]
fn tw_package_dependencies(package: ArcContainer<Package>) -> c_slice::Box<c_slice::Raw<u8>> {
    // Since ArcContainer was initialized just by casting pointer, we should retain it
    // counter will be decremented automatically when container drops
    package.retain();

    let dependencies = package.dependencies();
    let dependencies: Vec<_> = dependencies
        .map(|dep| c_slice::Raw::from(c_slice::Ref::from(dep.as_bytes())))
        .collect();
    c_slice::Box::from(dependencies.into_boxed_slice())
}

/// Deallocated package instance. Nothing else
#[ffi_export]
fn tw_package_release(package: ArcContainer<Package>) {
    // Should not call release on package here. It will be release by Drop::drop
    drop(package);
}

#[ffi_export]
fn tw_package_ref_count(package: ArcContainer<Package>) -> u64 {
    let package = ManuallyDrop::new(package);
    package.ref_count() as u64
}

#[ffi_export]
fn tw_package_retain(package: ArcContainer<Package>) {
    package.retain();
    mem::forget(package);
}

/// Rebuilds package to deb file.
///
/// \param[in] dpkg dpkg instance to run tasks
/// \param[in] parameters Different build parameters
///
/// \returns TW_RESULT_OK if rebuild is success
///
#[ffi_export]
fn tw_rebuild_packages(dpkg: &TwDpkg, parameters: TwBuildParameters<'_>) -> TwResult {
    if builder::rebuild_packages(dpkg, parameters).is_ok() {
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

/// Enables library internal logging
///
/// \param[in] functions Different log functions that will be used to called outside lib
/// \param[in] level Logging level
#[ffi_export]
fn tw_enable_logging(functions: TwLogFunctions, level: TwMessageLevel) {
    Logger::init(functions, level);
}

/// Checked if logger is already enabled
#[ffi_export]
fn tw_is_logging_enabled() -> bool {
    Logger::is_already_initted()
}

static VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    "-",
    env!("VERGEN_GIT_DESCRIBE"),
    "\0",
);

/// Returns library version. It is static - no need to deallocate it.
#[ffi_export]
fn tw_library_version() -> char_p::Ref<'static> {
    unsafe { char_p::Ref::from_ptr_unchecked(NonNull::new_unchecked(VERSION.as_ptr().cast_mut())) }
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
