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

use crate::{
    ffi::package::TwPackage,
    package::Package,
    progress::{MessageLevel, Progress},
};
use safer_ffi::{derive_ReprC, prelude::c_slice::Raw, slice::Ref};

#[derive_ReprC]
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum TwMessageLevel {
    Debug,
    Info,
    Warning,
    Error,
}

impl From<MessageLevel> for TwMessageLevel {
    fn from(level: MessageLevel) -> Self {
        match level {
            MessageLevel::Debug => Self::Debug,
            MessageLevel::Info => Self::Info,
            MessageLevel::Warning => Self::Warning,
            MessageLevel::Error => Self::Error,
        }
    }
}

#[derive_ReprC]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TwProgressFunctions {
    print_message: Option<unsafe extern "C" fn(message: Raw<u8>, level: TwMessageLevel)>,
    started_processing: Option<unsafe extern "C" fn(package: *const TwPackage)>,
    finished_processing: Option<unsafe extern "C" fn(package: *const TwPackage)>,
    finished_all: Option<unsafe extern "C" fn()>,
}

#[derive(Copy, Clone)]
pub(crate) struct TwProgressImpl {
    pub(crate) functions: TwProgressFunctions,
}

impl Progress for TwProgressImpl {
    fn print_message<M: AsRef<str>>(&self, message: M, level: MessageLevel) {
        if let Some(func) = self.functions.print_message {
            let message = Ref::from(message.as_ref().as_bytes());
            let message = Raw::from(message);
            unsafe { func(message, level.into()) };
        }
    }

    fn started_processing(&self, package: &Package) {
        if let Some(func) = self.functions.started_processing {
            let package = TwPackage::from(package);
            unsafe { func(std::ptr::addr_of!(package)) };
        }
    }

    fn finished_processing(&self, package: &Package) {
        if let Some(func) = self.functions.finished_processing {
            let package = TwPackage::from(package);
            unsafe { func(std::ptr::addr_of!(package)) };
        }
    }

    fn finished_all(&self) {
        if let Some(func) = self.functions.finished_all {
            unsafe { func() };
        }
    }
}
