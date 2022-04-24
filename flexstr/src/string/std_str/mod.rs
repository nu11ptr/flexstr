mod impls;

use alloc::borrow::Cow;
use alloc::string::{String, ToString};
use core::str;
use core::str::Utf8Error;

pub use self::impls::*;
use crate::inner::FlexStrInner;
use crate::string::Str;

/// Empty string constant
pub const EMPTY: &str = "";

impl Str for str {
    type StringType = String;
    type HeapType = [u8];
    type ConvertError = Utf8Error;

    #[inline]
    fn from_inline_data(bytes: &[u8]) -> &Self {
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

    #[inline(always)]
    fn empty(&self) -> Option<&'static Self> {
        if self.length() == 0 {
            Some(EMPTY)
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
        self.as_bytes()
    }

    #[inline(always)]
    fn as_inline_ptr(&self) -> *const u8 {
        self.as_ptr()
    }

    #[inline]
    fn to_string_type(&self) -> Self::StringType {
        self.to_string()
    }

    #[inline(always)]
    fn try_to_str(&self) -> Result<&str, crate::string::Utf8Error> {
        Ok(self)
    }

    #[inline(always)]
    fn to_string_lossy(&self) -> Cow<str> {
        Cow::Borrowed(self)
    }
}

impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexStr<'str, SIZE, BPAD, HPAD, HEAP>
{
    /// An empty ("") static constant string
    pub const EMPTY: Self = Self::from_static(EMPTY);

    /// Tries to create a wrapped static string literal from a raw byte slice. If it is successful, a
    /// [FlexStr] will be created using static wrapped storage. If unsuccessful (because encoding is
    /// incorrect) a [Utf8Error] is returned.
    /// ```
    /// use flexstr::{FlexStrCore, LocalStr};
    ///
    ///     const S: &[u8] = b"test";
    ///     let s = LocalStr::try_from_static_raw(S).unwrap();
    ///     assert!(s.is_static());
    /// ```
    #[inline]
    pub fn try_from_static_raw(s: &'static [u8]) -> Result<Self, Utf8Error> {
        // `from_utf8` still const fn unstable - use trait for now
        let s = str::try_from_raw_data(s)?;
        Ok(Self(FlexStrInner::from_static(s)))
    }
}
