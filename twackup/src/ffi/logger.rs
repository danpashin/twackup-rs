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

use log::{LevelFilter, Log, Metadata, Record};
use safer_ffi::{derive_ReprC, prelude::c_slice::Box};
use std::ffi::c_void;

#[derive_ReprC]
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum TwMessageLevel {
    Off,
    Debug,
    Info,
    Warning,
    Error,
}

impl From<TwMessageLevel> for LevelFilter {
    fn from(level: TwMessageLevel) -> Self {
        match level {
            TwMessageLevel::Off => Self::Off,
            TwMessageLevel::Debug => Self::Debug,
            TwMessageLevel::Info => Self::Info,
            TwMessageLevel::Warning => Self::Warn,
            TwMessageLevel::Error => Self::Error,
        }
    }
}

#[derive_ReprC]
#[repr(C)]
pub struct TwLogMessage {
    text: Box<u8>,
    target: Box<u8>,
}

#[derive_ReprC]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TwLogFunctions {
    context: Option<std::ptr::NonNull<c_void>>,
    log: unsafe extern "C" fn(
        Option<std::ptr::NonNull<c_void>>,
        message: TwLogMessage,
        level: TwMessageLevel,
    ),
    flush: unsafe extern "C" fn(Option<std::ptr::NonNull<c_void>>),
}

unsafe impl Send for TwLogFunctions {}
unsafe impl Sync for TwLogFunctions {}

pub(crate) struct Logger {
    functions: TwLogFunctions,
    level: TwMessageLevel,
}

impl Logger {
    pub(crate) fn init(functions: TwLogFunctions, level: TwMessageLevel) {
        let logger = Self { functions, level };

        ::log::set_max_level(level.into());
        ::log::set_boxed_logger(std::boxed::Box::new(logger)).expect("Logger failed");
    }
}

impl Log for Logger {
    fn enabled(&self, _metadata: &Metadata<'_>) -> bool {
        true
    }

    fn log(&self, record: &Record<'_>) {
        let message = TwLogMessage {
            text: {
                let message = record.args().to_string();
                Box::from(message.into_bytes().into_boxed_slice())
            },
            target: {
                let msg = record.target().to_string();
                Box::from(msg.into_bytes().into_boxed_slice())
            },
        };
        unsafe { (self.functions.log)(self.functions.context, message, self.level) };
    }

    fn flush(&self) {
        unsafe { (self.functions.flush)(self.functions.context) };
    }
}
