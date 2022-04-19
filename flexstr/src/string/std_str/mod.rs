mod impls;

use core::str;
use core::str::Utf8Error;

pub use self::impls::*;
use crate::string::Str;

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
}
