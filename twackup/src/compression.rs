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
    #[default]
    Gz,
    Xz,
    Zst,
}

/// Defines how much data encoder will compress.
/// Can be between 0 and 9
#[derive(Debug, Default, Copy, Clone)]
#[non_exhaustive]
pub enum Level {
    None,
    Fast,
    #[default]
    Normal,
    Best,
    Custom(u32),
}

#[derive(Debug, Default, Copy, Clone)]
#[non_exhaustive]
pub struct Compression {
    pub r#type: Type,
    pub level: Level,
}

#[non_exhaustive]
pub enum Encoder<T: Write> {
    Gzip(GzEncoder<T>),
    Xz(XzEncoder<T>),
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
