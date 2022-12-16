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

pub mod container;
pub(crate) mod field;
mod priority;
mod section;
mod status;

use self::{
    container::TwPackageRef, field::TwPackageField, priority::TwPackagePriority,
    section::TwPackageSection, status::TwPackageState,
};
use crate::package::{Field, Package};
use safer_ffi::{
    derive_ReprC,
    prelude::c_slice::{Box, Raw, Ref},
};

#[derive_ReprC]
#[repr(C)]
pub struct TwPackage {
    pub(crate) inner_ptr: TwPackageRef,
    identifier: Raw<u8>,
    name: Raw<u8>,
    version: Raw<u8>,
    section: TwPackageSection,
    state: TwPackageState,
    priority: TwPackagePriority,
    get_section_string: extern "C" fn(TwPackageRef) -> Raw<u8>,
    get_field: extern "C" fn(TwPackageRef, TwPackageField) -> Raw<u8>,
    build_control: extern "C" fn(TwPackageRef) -> Box<u8>,
    get_dependencies: extern "C" fn(TwPackageRef) -> Box<Raw<u8>>,
    deallocate: extern "C" fn(TwPackageRef),
}

impl From<TwPackageRef> for TwPackage {
    fn from(package_ptr: TwPackageRef) -> Self {
        let package = package_ptr.as_ref();
        Self {
            inner_ptr: package_ptr,
            identifier: Raw::from(Ref::from(package.id.as_bytes())),
            name: Raw::from(Ref::from(package.human_name().as_bytes())),
            version: Raw::from(Ref::from(package.version.as_bytes())),
            section: (&package.section).into(),
            state: package.status.into(),
            priority: package.priority.into(),
            get_section_string,
            get_field,
            build_control,
            get_dependencies,
            deallocate,
        }
    }
}

impl From<Package> for TwPackage {
    fn from(package: Package) -> Self {
        Self::from(TwPackageRef::owned(package))
    }
}

impl From<&Package> for TwPackage {
    fn from(package: &Package) -> Self {
        Self::from(TwPackageRef::unowned(package))
    }
}

pub(crate) extern "C" fn get_section_string(package: TwPackageRef) -> Raw<u8> {
    let package = package.as_ref();
    Raw::from(Ref::from(package.section.as_str().as_bytes()))
}

pub(crate) extern "C" fn get_field(package: TwPackageRef, field: TwPackageField) -> Raw<u8> {
    let package = package.as_ref();
    let field: Field = field.into();
    match package.get(field) {
        Ok(value) => Raw::from(Ref::from(value.as_bytes())),
        Err(_) => Raw::from(Ref::default()),
    }
}

pub(crate) extern "C" fn build_control(package: TwPackageRef) -> Box<u8> {
    let control = package.as_ref().to_control();
    Box::from(control.into_bytes().into_boxed_slice())
}

pub(crate) extern "C" fn get_dependencies(package: TwPackageRef) -> Box<Raw<u8>> {
    let dependencies = package.as_ref().dependencies();
    let dependencies: Vec<_> = dependencies
        .map(|dep| Raw::from(Ref::from(dep.as_bytes())))
        .collect();
    Box::from(dependencies.into_boxed_slice())
}

extern "C" fn deallocate(package: TwPackageRef) {
    drop(package);
}
