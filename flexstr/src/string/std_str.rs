use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::sync::Arc;

use crate::string::Str;
use crate::{FlexStr, TwoWordHeapStringSize};

impl Str for str {
    type StringType = String;
    type InlineType = u8;

    #[inline]
    unsafe fn from_raw_data(bytes: &[Self::InlineType]) -> &Self {
        core::str::from_utf8_unchecked(bytes)
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
pub type FlexStrBase<HEAP> = FlexStr<'static, TwoWordHeapStringSize, HEAP, str>;

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
pub type FlexStrRefBase<'str, HEAP> = FlexStr<'str, TwoWordHeapStringSize, HEAP, str>;

/// A flexible string type that transparently wraps a string literal, inline string, or an [`Rc<str>`]
///
/// # Note
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexStr]
pub type LocalStr = FlexStrBase<Rc<str>>;

/// A flexible string type that transparently wraps a string literal, inline string, or an [`Arc<str>`]
///
/// # Note
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexStr]
pub type SharedStr = FlexStrBase<Arc<str>>;

/// A flexible string type that transparently wraps a string literal, inline string, [`Rc<str>`], or
/// borrowed string (with appropriate lifetime)
///
/// # Note
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexStr]
pub type LocalStrRef<'str> = FlexStrRefBase<'str, Rc<str>>;

/// A flexible string type that transparently wraps a string literal, inline string, [`Arc<str>`], or
/// borrowed string (with appropriate lifetime)
///
/// # Note
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexStr]
pub type SharedStrRef<'str> = FlexStrRefBase<'str, Arc<str>>;

/// A flexible string type that transparently wraps a string literal, inline string, or a [`Box<str>`]
///
/// # Note
/// This type is included for convenience for those who need wrapped [`Box<str>`] support. Those who
/// do not have this special use case are encouraged to use [LocalStr] or [SharedStr] for much better
/// clone performance (without copy or additional allocation)
///
/// # Note 2
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexStr]
pub type BoxedStr = FlexStrBase<Box<str>>;

/// A flexible string type that transparently wraps a string literal, inline string, [`Box<str>`], or
/// borrowed string (with appropriate lifetime)
///
/// # Note
/// This type is included for convenience for those who need wrapped [`Box<str>`] support. Those who
/// do not have this special use case are encouraged to use [LocalStr] or [SharedStr] for much better
/// clone performance (without copy or additional allocation)
///
/// # Note 2
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexStr]
pub type BoxedStrRef<'str> = FlexStrRefBase<'str, Box<str>>;
