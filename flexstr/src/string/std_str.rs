use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::sync::Arc;
use core::mem;
use core::str;
use core::str::Utf8Error;

use paste::paste;

use crate::storage::Storage;
use crate::string::Str;
use crate::{
    define_flex_types, impl_flex_str, impl_validation, BorrowStr, FlexStrInner, InlineStr,
};

/// Empty string constant
pub const EMPTY: &str = "";

impl Str for str {
    type StringType = String;
    type InlineType = u8;
    type HeapType = [u8];
    type ConvertError = Utf8Error;

    #[inline]
    fn from_inline_data(bytes: &[Self::InlineType]) -> &Self {
        // SAFETY: This will always be previously vetted to ensure it is proper UTF8
        unsafe { core::str::from_utf8_unchecked(bytes) }
    }

    #[inline]
    fn from_heap_data(bytes: &Self::HeapType) -> &Self {
        Self::from_inline_data(bytes)
    }

    #[inline]
    fn try_from_raw_data(bytes: &[u8]) -> Result<&Self, Self::ConvertError> {
        str::from_utf8(bytes)
    }

    #[inline]
    fn empty(&self) -> Option<&'static Self> {
        if self.length() == 0 {
            Some(EMPTY)
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
        self.as_bytes()
    }

    #[inline]
    fn as_inline_ptr(&self) -> *const Self::InlineType {
        self.as_ptr()
    }
}

define_flex_types!("", str);

impl_flex_str!(FlexStr, str);

impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexStr<'str, SIZE, BPAD, HPAD, HEAP>
{
    impl_validation!(str);

    /// An empty ("") static constant string
    pub const EMPTY: Self = if Self::IS_VALID_SIZE {
        Self::from_static(EMPTY)
    } else {
        panic!("{}", BAD_SIZE_OR_ALIGNMENT);
    };

    /// Creates a wrapped static string literal. This function is equivalent to using the macro and
    /// is `const fn` so it can be used to initialize a constant at compile time with zero runtime cost.
    /// ```
    /// use flexstr::LocalStr;
    ///
    /// const S: LocalStr = LocalStr::from_static("test");
    /// assert!(S.is_static());
    /// ```
    #[inline]
    pub const fn from_static(s: &'static str) -> Self {
        if Self::IS_VALID_SIZE {
            Self(FlexStrInner {
                static_str: mem::ManuallyDrop::new(BorrowStr::from_static(s)),
            })
        } else {
            panic!("{}", BAD_SIZE_OR_ALIGNMENT);
        }
    }

    /// Tries to create a wrapped static string literal from a raw byte slice. If it is successful, a
    /// [FlexStr] will be created using static wrapped storage. If unsuccessful (because encoding is
    /// incorrect) a [Utf8Error] is returned.
    /// ```
    /// use flexstr::LocalStr;
    ///
    /// const S: &[u8] = b"test";
    /// let s = LocalStr::try_from_static_raw(S).unwrap();
    /// assert!(S.is_static());
    /// ```
    #[inline]
    pub fn try_from_static_raw(s: &'static [u8]) -> Result<Self, Utf8Error> {
        // `from_utf8` still const fn unstable - use trait for now
        let s = str::try_from_raw_data(s)?;
        Ok(Self::from_static(s))
    }
}
