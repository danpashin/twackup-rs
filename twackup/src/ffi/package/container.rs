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

use crate::package::Package;
#[cfg(feature = "ffi-headers")]
use safer_ffi::headers::Definer;
use safer_ffi::{layout::OpaqueKind, ptr};
use std::{ffi::c_void, mem::ManuallyDrop};

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct TwPackageRef(pub(crate) ptr::NonNull<c_void>);

impl TwPackageRef {
    pub(crate) fn from_package(package: Package) -> Self {
        let package = Box::leak(Box::new(package));
        Self(ptr::NonNull::new(package as *const Package as *mut c_void).unwrap())
    }

    #[inline]
    pub(crate) fn drop_self(&mut self) {
        unsafe {
            ManuallyDrop::<Package>::drop(self.as_mut());
        }
    }
}

impl AsRef<ManuallyDrop<Package>> for TwPackageRef {
    fn as_ref(&self) -> &ManuallyDrop<Package> {
        unsafe { &*self.0.as_ptr().cast() }
    }
}

impl AsMut<ManuallyDrop<Package>> for TwPackageRef {
    fn as_mut(&mut self) -> &mut ManuallyDrop<Package> {
        unsafe { &mut *self.0.as_ptr().cast() }
    }
}

unsafe impl Send for TwPackageRef {}

unsafe impl safer_ffi::layout::ReprC for TwPackageRef {
    type CLayout = Self;

    fn is_valid(_it: &'_ Self::CLayout) -> bool {
        true
    }
}

unsafe impl safer_ffi::layout::CType for TwPackageRef {
    type OPAQUE_KIND = OpaqueKind::Concrete;

    #[cfg(feature = "ffi-headers")]
    fn c_short_name_fmt(fmt: &'_ mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "TwPackageRef")
    }

    #[cfg(feature = "ffi-headers")]
    fn c_define_self(definer: &'_ mut dyn Definer) -> std::io::Result<()> {
        let me = &Self::c_short_name().to_string();
        definer.define_once(me, &mut |definer| {
            writeln!(definer.out(), "typedef void *{};", me)
        })
    }

    #[cfg(feature = "ffi-headers")]
    fn c_var_fmt(fmt: &'_ mut std::fmt::Formatter<'_>, var_name: &'_ str) -> std::fmt::Result {
        write!(fmt, "{} {}", Self::c_short_name(), var_name)
    }
}
