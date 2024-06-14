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

use crate::{ffi::package::TwPackage, package::Package, progress::Progress};
use safer_ffi::{derive_ReprC, prelude::c_slice::Raw, slice::Ref};
use std::{ffi::c_void, os::unix::ffi::OsStrExt, path::Path, ptr::NonNull};

type Context = Option<NonNull<c_void>>;

#[derive_ReprC]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TwProgressFunctions {
    context: Context,
    started_processing: Option<unsafe extern "C" fn(context: Context, package: TwPackage)>,
    finished_processing:
        Option<unsafe extern "C" fn(context: Context, package: TwPackage, deb_path: Raw<u8>)>,
    finished_all: Option<unsafe extern "C" fn(context: Context)>,
}

#[derive(Copy, Clone)]
pub(crate) struct TwProgressImpl {
    pub(crate) functions: TwProgressFunctions,
}

impl Progress for TwProgressImpl {
    fn started_processing(&self, package: &Package) {
        if let Some(func) = self.functions.started_processing {
            let package = TwPackage::from(package);
            unsafe { func(self.functions.context, package) };
        }
    }

    fn finished_processing<P: AsRef<Path>>(&self, package: &Package, deb_path: P) {
        if let Some(func) = self.functions.finished_processing {
            let package = TwPackage::from(package);
            let deb_path = deb_path.as_ref().as_os_str().as_bytes();
            let deb_path = Raw::from(Ref::from(deb_path));

            unsafe { func(self.functions.context, package, deb_path) };
        }
    }

    fn finished_all(&self) {
        if let Some(func) = self.functions.finished_all {
            unsafe { func(self.functions.context) };
        }
    }
}

unsafe impl Send for TwProgressFunctions {}
unsafe impl Sync for TwProgressFunctions {}
