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
pub type LocalStr = FlexStr<'static, str, Rc<str>>;

/// Shared `str` type
pub type SharedStr = FlexStr<'static, str, Arc<str>>;

/// Local `str` type that can optionally hold borrows (NOTE: This can't be shared between threads)
pub type LocalStrRef<'s> = FlexStr<'s, str, Rc<str>>;

/// Shared `str` type that can optionally hold borrows
pub type SharedStrRef<'s> = FlexStr<'s, str, Arc<str>>;

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

// *** Prost ***

#[cfg(feature = "prost")]
impl<R: RefCounted<str>> prost::Message for FlexStr<'static, str, R>
where
    Self: Default + core::fmt::Debug + Send + Sync,
{
    fn encode_raw(&self, buf: &mut impl prost::bytes::BufMut)
    where
        Self: Sized,
    {
        buf.put_slice(self.as_ref_type().as_bytes());
    }

    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: prost::encoding::WireType,
        buf: &mut impl prost::bytes::Buf,
        ctx: prost::encoding::DecodeContext,
    ) -> Result<(), prost::DecodeError>
    where
        Self: Sized,
    {
        prost::encoding::skip_field(wire_type, tag, buf, ctx)
    }

    fn encoded_len(&self) -> usize {
        self.as_ref_type().len()
    }

    fn clear(&mut self) {
        *self = Default::default();
    }

    fn merge(&mut self, mut buf: impl prost::bytes::Buf) -> Result<(), prost::DecodeError>
    where
        Self: Sized,
    {
        let bytes = buf.copy_to_bytes(buf.remaining());
        let s = core::str::from_utf8(&bytes)
            .map_err(|_| prost::DecodeError::new("invalid UTF-8 in string field"))?;
        *self = FlexStr::from_borrowed(s).into_owned();
        Ok(())
    }

    fn merge_length_delimited(
        &mut self,
        mut buf: impl prost::bytes::Buf,
    ) -> Result<(), prost::DecodeError>
    where
        Self: Sized,
    {
        let len = prost::encoding::decode_varint(&mut buf)? as usize;
        if buf.remaining() < len {
            return Err(prost::DecodeError::new("buffer underflow"));
        }
        self.merge(buf.take(len))
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
        // There might be a more efficient way to do this (or not?), but the lifetimes seem to be constraining
        // us to using an owned type here. Works at the cost of an allocation/copy.
        <String as sqlx::Encode<'r, DB>>::encode(self.to_string(), buf)
    }

    fn encode(
        self,
        buf: &mut <DB as sqlx::Database>::ArgumentBuffer<'r>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError>
    where
        Self: Sized,
    {
        use flexstr_support::StringLike as _;

        // This won't allocate IF this is a boxed string
        <String as sqlx::Encode<'r, DB>>::encode(self.into_string(), buf)
    }

    fn size_hint(&self) -> usize {
        self.len()
    }
}

#[cfg(feature = "sqlx")]
impl<'s, DB: sqlx::Database, R: RefCounted<str>> sqlx::Type<DB> for FlexStr<'s, str, R>
where
    str: sqlx::Type<DB>,
{
    fn type_info() -> <DB as sqlx::Database>::TypeInfo {
        <str as sqlx::Type<DB>>::type_info()
    }

    fn compatible(ty: &<DB as sqlx::Database>::TypeInfo) -> bool {
        <str as sqlx::Type<DB>>::compatible(ty)
    }
}

#[cfg(all(feature = "sqlx", feature = "sqlx_pg_arrays"))]
impl<'s, R: RefCounted<str>> sqlx::postgres::PgHasArrayType for FlexStr<'s, str, R>
where
    for<'a> &'a str: sqlx::postgres::PgHasArrayType,
{
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <&str as sqlx::postgres::PgHasArrayType>::array_type_info()
    }

    fn array_compatible(ty: &sqlx::postgres::PgTypeInfo) -> bool {
        <&str as sqlx::postgres::PgHasArrayType>::array_compatible(ty)
    }
}
