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

use crate::progress::Progress;
use ansi_term::Colour;
use indicatif::ProgressBar as ProgressBarImpl;
use std::mem;

pub(crate) static mut PROGRESS_BAR: Option<&'static ProgressBar> = None;

#[derive(Clone)]
pub(crate) struct ProgressBar(pub(crate) ProgressBarImpl);

impl ProgressBar {
    pub(crate) fn make_static(self) -> &'static Self {
        unsafe {
            if PROGRESS_BAR.is_some() {
                panic!("progress bar is already set!");
            }

            let pb = Box::leak(Box::new(self));
            PROGRESS_BAR = Some(pb);
            pb
        }
    }

    fn drop_pb(&self) {
        unsafe {
            if let Some(progress_bar) = PROGRESS_BAR {
                let progress_bar: *mut ProgressBar = mem::transmute(progress_bar);
                drop(progress_bar);

                PROGRESS_BAR = None;
            }
        }
    }
}

impl Progress for ProgressBar {
    fn new(total: u64) -> Self {
        Self(ProgressBarImpl::new(total))
    }

    fn increment(&self, delta: u64) {
        self.0.inc(delta)
    }

    fn finish(&self) {
        self.0.finish_and_clear();
        self.drop_pb();
    }

    fn print<M: AsRef<str>>(&self, message: M) {
        self.0.println(message)
    }

    fn print_warning<M: AsRef<str>>(&self, message: M) {
        self.print(Colour::Yellow.paint(message.as_ref()).to_string())
    }

    fn print_error<M: AsRef<str>>(&self, message: M) {
        self.print(Colour::Red.paint(message.as_ref()).to_string())
    }

    fn set_message<M: AsRef<str>>(&self, message: M) {
        self.0.set_message(message.as_ref().to_string())
    }
}
