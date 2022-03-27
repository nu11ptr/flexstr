use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::sync::Arc;
use core::mem;
use core::str;
use core::str::Utf8Error;

use paste::paste;

use crate::string::Str;
use crate::{define_flex_types, impl_flex_str, BorrowStr, FlexStrInner};

impl Str for str {
    type StringType = String;
    type StoredType = u8;
    type ConvertError = Utf8Error;

    #[inline]
    fn from_stored_data(bytes: &[Self::StoredType]) -> &Self {
        // SAFETY: This will always be previously vetted to ensure it is proper UTF8
        unsafe { core::str::from_utf8_unchecked(bytes) }
    }

    #[inline]
    fn try_from_raw_data(bytes: &[u8]) -> Result<&Self, Self::ConvertError> {
        str::from_utf8(bytes)
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

define_flex_types!("", str);

impl_flex_str!(FlexStr, str);

impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexStr<'str, SIZE, BPAD, HPAD, HEAP>
{
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
