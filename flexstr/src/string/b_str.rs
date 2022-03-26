#![cfg(feature = "bstr")]

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::sync::Arc;

use bstr::{BStr, BString};

use crate::string::Str;
use crate::{FlexStr, TwoWordHeapStringSize};

impl Str for BStr {
    type StringType = BString;
    type InlineType = u8;

    #[inline]
    unsafe fn from_raw_data(bytes: &[Self::InlineType]) -> &Self {
        bytes.into()
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
pub type FlexBStrBase<HEAP> = FlexStr<'static, TwoWordHeapStringSize, HEAP, BStr>;

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
pub type FlexBStrRefBase<'str, HEAP> = FlexStr<'str, TwoWordHeapStringSize, HEAP, BStr>;

/// A flexible string type that transparently wraps a string literal, inline string, or an [`Rc<BStr>`]
///
/// # Note
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexStr]
pub type LocalBStr = FlexBStrBase<Rc<BStr>>;

/// A flexible string type that transparently wraps a string literal, inline string, or an [`Arc<BStr>`]
///
/// # Note
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexStr]
pub type SharedBStr = FlexBStrBase<Arc<BStr>>;

/// A flexible string type that transparently wraps a string literal, inline string, [`Rc<BStr>`], or
/// borrowed string (with appropriate lifetime)
///
/// # Note
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexStr]
pub type LocalBStrRef<'str> = FlexBStrRefBase<'str, Rc<BStr>>;

/// A flexible string type that transparently wraps a string literal, inline string, [`Arc<BStr>`], or
/// borrowed string (with appropriate lifetime)
///
/// # Note
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexStr]
pub type SharedBStrRef<'str> = FlexBStrRefBase<'str, Arc<BStr>>;

/// A flexible string type that transparently wraps a string literal, inline string, or a [`Box<BStr>`]
///
/// # Note
/// This type is included for convenience for those who need wrapped [`Box<BStr>`] support. Those who
/// do not have this special use case are encouraged to use [LocalBStr] or [SharedBStr] for much better
/// clone performance (without copy or additional allocation)
///
/// # Note 2
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexStr]
pub type BoxedBStr = FlexBStrBase<Box<BStr>>;

/// A flexible string type that transparently wraps a string literal, inline string, [`Box<BStr>`], or
/// borrowed string (with appropriate lifetime)
///
/// # Note
/// This type is included for convenience for those who need wrapped [`Box<BStr>`] support. Those who
/// do not have this special use case are encouraged to use [LocalBStr] or [SharedBStr] for much better
/// clone performance (without copy or additional allocation)
///
/// # Note 2
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexStr]
pub type BoxedBStrRef<'str> = FlexBStrRefBase<'str, Box<BStr>>;
