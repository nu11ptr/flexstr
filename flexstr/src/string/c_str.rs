#![cfg(feature = "std")]

use alloc::rc::Rc;
use alloc::sync::Arc;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use crate::string::Str;
use crate::{FlexStr, TwoWordHeapStringSize};

impl Str for CStr {
    type StringType = CString;
    type InlineType = c_char;

    #[inline]
    unsafe fn from_raw_data(bytes: &[Self::InlineType]) -> &Self {
        Self::from_ptr(bytes as *const [Self::InlineType] as *const Self::InlineType)
    }

    #[inline]
    fn length(&self) -> usize {
        // TODO: Stdlib hints that it may change this to be non const time - might need a diff way
        // NOTE: This will include trailing null byte (this is storage, not usable chars)
        self.to_bytes_with_nul().len()
    }

    #[inline]
    fn as_pointer(&self) -> *const Self::InlineType {
        self.as_ptr()
    }
}

/// A flexible base string type that transparently wraps a string literal, inline string, or a custom `HEAP` type
///
/// # Note
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexStr](crate::FlexStr)
///
/// # Note 2
/// Custom concrete types need to specify a `HEAP` type with an exact size of two machine words (16 bytes
/// on 64-bit, and 8 bytes on 32-bit). Any other sized parameter will result in a runtime panic on string
/// creation.
pub type FlexCStrBase<HEAP> = FlexStr<'static, TwoWordHeapStringSize, HEAP, CStr>;

/// A flexible base string type that transparently wraps a string literal, inline string, a custom `HEAP` type, or
/// a borrowed string (with appropriate lifetime specified).
///
/// # Note
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexStr](crate::FlexStr)
///
/// # Note 2
/// Custom concrete types need to specify a `HEAP` type with an exact size of two machine words (16 bytes
/// on 64-bit, and 8 bytes on 32-bit). Any other sized parameter will result in a runtime panic on string
/// creation.
pub type BFlexCStrBase<'str, HEAP> = FlexStr<'str, TwoWordHeapStringSize, HEAP, CStr>;

/// A flexible string type that transparently wraps a string literal, inline string, or an [`Rc<CStr>`]
///
/// # Note
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexStr](crate::FlexStr)
pub type LocalCStr = FlexCStrBase<Rc<CStr>>;

/// A flexible string type that transparently wraps a string literal, inline string, or an [`Arc<CStr>`]
///
/// # Note
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexStr](crate::FlexStr)
pub type SharedCStr = FlexCStrBase<Arc<CStr>>;