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

//! Archiver is a async wrapper module for different types of archive
//!
//! #### Example usage
//!
//! ```no_run
//! use twackup::{archiver::{Encoder, Compression}, Result};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let compression = Compression::default();
//!     let archiver = Encoder::new(vec![], compression)?;
//!
//!     // do something with archiver as it implements AsyncWrite
//!  
//!     Ok(())
//! }
//! ```
//!

use flate2::write::GzEncoder;
use std::{
    io::{Error, Write},
    pin::Pin,
    task::{Context, Poll},
};
use tokio::{io::AsyncWrite, sync::Mutex};
use twackup_derive::StrEnumWithError;
use xz2::write::XzEncoder;
use zstd::Encoder as ZSTDEncoder;

/// Defines type of the encoder
#[derive(Debug, StrEnumWithError, Default, Copy, Clone)]
#[twackup(convert_all = "lower")]
#[non_exhaustive]
pub enum Type {
    /// Old-style Gzip type
    #[default]
    Gz,
    /// Modern-based xz type
    Xz,
    /// Super-modern and fast zstd type
    Zst,
}

/// Defines how much data encoder will compress.
/// Can be between 0 and 9
#[derive(Debug, Default, Copy, Clone)]
#[non_exhaustive]
pub enum Level {
    /// Do not perform any compression, equals to 0 level
    None,
    /// Fast but not effective by disk usage compression. Equals to 1
    Fast,
    /// Normal compression effective by CPU and disk usage. Equals to 6
    #[default]
    Normal,
    /// Best compression takes minimal disk space
    /// but it takes much more CPU and RAM usage
    Best,
    /// Custom type. Must be from 0 to 9 inclusive.
    Custom(u32),
}

/// Structure defining type and level of compression
#[derive(Debug, Default, Copy, Clone)]
#[non_exhaustive]
pub struct Compression {
    /// Type of applied compression
    pub r#type: Type,
    /// Level of applied compression
    pub level: Level,
}

/// Wrapper on underlying encoders
#[non_exhaustive]
pub enum Encoder<T: Write> {
    /// Old-style Gzip type
    Gzip(GzEncoder<T>),
    /// Modern-based xz type
    Xz(XzEncoder<T>),
    /// Super-modern and fast zstd type
    Zstd(Mutex<ZSTDEncoder<'static, T>>),
}

impl From<Level> for u32 {
    #[inline]
    fn from(level: Level) -> u32 {
        match level {
            Level::None => 0,
            Level::Fast => 1,
            Level::Normal => 6,
            Level::Best => 9,
            Level::Custom(custom) => custom,
        }
    }
}

impl<T: Write> Encoder<T> {
    /// Creates encoder with specified compression
    ///
    /// - `compression` - Structure, containing compression type and level
    ///
    /// # Errors
    /// Return error if zts compression failed
    ///
    #[inline]
    pub fn new(inner: T, compression: Compression) -> crate::error::Result<Self> {
        match compression.r#type {
            Type::Gz => Ok(Self::Gzip(GzEncoder::new(
                inner,
                flate2::Compression::new(compression.level.into()),
            ))),
            Type::Xz => Ok(Self::Xz(XzEncoder::new(inner, compression.level.into()))),
            Type::Zst => Ok(Self::Zstd(Mutex::new(ZSTDEncoder::new(inner, 0)?))),
        }
    }

    /// Consumes self and returns inner encoder
    ///
    /// # Errors
    /// Return encoder IO error if any
    #[inline]
    pub fn into_inner(self) -> std::io::Result<T> {
        match self {
            Encoder::Gzip(inner) => inner.finish(),
            Encoder::Xz(inner) => inner.finish(),
            Encoder::Zstd(inner) => {
                let inner = inner.into_inner();
                inner.finish()
            }
        }
    }
}

impl<T: Write + Unpin> AsyncWrite for Encoder<T> {
    fn poll_write(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, Error>> {
        let enum_self = self.get_mut();
        match enum_self {
            Encoder::Gzip(inner) => Poll::Ready(inner.write(buf)),
            Encoder::Xz(inner) => Poll::Ready(inner.write(buf)),
            Encoder::Zstd(inner) => {
                let inner = inner.get_mut();
                Poll::Ready(inner.write(buf))
            }
        }
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        let enum_self = self.get_mut();
        match enum_self {
            Encoder::Gzip(inner) => Poll::Ready(inner.flush()),
            Encoder::Xz(inner) => Poll::Ready(inner.flush()),
            Encoder::Zstd(inner) => {
                let inner = inner.get_mut();
                Poll::Ready(inner.flush())
            }
        }
    }

    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        let enum_self = self.get_mut();
        match enum_self {
            Encoder::Gzip(inner) => Poll::Ready(inner.try_finish()),
            Encoder::Xz(inner) => Poll::Ready(inner.try_finish()),
            Encoder::Zstd(inner) => {
                let inner = inner.get_mut();
                Poll::Ready(inner.do_finish())
            }
        }
    }
}
