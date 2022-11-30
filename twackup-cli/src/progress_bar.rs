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

use console::style;
use indicatif::ProgressBar as ProgressBarImpl;
use std::path::Path;
use twackup::{
    package::Package,
    progress::{MessageLevel, Progress},
};

pub(crate) static mut PROGRESS_BAR: Option<&'static ProgressBar> = None;

#[derive(Clone)]
pub(crate) struct ProgressBar(pub(crate) ProgressBarImpl);

impl ProgressBar {
    pub(crate) fn default(length: u64) -> &'static Self {
        let progress_bar = indicatif::ProgressBar::new(length);
        progress_bar.set_style(
            indicatif::ProgressStyle::default_bar()
                .template("{pos}/{len} [{wide_bar:.cyan/blue}] {msg}")
                .expect("Progress bar template error!")
                .progress_chars("##-"),
        );

        let progress_bar = Self(progress_bar);
        progress_bar.make_static()
    }

    pub(crate) fn make_static(self) -> &'static Self {
        unsafe {
            assert!(PROGRESS_BAR.is_none(), "progress bar is already set!");

            let pb = Box::leak(Box::new(self));
            PROGRESS_BAR = Some(pb);
            pb
        }
    }
}

impl Progress for ProgressBar {
    fn print_message<M: AsRef<str>>(&self, message: M, level: MessageLevel) {
        match level {
            MessageLevel::Debug | MessageLevel::Info => {
                self.0.println(message);
            }
            MessageLevel::Warning => {
                self.0.println(style(message.as_ref()).yellow().to_string());
            }
            MessageLevel::Error => {
                self.0.println(style(message.as_ref()).red().to_string());
            }
        }
    }

    fn started_processing(&self, package: &Package) {
        let message = format!("Processing {}", package.human_name());
        self.0.set_message(message);
    }

    fn finished_processing<P: AsRef<Path>>(&self, package: &Package, _deb_path: P) {
        self.0.inc(1);

        let message = format!("Done {}", package.human_name());
        self.0.set_message(message);
    }

    fn finished_all(&self) {
        self.0.finish_and_clear();

        unsafe {
            if let Some(progress_bar) = PROGRESS_BAR {
                let progress_bar: *mut Self = progress_bar as *const _ as *mut _;
                let progress_bar = Box::from_raw(progress_bar);
                drop(progress_bar);

                PROGRESS_BAR = None;
            }
        }
    }
}
