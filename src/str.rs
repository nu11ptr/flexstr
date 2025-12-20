use alloc::{
    borrow::Cow,
    ffi::{CString, IntoStringError},
    fmt,
    rc::Rc,
    string::{FromUtf8Error, String},
    sync::Arc,
    vec::Vec,
};
use core::{
    convert::Infallible,
    error::Error,
    str::{FromStr, Utf8Error},
};
#[cfg(feature = "std")]
use std::{ffi::OsStr, path::Path};

use crate::{
    FlexStr, InlineFlexStr, RefCounted, RefCountedMut, StringFromBytesMut, StringToFromBytes,
    inline::{TooLongForInlining, inline_partial_eq_impl},
    partial_eq_impl, ref_counted_mut_impl,
};

/// Local `str` type (NOTE: This can't be shared between threads)
pub type LocalStr<'s> = FlexStr<'s, str, Rc<str>>;

/// Shared `str` type
pub type SharedStr<'s> = FlexStr<'s, str, Arc<str>>;

/// Inline `str` type
pub type InlineStr = InlineFlexStr<str>;

const _: () = assert!(
    size_of::<Option<LocalStr>>() <= size_of::<String>(),
    "Option<LocalStr> must be less than or equal to the size of String"
);
const _: () = assert!(
    size_of::<Option<SharedStr>>() <= size_of::<String>(),
    "Option<SharedStr> must be less than or equal to the size of String"
);

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

// *** StringToFromBytes ***

impl StringToFromBytes for str {
    #[cfg(feature = "safe")]
    #[inline]
    fn bytes_as_self(bytes: &[u8]) -> &Self {
        // PANIC SAFETY: We know the bytes are valid UTF-8
        str::from_utf8(bytes).expect("Invalid UTF-8")
    }

    #[cfg(not(feature = "safe"))]
    #[inline]
    fn bytes_as_self(bytes: &[u8]) -> &Self {
        // SAFETY: We know the bytes are valid UTF-8
        unsafe { str::from_utf8_unchecked(bytes) }
    }

    #[inline]
    fn self_as_raw_bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}

// *** StringFromBytesMut ***

impl StringFromBytesMut for str {
    #[cfg(feature = "safe")]
    #[inline]
    fn bytes_as_self_mut(bytes: &mut [u8]) -> &mut Self {
        // PANIC SAFETY: We know the bytes are valid UTF-8
        str::from_utf8_mut(bytes).expect("Invalid UTF-8")
    }

    #[cfg(not(feature = "safe"))]
    #[inline]
    fn bytes_as_self_mut(bytes: &mut [u8]) -> &mut Self {
        // SAFETY: We know the bytes are valid UTF-8
        unsafe { str::from_utf8_unchecked_mut(bytes) }
    }
}

// *** RefCountedMut ***

ref_counted_mut_impl!(str);

// *** From<String> ***

// NOTE: Cannot be implemented generically because of impl<T> From<T> for T
impl<'s, R: RefCounted<str>> From<String> for FlexStr<'s, str, R> {
    fn from(s: String) -> Self {
        FlexStr::from_owned(s)
    }
}

// *** TryFrom for FlexStr ***

impl<'s, R: RefCounted<str>> TryFrom<&'s [u8]> for FlexStr<'s, str, R> {
    type Error = Utf8Error;

    #[inline]
    fn try_from(s: &'s [u8]) -> Result<Self, Self::Error> {
        Ok(FlexStr::from_borrowed(str::from_utf8(s)?))
    }
}

#[cfg(feature = "std")]
impl<'s, R: RefCounted<str>> TryFrom<&'s OsStr> for FlexStr<'s, str, R> {
    type Error = Utf8Error;

    #[inline]
    fn try_from(s: &'s OsStr) -> Result<Self, Self::Error> {
        Ok(FlexStr::from_borrowed(s.try_into()?))
    }
}

#[cfg(feature = "std")]
impl<'s, R: RefCounted<str>> TryFrom<&'s Path> for FlexStr<'s, str, R> {
    type Error = Utf8Error;

    #[inline]
    fn try_from(s: &'s Path) -> Result<Self, Self::Error> {
        Ok(FlexStr::from_borrowed(s.as_os_str().try_into()?))
    }
}

impl<R: RefCounted<str>> TryFrom<Vec<u8>> for FlexStr<'static, str, R> {
    type Error = FromUtf8Error;

    #[inline]
    fn try_from(s: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(FlexStr::from_owned(s.try_into()?))
    }
}

impl<R: RefCounted<str>> TryFrom<CString> for FlexStr<'static, str, R> {
    type Error = IntoStringError;

    #[inline]
    fn try_from(s: CString) -> Result<Self, Self::Error> {
        Ok(FlexStr::from_owned(s.try_into()?))
    }
}

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

partial_eq_impl!(str, str);
partial_eq_impl!(&str, str);
partial_eq_impl!(String, str);
partial_eq_impl!(Cow<'s, str>, str);

inline_partial_eq_impl!(str, str);
inline_partial_eq_impl!(&str, str);
inline_partial_eq_impl!(String, str);
inline_partial_eq_impl!(Cow<'_, str>, str);

// *** AsRef ***

impl<'s, S: ?Sized + StringToFromBytes, R: RefCounted<S>> AsRef<str> for FlexStr<'s, S, R>
where
    S: AsRef<str>,
{
    fn as_ref(&self) -> &str {
        self.as_ref_type().as_ref()
    }
}

impl<S: ?Sized + StringToFromBytes> AsRef<str> for InlineFlexStr<S>
where
    S: AsRef<str>,
{
    fn as_ref(&self) -> &str {
        self.as_ref_type().as_ref()
    }
}

// *** FromStr ***

impl<R: RefCounted<str>> FromStr for FlexStr<'static, str, R> {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(FlexStr::from_borrowed(s).into_owned())
    }
}

impl FromStr for InlineFlexStr<str> {
    type Err = TooLongForInlining;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        InlineFlexStr::try_from_type(s)
    }
}
