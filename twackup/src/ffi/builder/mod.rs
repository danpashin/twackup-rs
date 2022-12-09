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

pub(crate) mod progress;

use self::progress::{TwProgressFunctions, TwProgressImpl};
use crate::{
    builder::{Preferences, Worker},
    ffi::{c_dpkg::TwDpkg, package::TwPackage},
    progress::Progress,
    Result,
};
use safer_ffi::{
    derive_ReprC,
    prelude::{
        c_slice::{Box, Ref},
        char_p,
    },
    ptr::NonNullMut,
};
use std::{collections::LinkedList, os::unix::ffi::OsStringExt, sync::Arc};

#[derive_ReprC]
#[repr(C)]
pub struct TwPackagesRebuildResult {
    success: bool,
    deb_path: Option<Box<u8>>,
    error: Option<Box<u8>>,
}

pub(crate) fn rebuild_packages(
    dpkg: &TwDpkg,
    packages: Ref<'_, TwPackage>,
    functions: TwProgressFunctions,
    out_dir: char_p::Ref<'_>,
    results: Option<NonNullMut<Box<TwPackagesRebuildResult>>>,
) -> Result<()> {
    let progress = TwProgressImpl { functions };

    let tokio_rt = dpkg.inner_tokio_rt();

    let dpkg_paths = &dpkg.inner_dpkg().paths;
    let out_dir = out_dir.to_str();
    let preferences = Preferences::new(dpkg_paths, out_dir);
    let dpkg_contents = Arc::new(dpkg.inner_dpkg().info_dir_contents()?);

    let mut workers = LinkedList::new();
    for package in packages.iter() {
        let package = package.inner_ptr;
        let dpkg_contents = dpkg_contents.clone();
        let preferences = preferences.clone();

        workers.push_back(tokio_rt.spawn(async move {
            let w_package = package.as_ref();
            let worker = Worker::new(w_package, progress, None, preferences, dpkg_contents);
            worker.run().await
        }));
    }

    let mut errors_vec = vec![];
    tokio_rt.block_on(async {
        for worker in workers {
            let result = match worker.await {
                Ok(result) => result,
                _ => continue,
            };

            log::debug!("rebuild result = {:?}", result);

            let (path, error) = match result {
                Ok(path) => (Some(path), None),
                Err(error) => (None, Some(error)),
            };

            let deb_path = path.map(|path| {
                let path = OsStringExt::into_vec(path.into_os_string());
                Box::from(path.into_boxed_slice())
            });

            let error = error.map(|error| {
                let error = error.to_string().into_bytes();
                Box::from(error.into_boxed_slice())
            });

            errors_vec.push(TwPackagesRebuildResult {
                success: deb_path.is_some(),
                deb_path,
                error,
            });
        }
    });

    progress.finished_all();

    if let Some(mut results_ptr) = results {
        unsafe {
            let boxed = Box::from(errors_vec.into_boxed_slice());
            results_ptr.as_mut_ptr().write(boxed);
        }
    }

    Ok(())
}
