#![cfg(feature = "std")]

use alloc::rc::Rc;
use alloc::sync::Arc;
use core::mem;
use std::ffi::{OsStr, OsString};

use paste::paste;

use crate::string::Str;
use crate::{define_flex_types, impl_flex_str, BorrowStr, FlexStrInner};

impl Str for OsStr {
    type StringType = OsString;
    type InlineType = u8;

    #[inline]
    fn from_raw_data(_bytes: &[Self::InlineType]) -> &Self {
        // There is no function to convert us from &[u8] to &OsStr without UB unfortunately
        unreachable!("OsStr inline deref is not supported");
    }

    #[inline]
    fn length(&self) -> usize {
        self.len()
    }

    #[inline]
    fn as_pointer(&self) -> *const Self::InlineType {
        // There is no function to convert us from &OsStr to *const u8 without UB unfortunately
        unreachable!("OsStr inline raw copy is not supported");
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
}
