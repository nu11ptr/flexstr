#![cfg(feature = "bstr")]

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::sync::Arc;
use core::convert::Infallible;
use core::mem;

use bstr::{BStr, BString};
use paste::paste;

use crate::storage::Storage;
use crate::string::Str;
use crate::{
    define_flex_types, impl_flex_str, impl_validation, BorrowStr, FlexStrInner, InlineStr,
};

const RAW_EMPTY: &[u8] = b"";

impl Str for BStr {
    type StringType = BString;
    type InlineType = u8;
    type HeapType = [u8];
    type ConvertError = Infallible;

    #[inline]
    fn from_inline_data(bytes: &[Self::InlineType]) -> &Self {
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

    #[inline]
    fn empty(&self) -> Option<&'static Self> {
        if self.length() == 0 {
            Some(Self::from_inline_data(RAW_EMPTY))
        } else {
            None
        }
    }

    #[inline]
    fn length(&self) -> usize {
        self.len()
    }

    #[inline]
    fn as_heap_type(&self) -> &Self::HeapType {
        self
    }

    #[inline]
    fn as_inline_ptr(&self) -> *const Self::InlineType {
        self.as_ptr()
    }
}

define_flex_types!("B", BStr);

impl_flex_str!(FlexBStr, BStr);

impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexBStr<'str, SIZE, BPAD, HPAD, HEAP>
{
    impl_validation!(BStr);

    /// Creates a wrapped static string literal. This function is equivalent to using the macro and
    /// is `const fn` so it can be used to initialize a constant at compile time with zero runtime cost.
    #[inline]
    pub const fn from_static(s: &'static BStr) -> Self {
        if Self::IS_VALID_SIZE {
            Self(FlexStrInner {
                static_str: mem::ManuallyDrop::new(BorrowStr::from_static(s)),
            })
        } else {
            panic!("{}", BAD_SIZE_OR_ALIGNMENT);
        }
    }

    /// Creates a wrapped static string literal from a raw byte slice.
    #[inline]
    pub fn from_static_raw(s: &'static [u8]) -> Self {
        // There are no `const fn` functions in BStr to do this so we use trait
        Self::from_static(BStr::from_inline_data(s))
    }
}
