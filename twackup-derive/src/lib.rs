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

//! Different Twackup internal derives.
//! Not intended to be used in any crate except Twackup

#![deny(rust_2018_idioms, clippy::pedantic, unreachable_pub)]

mod enum_derive;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// Creates a simple wrapper over enum
/// and implements `as_str` and `from(&str)` traits
///
/// Not intended to be used in any crate except Twackup
#[proc_macro_derive(StrEnumWithDefault, attributes(twackup))]
pub fn enum_with_default_field(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    enum_derive::with_default_field::derive(&input)
}

/// Creates a simple wrapper over enum a
/// and implements `as_str` and `try_from(&str)` traits
///
/// Not intended to be used in any crate except Twackup
#[proc_macro_derive(StrEnumWithError, attributes(twackup))]
pub fn enum_with_error(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    enum_derive::with_error::derive(&input)
}
