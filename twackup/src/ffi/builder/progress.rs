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

#[derive_ReprC]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TwProgressFunctions {
    did_increment: unsafe extern "C" fn(delta: u64),
    set_message: unsafe extern "C" fn(message: Raw<u8>),
    print_message: unsafe extern "C" fn(message: Raw<u8>),
    print_warning: unsafe extern "C" fn(warning: Raw<u8>),
    print_error: unsafe extern "C" fn(error: Raw<u8>),
}

#[derive(Copy, Clone)]
pub(crate) struct TwProgressImpl {
    pub(crate) functions: Option<TwProgressFunctions>,
}

impl TwProgressImpl {
    fn get_functions(&self) -> &TwProgressFunctions {
        self.functions.as_ref().expect("Set functions first")
    }
}

impl Progress for TwProgressImpl {
    fn new(_total: u64) -> Self {
        Self { functions: None }
    }

    fn increment(&self, delta: u64) {
        unsafe { (self.get_functions().did_increment)(delta) };
    }

    fn finish(&self) {}

    fn print<M: AsRef<str>>(&self, message: M) {
        let message = Ref::from(message.as_ref().as_bytes());
        let message = Raw::from(message);
        unsafe { (self.get_functions().print_message)(message) };
    }

    fn print_warning<M: AsRef<str>>(&self, message: M) {
        let message = Ref::from(message.as_ref().as_bytes());
        let message = Raw::from(message);
        unsafe { (self.get_functions().print_warning)(message) };
    }

    fn print_error<M: AsRef<str>>(&self, message: M) {
        let message = Ref::from(message.as_ref().as_bytes());
        let message = Raw::from(message);
        unsafe { (self.get_functions().print_error)(message) };
    }

    fn set_message<M: AsRef<str>>(&self, message: M) {
        let message = Ref::from(message.as_ref().as_bytes());
        let message = Raw::from(message);
        unsafe { (self.get_functions().set_message)(message) };
    }
}
