use alloc::borrow::Cow;
use core::str::FromStr;
#[cfg(feature = "serde")]
use core::{fmt, marker::PhantomData};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

#[cfg(feature = "serde")]
use crate::inline::deserialize_too_long;
use crate::inline::{InlineFlexStr, TooLongForInlining, inline_partial_eq_impl};

use flexstr_support::StringToFromBytes;
#[cfg(feature = "serde")]
use serde::de::{Error, Unexpected, Visitor};

/// Inline `Path` type
pub type InlinePath = InlineFlexStr<Path>;

// *** TryFrom for InlineFlexStr ***

// NOTE: Cannot be implemented generically because of impl<T, U> TryFrom<U> for T where U: Into<T>
impl<'s> TryFrom<&'s Path> for InlineFlexStr<Path> {
    type Error = TooLongForInlining;

    #[inline]
    fn try_from(s: &'s Path) -> Result<Self, Self::Error> {
        InlineFlexStr::try_from_type(s)
    }
}

impl<'s> TryFrom<&'s str> for InlineFlexStr<Path> {
    type Error = TooLongForInlining;

    #[inline]
    fn try_from(s: &'s str) -> Result<Self, Self::Error> {
        InlineFlexStr::try_from_type(Path::new(s))
    }
}

impl<'s> TryFrom<&'s OsStr> for InlineFlexStr<Path> {
    type Error = TooLongForInlining;

    #[inline]
    fn try_from(s: &'s OsStr) -> Result<Self, Self::Error> {
        InlineFlexStr::try_from_type(Path::new(s))
    }
}

// *** PartialEq ***

inline_partial_eq_impl!(Path, Path);
inline_partial_eq_impl!(&Path, Path);
inline_partial_eq_impl!(PathBuf, Path);
inline_partial_eq_impl!(Cow<'_, Path>, Path);

// *** AsRef ***

impl<S: ?Sized + StringToFromBytes> AsRef<Path> for InlineFlexStr<S>
where
    S: AsRef<Path>,
{
    fn as_ref(&self) -> &Path {
        self.as_ref_type().as_ref()
    }
}

// *** FromStr ***

impl FromStr for InlineFlexStr<Path> {
    type Err = TooLongForInlining;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        InlineFlexStr::try_from_type(Path::new(s))
    }
}

// *** Deserialize ***

#[cfg(feature = "serde")]
pub(crate) struct PathVisitor<S: ?Sized>(pub(crate) PhantomData<S>);

#[cfg(feature = "serde")]
impl<'de, S: ?Sized + StringToFromBytes> Visitor<'de> for PathVisitor<S> {
    type Value = InlineFlexStr<S>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
        // The Deserialize type guard limits S to Path, which accepts UTF-8 bytes.
        InlineFlexStr::try_from_type(S::bytes_as_self(value.as_bytes()))
            .map_err(|error| deserialize_too_long(error.length))
    }

    fn visit_bytes<E: Error>(self, value: &[u8]) -> Result<Self::Value, E> {
        let value = core::str::from_utf8(value)
            .map_err(|_| E::invalid_value(Unexpected::Bytes(value), &self))?;
        self.visit_str(value)
    }
}
