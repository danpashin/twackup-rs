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
use safer_ffi::{derive_ReprC, prelude::c_slice::Raw, slice::Ref};
use std::{ffi::c_void, mem, ptr};

#[derive_ReprC]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TwProgressFunctions {
    did_initialize: unsafe extern "C" fn(total: u64),
    did_increment: unsafe extern "C" fn(delta: u64),
    set_message: unsafe extern "C" fn(message: Raw<u8>),
    print_message: unsafe extern "C" fn(message: Raw<u8>),
    print_warning: unsafe extern "C" fn(warning: Raw<u8>),
    print_error: unsafe extern "C" fn(error: Raw<u8>),
}

impl Default for TwProgressFunctions {
    fn default() -> Self {
        unsafe {
            Self {
                did_initialize: mem::transmute(ptr::null::<c_void>()),
                did_increment: mem::transmute(ptr::null::<c_void>()),
                set_message: mem::transmute(ptr::null::<c_void>()),
                print_message: mem::transmute(ptr::null::<c_void>()),
                print_warning: mem::transmute(ptr::null::<c_void>()),
                print_error: mem::transmute(ptr::null::<c_void>()),
            }
        }
    }
}

#[derive_ReprC]
#[repr(C)]
#[derive(Clone)]
pub struct TwProgressImpl {
    total: u64,
    functions: TwProgressFunctions,
}

impl TwProgressImpl {
    pub(crate) fn set_functions(&mut self, functions: TwProgressFunctions) {
        unsafe { (functions.did_initialize)(self.total) };
        self.functions = functions;
    }
}

impl Progress for TwProgressImpl {
    fn new(total: u64) -> Self {
        Self {
            total,
            functions: TwProgressFunctions::default(),
        }
    }

    fn increment(&self, delta: u64) {
        unsafe { (self.functions.did_increment)(delta) };
    }

    fn finish(&self) {}

    fn print<M: AsRef<str>>(&self, message: M) {
        let message = Ref::from(message.as_ref().as_bytes());
        let message = Raw::from(message);
        unsafe { (self.functions.print_message)(message) };
    }

    fn print_warning<M: AsRef<str>>(&self, message: M) {
        let message = Ref::from(message.as_ref().as_bytes());
        let message = Raw::from(message);
        unsafe { (self.functions.print_warning)(message) };
    }

    fn print_error<M: AsRef<str>>(&self, message: M) {
        let message = Ref::from(message.as_ref().as_bytes());
        let message = Raw::from(message);
        unsafe { (self.functions.print_error)(message) };
    }

    fn set_message<M: AsRef<str>>(&self, message: M) {
        let message = Ref::from(message.as_ref().as_bytes());
        let message = Raw::from(message);
        unsafe { (self.functions.set_message)(message) };
    }
}
