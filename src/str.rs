use alloc::{
    borrow::Cow,
    ffi::{CString, IntoStringError},
    rc::Rc,
    string::{FromUtf8Error, String},
    sync::Arc,
    vec::Vec,
};
use core::{
    convert::Infallible,
    str::{FromStr, Utf8Error},
};
#[cfg(feature = "std")]
use std::{ffi::OsStr, path::Path};

use crate::flex::{FlexStr, RefCounted, RefCountedMut, partial_eq_impl, ref_counted_mut_impl};

use flexstr_support::StringToFromBytes;

/// Local `str` type (NOTE: This can't be shared between threads)
pub type LocalStr<'s> = FlexStr<'s, str, Rc<str>>;

/// Shared `str` type
pub type SharedStr<'s> = FlexStr<'s, str, Arc<str>>;

const _: () = assert!(
    size_of::<Option<LocalStr>>() <= size_of::<String>(),
    "Option<LocalStr> must be less than or equal to the size of String"
);
const _: () = assert!(
    size_of::<Option<SharedStr>>() <= size_of::<String>(),
    "Option<SharedStr> must be less than or equal to the size of String"
);

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

// *** PartialEq ***

partial_eq_impl!(str, str);
partial_eq_impl!(&str, str);
partial_eq_impl!(String, str);
partial_eq_impl!(Cow<'s, str>, str);

// *** AsRef ***

impl<'s, S: ?Sized + StringToFromBytes, R: RefCounted<S>> AsRef<str> for FlexStr<'s, S, R>
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

// *** SQLx ***

#[cfg(feature = "sqlx")]
impl<'r, 's, DB: sqlx::Database, R: RefCounted<str>> sqlx::Decode<'r, DB> for FlexStr<'s, str, R>
where
    &'r str: sqlx::Decode<'r, DB>,
{
    fn decode(
        value: <DB as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let value = <&str as sqlx::Decode<DB>>::decode(value)?;
        let s: FlexStr<'_, str, R> = value.into();
        Ok(s.into_owned())
    }
}

#[cfg(feature = "sqlx")]
impl<'r, 's, DB: sqlx::Database, R: RefCounted<str>> sqlx::Encode<'r, DB> for FlexStr<'s, str, R>
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
