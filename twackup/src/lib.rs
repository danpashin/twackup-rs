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

#![deny(rust_2018_idioms, clippy::pedantic, unreachable_pub)]
#![warn(clippy::unused_self)]
// #![warn(missing_docs, clippy::missing_docs_in_private_items)]

pub mod builder;
pub mod compression;
pub mod dpkg;
pub mod error;
pub mod package;
pub mod parser;
pub mod progress;
pub mod repository;
