// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use core::fmt;
use serde::{de, ser};
use strum::Display;

#[cfg(feature = "std")]
use thiserror::Error;

#[cfg(not(feature = "std"))]
use alloc::string::{String, ToString};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, Display, Eq, PartialEq)]
#[cfg_attr(feature = "std", derive(Error))]
pub enum Error {
    #[strum(to_string = "unexpected end of input")]
    Eof,
    #[strum(to_string = "I/O error: {0}")]
    Io(String),
    #[strum(to_string = "exceeded max sequence length: {0}")]
    ExceededMaxLen(usize),
    #[strum(to_string = "exceeded max container depth while entering: {0}")]
    ExceededContainerDepthLimit(&'static str),
    #[strum(to_string = "expected boolean")]
    ExpectedBoolean,
    #[strum(to_string = "expected map key")]
    ExpectedMapKey,
    #[strum(to_string = "expected map value")]
    ExpectedMapValue,
    #[strum(to_string = "keys of serialized maps must be unique and in increasing order")]
    NonCanonicalMap,
    #[strum(to_string = "expected option type")]
    ExpectedOption,
    #[strum(to_string = "{0}")]
    Custom(String),
    #[strum(to_string = "sequence missing length")]
    MissingLen,
    #[strum(to_string = "not supported: {0}")]
    NotSupported(&'static str),
    #[strum(to_string = "remaining input")]
    RemainingInput,
    #[strum(to_string = "malformed utf8")]
    Utf8,
    #[strum(to_string = "ULEB128 encoding was not minimal in size")]
    NonCanonicalUleb128Encoding,
    #[strum(to_string = "ULEB128-encoded integer did not fit in the target size")]
    IntegerOverflowDuringUleb128Decoding,
}

#[cfg(feature = "std")]
impl From<crate::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        if err.kind() == crate::io::ErrorKind::UnexpectedEof {
            Error::Eof
        } else {
            Error::Io(err.to_string())
        }
    }
}

#[cfg(not(feature = "std"))]
impl From<crate::io::Error> for Error {
    fn from(err: crate::io::Error) -> Self {
        if err.kind() == crate::io::ErrorKind::UnexpectedEof {
            Error::Eof
        } else {
            Error::Io(err.to_string())
        }
    }
}

#[cfg(feature = "std")]
impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}

#[cfg(not(feature = "std"))]
impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}

#[cfg(feature = "std")]
impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}

#[cfg(not(feature = "std"))]
impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}

#[cfg(not(feature = "std"))]
impl de::StdError for Error {}
