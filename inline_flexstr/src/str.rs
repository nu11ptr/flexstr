use alloc::{borrow::Cow, fmt, string::String};
use core::{
    error::Error,
    str::{FromStr, Utf8Error},
};
#[cfg(feature = "std")]
use std::{ffi::OsStr, path::Path};

use crate::inline::{InlineFlexStr, TooLongForInlining, inline_partial_eq_impl};

use flexstr_support::StringToFromBytes;

/// Inline `str` type
pub type InlineStr = InlineFlexStr<str>;

// *** TooLongOrUtf8Error ***

/// Error type returned when a string is too long for inline storage or has an invalid UTF-8 sequence.
#[derive(Debug)]
pub enum TooLongOrUtf8Error {
    /// The string is too long for inline storage
    TooLong(TooLongForInlining),
    /// The string has an invalid UTF-8 sequence
    Utf8Error(Utf8Error),
}

impl fmt::Display for TooLongOrUtf8Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TooLongOrUtf8Error::TooLong(e) => e.fmt(f),
            TooLongOrUtf8Error::Utf8Error(e) => e.fmt(f),
        }
    }
}

impl Error for TooLongOrUtf8Error {}

// *** TryFrom for InlineFlexStr ***

// NOTE: Cannot be implemented generically because of impl<T, U> TryFrom<U> for T where U: Into<T>
impl<'s> TryFrom<&'s str> for InlineFlexStr<str> {
    type Error = TooLongForInlining;

    #[inline]
    fn try_from(s: &'s str) -> Result<Self, Self::Error> {
        InlineFlexStr::try_from_type(s)
    }
}

impl<'s> TryFrom<&'s [u8]> for InlineFlexStr<str> {
    type Error = TooLongOrUtf8Error;

    #[inline]
    fn try_from(s: &'s [u8]) -> Result<Self, Self::Error> {
        match str::from_utf8(s) {
            Ok(s) => InlineFlexStr::try_from_type(s).map_err(TooLongOrUtf8Error::TooLong),
            Err(e) => Err(TooLongOrUtf8Error::Utf8Error(e)),
        }
    }
}

#[cfg(feature = "std")]
impl<'s> TryFrom<&'s OsStr> for InlineFlexStr<str> {
    type Error = TooLongOrUtf8Error;

    #[inline]
    fn try_from(s: &'s OsStr) -> Result<Self, Self::Error> {
        match s.try_into() {
            Ok(s) => InlineFlexStr::try_from_type(s).map_err(TooLongOrUtf8Error::TooLong),
            Err(e) => Err(TooLongOrUtf8Error::Utf8Error(e)),
        }
    }
}

#[cfg(feature = "std")]
impl<'s> TryFrom<&'s Path> for InlineFlexStr<str> {
    type Error = TooLongOrUtf8Error;

    #[inline]
    fn try_from(s: &'s Path) -> Result<Self, Self::Error> {
        match s.as_os_str().try_into() {
            Ok(s) => InlineFlexStr::try_from_type(s).map_err(TooLongOrUtf8Error::TooLong),
            Err(e) => Err(TooLongOrUtf8Error::Utf8Error(e)),
        }
    }
}

// *** PartialEq ***

inline_partial_eq_impl!(str, str);
inline_partial_eq_impl!(&str, str);
inline_partial_eq_impl!(String, str);
inline_partial_eq_impl!(Cow<'_, str>, str);

// *** AsRef ***

impl<S: ?Sized + StringToFromBytes> AsRef<str> for InlineFlexStr<S>
where
    S: AsRef<str>,
{
    fn as_ref(&self) -> &str {
        self.as_ref_type().as_ref()
    }
}

// *** FromStr ***

impl FromStr for InlineFlexStr<str> {
    type Err = TooLongForInlining;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        InlineFlexStr::try_from_type(s)
    }
}

// *** SQLx ***

#[cfg(feature = "sqlx")]
impl<'r, DB: sqlx::Database> sqlx::Decode<'r, DB> for InlineFlexStr<str>
where
    &'r str: sqlx::Decode<'r, DB>,
{
    fn decode(
        value: <DB as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let value = <&str as sqlx::Decode<DB>>::decode(value)?;
        Ok(value.try_into()?)
    }
}

#[cfg(feature = "sqlx")]
impl<'r, DB: sqlx::Database> sqlx::Encode<'r, DB> for InlineFlexStr<str>
where
    String: sqlx::Encode<'r, DB>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <DB as sqlx::Database>::ArgumentBuffer<'r>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        // There might be a more efficient way to do this (or not?), but the lifetimes seem to be contraining
        // us to using an owned type here. Works at the cost of an allocation/copy.
        <String as sqlx::Encode<'r, DB>>::encode(self.to_string(), buf)
    }
}
