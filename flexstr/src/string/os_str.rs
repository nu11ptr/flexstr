#![cfg(feature = "std")]

use alloc::rc::Rc;
use alloc::sync::Arc;
use std::ffi::{OsStr, OsString};

use crate::string::Str;
use crate::{FlexStr, TwoWordHeapStringSize};

impl Str for OsStr {
    type StringType = OsString;
    type InlineType = u8;

    #[inline]
    unsafe fn from_raw_data(_bytes: &[Self::InlineType]) -> &Self {
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

/// A flexible base string type that transparently wraps a string literal, inline string, or a custom `HEAP` type
///
/// # Note
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexStr](crate::FlexStr)
///
/// # Note 2
/// Custom concrete types need to specify a `HEAP` type with an exact size of two machine words (16 bytes
/// on 64-bit, and 8 bytes on 32-bit). Any other sized parameter will result in a runtime panic on string
/// creation.
pub type FlexOsStrBase<HEAP> = FlexStr<'static, TwoWordHeapStringSize, HEAP, OsStr>;

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
pub type FlexOsStrRefBase<'str, HEAP> = FlexStr<'str, TwoWordHeapStringSize, HEAP, OsStr>;

/// A flexible string type that transparently wraps a string literal, inline string, or an [`Rc<OsStr>`]
///
/// # Note
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexStr](crate::FlexStr)
pub type LocalOsStr = FlexOsStrBase<Rc<OsStr>>;

/// A flexible string type that transparently wraps a string literal, inline string, or an [`Arc<OsStr>`]
///
/// # Note
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexStr](crate::FlexStr)
pub type SharedOsStr = FlexOsStrBase<Arc<OsStr>>;

/// A flexible string type that transparently wraps a string literal, inline string, [`Rc<OsStr>`], or
/// borrowed string (with appropriate lifetime)
///
/// # Note
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexStr]
pub type LocalOsStrRef<'str> = FlexOsStrRefBase<'str, Rc<OsStr>>;

/// A flexible string type that transparently wraps a string literal, inline string, [`Arc<OsStr>`], or
/// borrowed string (with appropriate lifetime)
///
/// # Note
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexStr]
pub type SharedOsStrRef<'str> = FlexOsStrRefBase<'str, Arc<OsStr>>;

/// A flexible string type that transparently wraps a string literal, inline string, or a [`Box<OsStr>`]
///
/// # Note
/// This type is included for convenience for those who need wrapped [`Box<OsStr>`] support. Those who
/// do not have this special use case are encouraged to use [LocalOsStr] or [SharedOsStr] for much better
/// clone performance (without copy or additional allocation)
///
/// # Note 2
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexStr]
pub type BoxedOsStr = FlexOsStrBase<Box<OsStr>>;

/// A flexible string type that transparently wraps a string literal, inline string, [`Box<OsStr>`], or
/// borrowed string (with appropriate lifetime)
///
/// # Note
/// This type is included for convenience for those who need wrapped [`Box<OsStr>`] support. Those who
/// do not have this special use case are encouraged to use [LocalOsStr] or [SharedOsStr] for much better
/// clone performance (without copy or additional allocation)
///
/// # Note 2
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexStr]
pub type BoxedOsStrRef<'str> = FlexOsStrRefBase<'str, Box<OsStr>>;
