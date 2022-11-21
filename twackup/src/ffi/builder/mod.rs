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

pub mod progress;

use super::{c_dpkg::TwDpkg, package::TwPackage};
use crate::{
    builder::{Preferences, Worker},
    progress::Progress,
};
use progress::{TwProgressFunctions, TwProgressImpl};
use safer_ffi::{
    derive_ReprC,
    prelude::{
        c_slice::{Box, Ref},
        char_p,
    },
};
use std::collections::LinkedList;
use std::sync::Arc;

#[derive_ReprC]
#[repr(transparent)]
pub struct TwPackagesRebuildResult(Box<Box<u8>>);

pub(crate) fn rebuild_packages(
    dpkg: &TwDpkg,
    packages: Ref<'_, TwPackage>,
    functions: &'static TwProgressFunctions,
    out_dir: char_p::Ref<'_>,
) -> TwPackagesRebuildResult {
    let mut progress = TwProgressImpl::new(packages.len() as u64);
    progress.set_functions(*functions);

    let tokio_rt = dpkg.inner_tokio_rt();

    let dpkg_paths = &dpkg.inner_dpkg().paths;
    let out_dir = out_dir.to_str();
    let preferences = Preferences::new(dpkg_paths, out_dir);
    let dpkg_contents = dpkg.inner_dpkg().info_dir_contents().unwrap();
    let dpkg_contents = Arc::new(dpkg_contents);

    let mut workers = LinkedList::new();
    for package in packages.iter() {
        let package = package.inner_ptr;
        let progress = progress.clone();
        let dpkg_contents = dpkg_contents.clone();
        let preferences = preferences.clone();

        workers.push_back(tokio_rt.spawn(async move {
            let w_package = package.as_ref();
            let worker = Worker::new(w_package, progress, None, preferences, dpkg_contents);
            worker.run().await
        }));
    }

    let mut errors = vec![];
    tokio_rt.block_on(async {
        for worker in workers {
            if let Ok(Err(error)) = worker.await {
                let error = error.to_string();
                let error = Box::from(error.into_bytes().into_boxed_slice());
                errors.push(error);
            }
        }
    });

    TwPackagesRebuildResult(Box::from(errors.into_boxed_slice()))
}
