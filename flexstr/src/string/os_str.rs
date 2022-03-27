#![cfg(feature = "std")]

use alloc::rc::Rc;
use alloc::sync::Arc;
use core::convert::Infallible;
use core::mem;
use std::ffi::{OsStr, OsString};

use paste::paste;

use crate::string::Str;
use crate::{define_flex_types, impl_flex_str, BorrowStr, FlexStrInner};

#[cfg(unix)]
const RAW_EMPTY: &[u8] = b"";

impl Str for OsStr {
    type StringType = OsString;
    type StoredType = u8;
    type ConvertError = Infallible;

    #[cfg(unix)]
    #[inline]
    fn from_stored_data(bytes: &[Self::StoredType]) -> &Self {
        use std::os::unix::ffi::OsStrExt;
        OsStr::from_bytes(bytes)
    }

    #[cfg(not(unix))]
    #[inline]
    fn from_stored_data(_bytes: &[Self::StoredType]) -> &Self {
        // TODO: Does os_str_bytes have a feature to help with this? Didn't see one
        unreachable!("Raw byte slice conversion not supported on this platform");
    }

    #[cfg(unix)]
    #[inline]
    fn try_from_raw_data(bytes: &[u8]) -> Result<&Self, Self::ConvertError> {
        Ok(Self::from_stored_data(bytes))
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
            Some(Self::from_stored_data(RAW_EMPTY))
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

    #[cfg(unix)]
    #[inline]
    fn as_pointer(&self) -> *const Self::StoredType {
        use std::os::unix::ffi::OsStrExt;
        self.as_bytes() as *const [Self::StoredType] as *const Self::StoredType
    }

    #[cfg(not(unix))]
    #[inline]
    fn as_pointer(&self) -> *const Self::StoredType {
        // TODO: Does os_str_bytes have a feature to help with this? Didn't see one
        unreachable!("Conversion back to raw byte slice not support on this platform");
    }
}

define_flex_types!("Os", OsStr);

impl_flex_str!(FlexOsStr, OsStr);

impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexOsStr<'str, SIZE, BPAD, HPAD, HEAP>
{
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
        Self::from_static(OsStr::from_stored_data(s))
    }
}
