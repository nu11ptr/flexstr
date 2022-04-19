#![cfg(feature = "bstr")]

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::sync::Arc;
use core::convert::Infallible;

use bstr::{BStr, BString};
use paste::paste;

use crate::inner::FlexStrInner;
use crate::string::Str;
use crate::traits::private;
use crate::traits::private::FlexStrCoreInner;
use crate::{define_flex_types, FlexStrCore, FlexStrCoreRef, Storage};

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
}

define_flex_types!("BStr", BStr, [u8]);

macro_rules! impl_body {
    () => {
        /// Creates a wrapped static string literal from a raw byte slice.
        #[inline]
        pub fn from_static_raw(s: &'static [u8]) -> Self {
            // There are no `const fn` functions in BStr to do this so we use trait
            Self(FlexStrInner::from_static(BStr::from_inline_data(s)))
        }
    };
}

// *** FlexBStr ***

impl<const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexBStr<SIZE, BPAD, HPAD, HEAP>
{
    impl_body!();
}

impl<const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexStrCore<'static, SIZE, BPAD, HPAD, HEAP, BStr> for FlexBStr<SIZE, BPAD, HPAD, HEAP>
where
    HEAP: Storage<BStr>,
{
    #[inline(always)]
    fn as_str_type(&self) -> &BStr {
        self.inner().as_str_type()
    }
}

// *** FlexBStrRef ***

impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexBStrRef<'str, SIZE, BPAD, HPAD, HEAP>
{
    impl_body!();
}

impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexStrCore<'str, SIZE, BPAD, HPAD, HEAP, BStr> for FlexBStrRef<'str, SIZE, BPAD, HPAD, HEAP>
where
    HEAP: Storage<BStr>,
{
    #[inline(always)]
    fn as_str_type(&self) -> &BStr {
        self.inner().as_str_type()
    }
}
