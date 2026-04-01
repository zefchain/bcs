// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use serde_core::{de, ser};
use std::{fmt, io::ErrorKind};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    Eof,
    Io(String),
    ExceededMaxLen(usize),
    ExceededContainerDepthLimit(&'static str),
    ExpectedBoolean,
    ExpectedMapKey,
    ExpectedMapValue,
    NonCanonicalMap,
    ExpectedOption,
    Custom(String),
    MissingLen,
    NotSupported(&'static str),
    RemainingInput,
    Utf8,
    NonCanonicalUleb128Encoding,
    IntegerOverflowDuringUleb128Decoding,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Eof => f.write_str("unexpected end of input"),
            Self::Io(err) => write!(f, "I/O error: {err}"),
            Self::ExceededMaxLen(len) => write!(f, "exceeded max sequence length: {len}"),
            Self::ExceededContainerDepthLimit(ty) => {
                write!(f, "exceeded max container depth while entering: {ty}")
            }
            Self::ExpectedBoolean => f.write_str("expected boolean"),
            Self::ExpectedMapKey => f.write_str("expected map key"),
            Self::ExpectedMapValue => f.write_str("expected map value"),
            Self::NonCanonicalMap => {
                f.write_str("keys of serialized maps must be unique and in increasing order")
            }
            Self::ExpectedOption => f.write_str("expected option type"),
            Self::Custom(msg) => f.write_str(msg),
            Self::MissingLen => f.write_str("sequence missing length"),
            Self::NotSupported(msg) => write!(f, "not supported: {msg}"),
            Self::RemainingInput => f.write_str("remaining input"),
            Self::Utf8 => f.write_str("malformed utf8"),
            Self::NonCanonicalUleb128Encoding => {
                f.write_str("ULEB128 encoding was not minimal in size")
            }
            Self::IntegerOverflowDuringUleb128Decoding => {
                f.write_str("ULEB128-encoded integer did not fit in the target size")
            }
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        if err.kind() == ErrorKind::UnexpectedEof {
            Error::Eof
        } else {
            Error::Io(err.to_string())
        }
    }
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}
