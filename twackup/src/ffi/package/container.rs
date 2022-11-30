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

struct Contents<'a> {
    is_inner_droppable: bool,
    package: &'a Package,
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct TwPackageRef(pub(crate) ptr::NonNull<c_void>);

impl TwPackageRef {
    pub(crate) fn owned(package: Package) -> Self {
        let package = Box::leak(Box::new(package));
        let contents = Box::leak(Box::new(Contents {
            is_inner_droppable: true,
            package,
        }));

        Self(ptr::NonNull::new(contents as *const Contents<'_> as *mut c_void).unwrap())
    }

    pub(crate) fn unowned(package: &Package) -> Self {
        let contents = Box::leak(Box::new(Contents {
            is_inner_droppable: false,
            package,
        }));

        Self(ptr::NonNull::new(contents as *const Contents<'_> as *mut c_void).unwrap())
    }

    fn inner(&self) -> &ManuallyDrop<Contents<'_>> {
        unsafe { &*self.0.as_ptr().cast() }
    }

    fn inner_mut(&mut self) -> &mut ManuallyDrop<Contents<'_>> {
        unsafe { &mut *self.0.as_ptr().cast() }
    }

    #[inline]
    pub(crate) fn drop_self(&mut self) {
        unsafe {
            let contents: &mut ManuallyDrop<Contents<'_>> = self.inner_mut();
            if contents.is_inner_droppable {
                let package = contents.package as *const Package;
                let package = &mut *package.cast_mut().cast();
                ManuallyDrop::<Package>::drop(package);
            }

            ManuallyDrop::drop(contents);
        }
    }
}

impl AsRef<Package> for TwPackageRef {
    fn as_ref(&self) -> &Package {
        self.inner().package
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
