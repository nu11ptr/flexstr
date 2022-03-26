//! This module is useful for defining custom string types

use alloc::string::String;
use core::mem;

/// Padding the size of a pointer for this platform minus one
pub const PTR_SIZED_PAD: usize = mem::size_of::<*const ()>() - 1;

/// Using this inline capacity will result in a type with the same memory size as a builtin [String]
pub const STRING_SIZED_INLINE: usize = mem::size_of::<String>() - 2;

/// Type representing the inline storage including its size and string type. This is only used when
/// implementing [Size]
pub type InlineStorage<const N: usize, STR> = [mem::MaybeUninit<<STR as Str>::InlineType>; N];

pub use crate::string::Str;

/// Trait for defining the various sizes of all the inner union variants to ensure proper size and alignment
pub trait Size<STR>
where
    STR: Str + ?Sized,
{
    /// Pad type and size for heap union variant (Only type [Pad] supported)
    type HeapPad;
    /// Pad type and size for borrowed union variants (Only type [Pad] supported)
    type BorrowPad;
    /// Type and size used for inline strings (Only [InlineStorage] supported)
    type InlineStorage;
}

/// Type that supplies internal padding to the internal union structures. This is only needed when
/// implementing [Size]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Pad<const N: usize>([mem::MaybeUninit<u8>; N]);

impl<const N: usize> Default for Pad<N> {
    fn default() -> Self {
        // SAFETY: Padding, never actually used
        unsafe { Self(mem::MaybeUninit::uninit().assume_init()) }
    }
}

/// Predefined type implementing [Size] for string types using a two word heap type (ie [`Rc<str>`](std::rc::Rc)
/// with the same inline size as a [String]
pub struct TwoWordHeapStringSize;

impl<STR> Size<STR> for TwoWordHeapStringSize
where
    STR: Str + ?Sized,
{
    type HeapPad = Pad<PTR_SIZED_PAD>;
    type BorrowPad = Pad<PTR_SIZED_PAD>;
    type InlineStorage = InlineStorage<STRING_SIZED_INLINE, STR>;
}

pub use crate::string::std_str::{FlexStrBase, FlexStrRefBase};

/// Provides support for custom [BStr](bstr::BStr)-based [FlexStr](crate::FlexStr) strings
#[cfg(feature = "bstr")]
pub mod b_str {
    pub use crate::string::b_str::{FlexBStrBase, FlexBStrRefBase};
}

/// Provides support for custom [CStr](std::ffi::CStr)-based [FlexStr](crate::FlexStr) strings
#[cfg(feature = "std")]
pub mod c_str {
    pub use crate::string::c_str::{FlexCStrBase, FlexCStrRefBase};
}

/// Provides support for custom [OsStr](std::ffi::OsStr)-based [FlexStr](crate::FlexStr) strings
#[cfg(feature = "std")]
pub mod os_str {
    pub use crate::string::os_str::{FlexOsStrBase, FlexOsStrRefBase};
}

/// Provides support for custom raw [`[u8]`](slice)-based [FlexStr](crate::FlexStr) strings
pub mod raw_str {
    pub use crate::string::raw_str::{FlexRawStrBase, FlexRawStrRefBase};
}
