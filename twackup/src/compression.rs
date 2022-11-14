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

use bzip2::write::BzEncoder;
use flate2::write::{GzEncoder, ZlibEncoder};
use std::{
    io::{Error, Write},
    pin::Pin,
    task::{Context, Poll},
};
use tokio::io::AsyncWrite;
use twackup_derive::StrEnumWithError;
use xz2::write::XzEncoder;

#[derive(Debug, StrEnumWithError, Default, Copy, Clone)]
#[twackup(convert_all = "lower")]
pub enum CompressionType {
    Deflate,
    #[default]
    Gz,
    Bzip2,
    Xz,
}

#[derive(Debug, Default, Copy, Clone)]
pub enum CompressionLevel {
    None,
    Fast,
    #[default]
    Normal,
    Best,
    Custom(u32),
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Compression {
    pub r#type: CompressionType,
    pub level: CompressionLevel,
}

pub enum Encoder<T: Write> {
    Deflate(ZlibEncoder<T>),
    Gzip(GzEncoder<T>),
    Bzip2(BzEncoder<T>),
    Xz(XzEncoder<T>),
}

impl From<CompressionLevel> for u32 {
    fn from(level: CompressionLevel) -> u32 {
        match level {
            CompressionLevel::None => 0,
            CompressionLevel::Fast => 1,
            CompressionLevel::Normal => 6,
            CompressionLevel::Best => 9,
            CompressionLevel::Custom(custom) => custom,
        }
    }
}

impl<T: Write> Encoder<T> {
    pub fn new(inner: T, compression: Compression) -> Self {
        match compression.r#type {
            CompressionType::Gz => Self::Gzip(GzEncoder::new(
                inner,
                flate2::Compression::new(compression.level.into()),
            )),
            CompressionType::Deflate => Self::Deflate(ZlibEncoder::new(
                inner,
                flate2::Compression::new(compression.level.into()),
            )),
            CompressionType::Bzip2 => Self::Bzip2(BzEncoder::new(
                inner,
                bzip2::Compression::new(compression.level.into()),
            )),
            CompressionType::Xz => Self::Xz(XzEncoder::new(inner, compression.level.into())),
        }
    }

    pub fn into_inner(self) -> std::io::Result<T> {
        match self {
            Encoder::Deflate(inner) => inner.finish(),
            Encoder::Gzip(inner) => inner.finish(),
            Encoder::Bzip2(inner) => inner.finish(),
            Encoder::Xz(inner) => inner.finish(),
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
            Encoder::Deflate(inner) => Poll::Ready(inner.write(buf)),
            Encoder::Gzip(inner) => Poll::Ready(inner.write(buf)),
            Encoder::Bzip2(inner) => Poll::Ready(inner.write(buf)),
            Encoder::Xz(inner) => Poll::Ready(inner.write(buf)),
        }
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        let enum_self = self.get_mut();
        match enum_self {
            Encoder::Deflate(inner) => Poll::Ready(inner.flush()),
            Encoder::Gzip(inner) => Poll::Ready(inner.flush()),
            Encoder::Bzip2(inner) => Poll::Ready(inner.flush()),
            Encoder::Xz(inner) => Poll::Ready(inner.flush()),
        }
    }

    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        let enum_self = self.get_mut();
        match enum_self {
            Encoder::Deflate(inner) => Poll::Ready(inner.try_finish()),
            Encoder::Gzip(inner) => Poll::Ready(inner.try_finish()),
            Encoder::Bzip2(inner) => Poll::Ready(inner.try_finish()),
            Encoder::Xz(inner) => Poll::Ready(inner.try_finish()),
        }
    }
}
