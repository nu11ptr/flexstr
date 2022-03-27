use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::convert::Infallible;
use core::mem;

use paste::paste;

use crate::string::Str;
use crate::{define_flex_types, impl_flex_str, BorrowStr, FlexStrInner};

impl Str for [u8] {
    type StringType = Vec<u8>;
    type StoredType = u8;
    type ConvertError = Infallible;

    #[inline]
    fn from_stored_data(bytes: &[Self::StoredType]) -> &Self {
        bytes
    }

    #[inline]
    fn try_from_raw_data(bytes: &[u8]) -> Result<&Self, Self::ConvertError> {
        Ok(Self::from_stored_data(bytes))
    }

    #[inline]
    fn length(&self) -> usize {
        self.len()
    }

    #[inline]
    fn as_pointer(&self) -> *const Self::StoredType {
        self.as_ptr()
    }
}

define_flex_types!("Raw", [u8]);

impl_flex_str!(FlexRawStr, [u8]);

impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexRawStr<'str, SIZE, BPAD, HPAD, HEAP>
{
    /// Creates a wrapped static string literal. This function is equivalent to using the macro and
    /// is `const fn` so it can be used to initialize a constant at compile time with zero runtime cost.
    #[inline]
    pub const fn from_static(s: &'static [u8]) -> Self {
        if Self::IS_VALID_SIZE {
            Self(FlexStrInner {
                static_str: mem::ManuallyDrop::new(BorrowStr::from_static(s)),
            })
        } else {
            panic!("{}", BAD_SIZE_OR_ALIGNMENT);
        }
    }
}
