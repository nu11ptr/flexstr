#![cfg(feature = "bstr")]

mod impls;

use alloc::borrow::Cow;
use core::convert::Infallible;

use bstr::{BStr, BString, ByteSlice};

pub use self::impls::*;
use crate::inner::FlexStrInner;
use crate::string::{Str, Utf8Error};

const RAW_EMPTY: &[u8] = b"";

impl Str for BStr {
    type StringType = BString;
    type HeapType = [u8];
    type ConvertError = Infallible;

    #[inline]
    fn from_inline_data(bytes: &[u8]) -> &Self {
        bytes.into()
    }

    #[inline]
    fn from_heap_data(bytes: &Self::HeapType) -> &Self {
        Self::from_inline_data(bytes)
    }

    #[inline]
    fn try_from_raw_data(bytes: &[u8]) -> Result<&Self, Self::ConvertError> {
        Ok(Self::from_inline_data(bytes))
    }

    #[inline(always)]
    fn empty(&self) -> Option<&'static Self> {
        if self.length() == 0 {
            Some(Self::from_inline_data(RAW_EMPTY))
        } else {
            None
        }
    }

    #[inline(always)]
    fn length(&self) -> usize {
        self.len()
    }

    #[inline]
    fn as_heap_type(&self) -> &Self::HeapType {
        self
    }

    #[inline(always)]
    fn as_inline_ptr(&self) -> *const u8 {
        self.as_ptr()
    }

    #[inline]
    fn to_string_type(&self) -> Self::StringType {
        self.into()
    }

    #[inline(always)]
    fn try_to_str(&self) -> Result<&str, Utf8Error> {
        self.to_str().map_err(|err| Utf8Error::WithData {
            valid_up_to: err.valid_up_to(),
            error_len: err.error_len(),
        })
    }

    #[inline(always)]
    fn to_string_lossy(&self) -> Cow<str> {
        self.to_str_lossy()
    }
}

impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexBStr<'str, SIZE, BPAD, HPAD, HEAP>
{
    /// Creates a wrapped static string literal from a raw byte slice.
    #[inline]
    pub fn from_static_raw(s: &'static [u8]) -> Self {
        // There are no `const fn` functions in BStr to do this so we use trait
        Self(FlexStrInner::from_static(BStr::from_inline_data(s)))
    }
}
