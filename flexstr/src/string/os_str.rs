#![cfg(feature = "std")]

use alloc::rc::Rc;
use alloc::sync::Arc;
use core::convert::Infallible;
use core::mem;
use std::ffi::{OsStr, OsString};

use paste::paste;

use crate::storage::Storage;
use crate::string::Str;
use crate::{
    define_flex_types, impl_flex_str, impl_validation, BorrowStr, FlexStrInner, InlineStr,
};

#[cfg(unix)]
const RAW_EMPTY: &[u8] = b"";

impl Str for OsStr {
    type StringType = OsString;
    type InlineType = u8;
    type HeapType = OsStr;
    type ConvertError = Infallible;

    #[cfg(unix)]
    #[inline]
    fn from_inline_data(bytes: &[Self::InlineType]) -> &Self {
        use std::os::unix::ffi::OsStrExt;
        OsStr::from_bytes(bytes)
    }

    #[cfg(not(unix))]
    #[inline]
    fn from_inline_data(_bytes: &[Self::InlineType]) -> &Self {
        // TODO: Does os_str_bytes have a feature to help with this? Didn't see one
        unreachable!("Raw byte slice conversion not supported on this platform");
    }

    #[inline]
    fn from_heap_data(bytes: &Self::HeapType) -> &Self {
        bytes
    }

    #[cfg(unix)]
    #[inline]
    fn try_from_raw_data(bytes: &[u8]) -> Result<&Self, Self::ConvertError> {
        Ok(Self::from_inline_data(bytes))
    }

    #[cfg(not(unix))]
    #[inline]
    fn try_from_raw_data(bytes: &[u8]) -> Result<&Self, Self::ConvertError> {
        // TODO: Use os_str_bytes for platforms other than unix
        unreachable!("Raw byte slice conversion not supported on this platform")
    }

    #[cfg(unix)]
    #[inline]
    fn empty(&self) -> Option<&'static Self> {
        if self.length() == 0 {
            Some(Self::from_inline_data(RAW_EMPTY))
        } else {
            None
        }
    }

    #[cfg(not(unix))]
    #[inline]
    fn empty(&self) -> Option<&'static Self> {
        None
    }

    #[inline]
    fn length(&self) -> usize {
        self.len()
    }

    #[inline]
    fn as_heap_type(&self) -> &Self::HeapType {
        self
    }

    #[cfg(unix)]
    #[inline]
    fn as_inline_ptr(&self) -> *const Self::InlineType {
        use std::os::unix::ffi::OsStrExt;
        self.as_bytes() as *const [Self::InlineType] as *const Self::InlineType
    }

    #[cfg(not(unix))]
    #[inline]
    fn as_inline_ptr(&self) -> *const Self::InlineType {
        // TODO: Does os_str_bytes have a feature to help with this? Didn't see one
        unreachable!("Conversion back to raw pointer not supported on this platform");
    }
}

define_flex_types!("Os", OsStr);

impl_flex_str!(FlexOsStr, OsStr);

impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexOsStr<'str, SIZE, BPAD, HPAD, HEAP>
{
    impl_validation!(OsStr);

    /// Creates a wrapped static string literal. This function is equivalent to using the macro and
    /// is `const fn` so it can be used to initialize a constant at compile time with zero runtime cost.
    #[inline]
    pub const fn from_static(s: &'static OsStr) -> Self {
        if Self::IS_VALID_SIZE {
            Self(FlexStrInner {
                static_str: mem::ManuallyDrop::new(BorrowStr::from_static(s)),
            })
        } else {
            panic!("{}", BAD_SIZE_OR_ALIGNMENT);
        }
    }

    /// Creates a wrapped static string literal from a raw byte slice.
    #[cfg(unix)]
    #[inline]
    pub fn from_static_raw(s: &'static [u8]) -> Self {
        // I see no mention of const fn for these functions on unix - use trait
        Self::from_static(OsStr::from_inline_data(s))
    }
}
