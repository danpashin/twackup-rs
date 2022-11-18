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

pub(crate) mod field;
mod priority;
mod section;
mod status;

use self::{priority::TwPackagePriority, section::TwPackageSection, status::TwPackageState};
use crate::ffi::package::field::TwPackageField;
use crate::package::{Field, Package};
use safer_ffi::{derive_ReprC, prelude::c_slice, ptr};
use std::{ffi::c_void, mem::ManuallyDrop};

#[derive_ReprC]
#[repr(C)]
pub struct TwPackage<'a> {
    inner_ptr: ptr::NonNull<c_void>,
    identifier: c_slice::Ref<'a, u8>,
    name: c_slice::Ref<'a, u8>,
    version: c_slice::Ref<'a, u8>,
    section: TwPackageSection,
    state: TwPackageState,
    priority: TwPackagePriority,
}

impl<'a> TwPackage<'a> {
    pub(crate) fn new(package: Package) -> Self {
        let package = Box::leak(Box::new(package));

        Self {
            inner_ptr: ptr::NonNull::new(package as *const Package as *mut c_void).unwrap(),
            identifier: c_slice::Ref::from(package.id.as_bytes()),
            name: c_slice::Ref::from(package.human_name().as_bytes()),
            version: c_slice::Ref::from(package.version.as_bytes()),
            section: (&package.section).into(),
            state: package.status.into(),
            priority: package.priority.into(),
        }
    }

    #[inline]
    #[must_use]
    fn rust_package(&self) -> &Package {
        unsafe { &*self.inner_ptr.as_ptr().cast() }
    }

    pub(crate) fn get_section_description(&self) -> c_slice::Ref<'_, u8> {
        let package = self.rust_package();
        c_slice::Ref::from(package.section.as_str().as_bytes())
    }

    pub(crate) fn get_field(&self, field: TwPackageField) -> c_slice::Ref<'_, u8> {
        let package = self.rust_package();
        let field: Field = field.into();
        match package.get(field) {
            Ok(value) => c_slice::Ref::from(value.as_bytes()),
            Err(_) => c_slice::Ref::default(),
        }
    }
}

impl<'a> Drop for TwPackage<'a> {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::<Package>::drop(&mut *self.inner_ptr.as_ptr().cast());
        }
    }
}
