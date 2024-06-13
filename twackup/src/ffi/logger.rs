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
use std::sync::atomic::{AtomicBool, Ordering};

#[derive_ReprC]
#[repr(u8)]
#[derive(Copy, Clone, PartialOrd, PartialEq, Eq)]
pub enum TwMessageLevel {
    Off,
    Error,
    Warning,
    Info,
    Debug,
}

impl From<TwMessageLevel> for LevelFilter {
    fn from(level: TwMessageLevel) -> Self {
        match level {
            TwMessageLevel::Off => Self::Off,
            TwMessageLevel::Error => Self::Error,
            TwMessageLevel::Warning => Self::Warn,
            TwMessageLevel::Info => Self::Info,
            TwMessageLevel::Debug => Self::Debug,
        }
    }
}

impl From<log::Level> for TwMessageLevel {
    fn from(level: log::Level) -> Self {
        match level {
            log::Level::Error => Self::Error,
            log::Level::Warn => Self::Warning,
            log::Level::Info => Self::Info,
            log::Level::Debug | log::Level::Trace => Self::Debug,
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

static LOGGER_SETTED_UP: AtomicBool = AtomicBool::new(false);

impl Logger {
    pub(crate) fn init(functions: TwLogFunctions, level: TwMessageLevel) {
        let already_set = LOGGER_SETTED_UP.swap(true, Ordering::SeqCst);
        assert!(!already_set, "Logger is already set!");

        let logger = Self { functions, level };

        ::log::set_max_level(level.into());
        ::log::set_boxed_logger(std::boxed::Box::new(logger)).expect("Logger failed");
        LOGGER_SETTED_UP.store(true, Ordering::SeqCst);
    }

    pub(crate) fn is_already_initted() -> bool {
        LOGGER_SETTED_UP.load(Ordering::SeqCst)
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        let level: TwMessageLevel = metadata.level().into();
        level > self.level
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
        unsafe { (self.functions.log)(self.functions.context, message, record.level().into()) };
    }

    fn flush(&self) {
        unsafe { (self.functions.flush)(self.functions.context) };
    }
}
