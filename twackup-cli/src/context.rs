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

use super::progress_bar::ProgressBar;
use std::{
    collections::BTreeMap,
    path::Path,
    time::{Duration, Instant},
};
use twackup::{dpkg::Dpkg, error::Result, package::Package};

pub(crate) struct Context {
    pub(crate) start_time: Instant,
    pub(crate) is_root: bool,
}

impl Context {
    pub(crate) fn new() -> Self {
        Self {
            start_time: Instant::now(),
            is_root: libproc::libproc::proc_pid::am_root(),
        }
    }

    pub(crate) fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    pub(crate) fn progress_bar(&self, length: u64) -> &'static ProgressBar {
        let progress_bar = indicatif::ProgressBar::new(length);
        progress_bar.set_style(
            indicatif::ProgressStyle::default_bar()
                .template("{pos}/{len} [{wide_bar:.cyan/blue}] {msg}")
                .expect("Progress bar template error!")
                .progress_chars("##-"),
        );

        let progress_bar = ProgressBar(progress_bar);
        progress_bar.make_static()
    }

    pub(crate) async fn packages<P: AsRef<Path>>(
        &self,
        admin_dir: P,
        leaves_only: bool,
    ) -> Result<BTreeMap<String, Package>> {
        Dpkg::new(admin_dir, true).packages(leaves_only).await
    }
}
