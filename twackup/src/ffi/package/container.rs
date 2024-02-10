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
use safer_ffi::layout::OpaqueKind;
use std::ptr::NonNull;

enum Contents {
    Owned(Package),
    Borrowed(NonNull<Package>),
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct TwPackageRef(NonNull<Contents>);

impl TwPackageRef {
    pub(crate) fn owned(package: Package) -> Self {
        let contents = Box::new(Contents::Owned(package));
        Self(NonNull::new(Box::into_raw(contents)).unwrap())
    }

    pub(crate) fn unowned(package: &Package) -> Self {
        let contents = Box::new(Contents::Borrowed(
            NonNull::new(package as *const Package as *mut Package).unwrap(),
        ));
        Self(NonNull::new(Box::into_raw(contents)).unwrap())
    }

    pub(crate) fn deinit(self) {
        let raw = unsafe { Box::from_raw(self.0.as_ptr()) };
        drop(raw);
    }
}

impl AsRef<Package> for TwPackageRef {
    fn as_ref(&self) -> &Package {
        match unsafe { self.0.as_ref() } {
            Contents::Owned(package) => package,
            Contents::Borrowed(package) => unsafe { package.as_ref() },
        }
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
    fn short_name() -> String {
        "TwPackageRef".to_string()
    }

    #[cfg(feature = "ffi-headers")]
    fn define_self__impl(
        language: &'_ dyn safer_ffi::headers::languages::HeaderLanguage,
        definer: &'_ mut dyn safer_ffi::headers::Definer,
    ) -> std::io::Result<()> {
        use safer_ffi::headers::languages;
        if language.is::<languages::C>() {
            let me = Self::name(language);
            writeln!(definer.out(), "typedef void *{};", me)
        } else {
            Ok(())
        }
    }
}
