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

#![deny(
    rust_2018_idioms,
    clippy::pedantic,
    clippy::string_lit_as_bytes,
    clippy::deref_by_slicing,
    clippy::filetype_is_file,
    clippy::let_underscore_must_use,
    clippy::print_stdout,
    clippy::print_stderr,
    clippy::redundant_feature_names
)]
#![warn(clippy::unused_self, missing_docs)]
#![allow(
    clippy::expl_impl_clone_on_copy,
    clippy::uninlined_format_args,
    clippy::cast_possible_wrap
)]

//! A Tokio-based DPKG database parsing library.
//!
//! Twackup is a super-fast, reliable and can be used
//! in jailbroken iOS/macOS as well as in any other Debian-based system.
//!
//! ### Example usage
//!
//! ```no_run
//! use twackup::{Dpkg, Result};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let dpkg_dir = "/var/lib/dpkg";
//!     let should_lock_dir = false;
//!     let dpkg = Dpkg::new(dpkg_dir, should_lock_dir);
//!
//!     let return_leaves = true;
//!     let packages = dpkg.unsorted_packages(return_leaves).await?;
//!
//!     for package in packages {
//!         println!("Found package with name {:?}", package.human_name());
//!     }
//!
//!     Ok(())
//! }
//! ```

pub mod archiver;
pub mod builder;
pub(crate) mod dpkg;
mod error;
pub mod package;
mod parser;
pub mod progress;
pub mod repository;

#[cfg(feature = "ffi")]
pub mod ffi;

pub use dpkg::{Dpkg, PackagesSort};
pub use error::{Generic as GenericError, Result};
pub use parser::{Parsable, Parser};
