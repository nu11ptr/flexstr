use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::sync::Arc;
use alloc::vec::Vec;

use crate::string::Str;
use crate::{FlexStr, TwoWordHeapStringSize};

impl Str for [u8] {
    type StringType = Vec<u8>;
    type InlineType = u8;

    #[inline]
    unsafe fn from_raw_data(bytes: &[Self::InlineType]) -> &Self {
        bytes
    }

    #[inline]
    fn length(&self) -> usize {
        self.len()
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
pub type FlexRawStrBase<HEAP> = FlexStr<'static, TwoWordHeapStringSize, HEAP, [u8]>;

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
pub type FlexRawStrRefBase<'str, HEAP> = FlexStr<'str, TwoWordHeapStringSize, HEAP, [u8]>;

/// A flexible string type that transparently wraps a string literal, inline string, or an [`Rc<[u8]>`]
///
/// # Note
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexStr]
pub type LocalRawStr = FlexRawStrBase<Rc<[u8]>>;

/// A flexible string type that transparently wraps a string literal, inline string, or an [`Arc<[u8]>`]
///
/// # Note
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexStr]
pub type SharedRawStr = FlexRawStrBase<Arc<[u8]>>;

/// A flexible string type that transparently wraps a string literal, inline string, [`Rc<[u8]>`], or
/// borrowed string (with appropriate lifetime)
///
/// # Note
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexStr]
pub type LocalRawStrRef<'str> = FlexRawStrRefBase<'str, Rc<[u8]>>;

/// A flexible string type that transparently wraps a string literal, inline string, [`Arc<[u8]>`], or
/// borrowed string (with appropriate lifetime)
///
/// # Note
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexStr]
pub type SharedRawStrRef<'str> = FlexRawStrRefBase<'str, Arc<[u8]>>;

/// A flexible string type that transparently wraps a string literal, inline string, or a [`Box<[u8]>`]
///
/// # Note
/// This type is included for convenience for those who need wrapped [`Box<[u8]>`] support. Those who
/// do not have this special use case are encouraged to use [LocalRawStr] or [SharedRawStr] for much better
/// clone performance (without copy or additional allocation)
///
/// # Note 2
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexStr]
pub type BoxedRawStr = FlexRawStrBase<Box<[u8]>>;

/// A flexible string type that transparently wraps a string literal, inline string, [`Box<[u8]>`], or
/// borrowed string (with appropriate lifetime)
///
/// # Note
/// This type is included for convenience for those who need wrapped [`Box<[u8]>`] support. Those who
/// do not have this special use case are encouraged to use [LocalRawStr] or [SharedRawStr] for much better
/// clone performance (without copy or additional allocation)
///
/// # Note 2
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexStr]
pub type BoxedRawStrRef<'str> = FlexRawStrRefBase<'str, Box<[u8]>>;
