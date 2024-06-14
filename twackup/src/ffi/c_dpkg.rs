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

use super::package::TwPackage;
use crate::{dpkg::PackagesSort, Dpkg};
use safer_ffi::{derive_ReprC, prelude::c_slice};
use std::{ffi::c_void, ptr::NonNull};
use tokio::runtime::{Builder, Runtime};

#[derive_ReprC]
#[repr(u8)]
#[derive(PartialEq, Eq)]
#[non_exhaustive]
pub enum TwPackagesSort {
    Unsorted,
    Identifier,
    Name,
}

impl From<TwPackagesSort> for PackagesSort {
    fn from(value: TwPackagesSort) -> Self {
        match value {
            TwPackagesSort::Identifier => Self::Identifier,
            TwPackagesSort::Name => Self::Name,
            TwPackagesSort::Unsorted => unreachable!("wtf, how did you broke this?"),
        }
    }
}

#[derive_ReprC]
#[repr(C)]
pub struct TwDpkg {
    dpkg_ptr: NonNull<c_void>,
    runtime_ptr: NonNull<c_void>,
}

impl TwDpkg {
    pub(crate) fn new(inner: Dpkg) -> Self {
        let dpkg_ptr = Box::into_raw(Box::new(inner));

        let tokio_rt = Builder::new_multi_thread()
            .max_blocking_threads(2)
            .build()
            .expect("Cannot start tokio runtime");
        let runtime_ptr = Box::into_raw(Box::new(tokio_rt));

        unsafe {
            Self {
                dpkg_ptr: NonNull::new_unchecked(dpkg_ptr.cast()),
                runtime_ptr: NonNull::new_unchecked(runtime_ptr.cast()),
            }
        }
    }

    #[inline]
    #[must_use]
    pub(crate) fn inner_dpkg(&self) -> &Dpkg {
        unsafe { self.dpkg_ptr.cast().as_ref() }
    }

    #[inline]
    #[must_use]
    pub(crate) fn inner_tokio_rt(&self) -> &Runtime {
        unsafe { self.runtime_ptr.cast().as_ref() }
    }

    pub(crate) fn get_packages(
        &self,
        leaves_only: bool,
        sort: TwPackagesSort,
    ) -> Option<c_slice::Box<TwPackage>> {
        let dpkg = self.inner_dpkg();
        let tokio_rt = self.inner_tokio_rt();

        let packages: Option<Vec<_>> = tokio_rt.block_on(async {
            if sort == TwPackagesSort::Unsorted {
                let packages = dpkg.unsorted_packages(leaves_only).await.ok()?;
                Some(packages.into_iter().map(Into::into).collect())
            } else {
                let packages = dpkg.packages(leaves_only, sort.into()).await.ok()?;
                Some(packages.into_values().map(Into::into).collect())
            }
        });

        Some(c_slice::Box::from(packages?.into_boxed_slice()))
    }
}

impl Drop for TwDpkg {
    fn drop(&mut self) {
        unsafe {
            drop(Box::from_raw(self.dpkg_ptr.cast::<Dpkg>().as_ptr()));
            drop(Box::from_raw(self.runtime_ptr.cast::<Runtime>().as_ptr()));
        }
    }
}
