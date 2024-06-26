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
use super::ArcContainer;
use crate::{
    builder::{Preferences, Worker},
    ffi::{c_dpkg::TwDpkg, package::TwPackage},
    package::Package,
    progress::Progress,
    Result,
};
use safer_ffi::{
    derive_ReprC,
    prelude::{
        c_slice::{Box, Ref},
        char_p, Out,
    },
    ptr::NonNullMut,
};
use std::{collections::LinkedList, os::unix::ffi::OsStringExt, sync::Arc};

#[derive_ReprC]
#[repr(C)]
pub struct TwPackagesRebuildResult {
    success: bool,
    package: ArcContainer<Package>,
    deb_path: Option<Box<u8>>,
    error: Option<Box<u8>>,
}

#[derive_ReprC]
#[repr(u8)]
pub enum TwCompressionType {
    Gz,
    Xz,
    Zst,
    Bz2,
}

impl From<TwCompressionType> for crate::archiver::Type {
    fn from(value: TwCompressionType) -> Self {
        match value {
            TwCompressionType::Gz => Self::Gz,
            TwCompressionType::Xz => Self::Xz,
            TwCompressionType::Zst => Self::Zst,
            TwCompressionType::Bz2 => Self::Bz2,
        }
    }
}

#[derive_ReprC]
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum TwCompressionLevel {
    None,
    Fast,
    Normal,
    Best,
}

impl From<TwCompressionLevel> for crate::archiver::Level {
    fn from(level: TwCompressionLevel) -> Self {
        match level {
            TwCompressionLevel::None => Self::None,
            TwCompressionLevel::Fast => Self::Fast,
            TwCompressionLevel::Normal => Self::Normal,
            TwCompressionLevel::Best => Self::Best,
        }
    }
}

#[derive_ReprC]
#[repr(C)]
pub struct TwBuildPreferences {
    compression_type: TwCompressionType,
    compression_level: TwCompressionLevel,
    follow_symlinks: bool,
}

#[derive_ReprC]
#[repr(C)]
pub struct TwBuildParameters<'a> {
    packages: Ref<'a, ArcContainer<Package>>,
    functions: TwProgressFunctions,
    out_dir: char_p::Ref<'a>,
    preferences: TwBuildPreferences,
    results: Option<Out<'a, Box<TwPackagesRebuildResult>>>,
}

pub(crate) fn rebuild_packages(dpkg: &TwDpkg, parameters: TwBuildParameters<'_>) -> Result<()> {
    let progress = TwProgressImpl {
        functions: parameters.functions,
    };

    let tokio_rt = dpkg.inner_tokio_rt();

    let dpkg_paths = &dpkg.inner_dpkg().paths;
    let out_dir = parameters.out_dir.to_str();
    let mut preferences = Preferences::new(dpkg_paths, out_dir);
    preferences.compression.level = parameters.preferences.compression_level.into();
    preferences.compression.r#type = parameters.preferences.compression_type.into();
    preferences.follow_symlinks = parameters.preferences.follow_symlinks;

    let dpkg_contents = Arc::new(dpkg.inner_dpkg().info_dir_contents()?);

    let mut workers = LinkedList::new();
    for package in parameters.packages.iter() {
        let package = package.clone();
        let dpkg_contents = dpkg_contents.clone();
        let preferences = preferences.clone();

        workers.push_back(tokio_rt.spawn(async move {
            let worker = Worker::new(&*package, progress, None, preferences, dpkg_contents);
            let result = worker.run().await;
            (package, result)
        }));
    }

    let mut results = vec![];
    tokio_rt.block_on(async {
        for worker in workers {
            let Ok((package, result)) = worker.await else {
                continue;
            };

            log::debug!("rebuild result = {result:?}");

            let result = result
                .map(|path| {
                    let path = OsStringExt::into_vec(path.into_os_string());
                    Box::from(path.into_boxed_slice())
                })
                .map_err(|error| {
                    let error = error.to_string().into_bytes();
                    Box::from(error.into_boxed_slice())
                });

            let (deb_path, error) = match result {
                Ok(path) => (Some(path), None),
                Err(error) => (None, Some(error)),
            };

            results.push(TwPackagesRebuildResult {
                success: deb_path.is_some(),
                package,
                deb_path,
                error,
            });
        }
    });

    progress.finished_all();

    if let Some(results_out) = parameters.results {
        let boxed = Box::from(results.into_boxed_slice());
        results_out.write(boxed);
    }

    Ok(())
}
