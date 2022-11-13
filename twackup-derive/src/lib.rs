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

#![deny(rust_2018_idioms, clippy::pedantic)]

mod enum_derive;

use proc_macro::TokenStream;

#[proc_macro_derive(StrEnumWithDefault, attributes(twackup))]
pub fn enum_with_default_field(input: TokenStream) -> TokenStream {
    enum_derive::with_default_field::derive(input)
}

#[proc_macro_derive(StrEnumWithError, attributes(twackup))]
pub fn enum_with_error(input: TokenStream) -> TokenStream {
    enum_derive::with_error::derive(input)
}
