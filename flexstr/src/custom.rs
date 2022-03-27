//! This module is useful for defining custom string types

use alloc::string::String;
use core::mem;

/// Padding the size of a pointer for this platform minus one
pub const PTR_SIZED_PAD: usize = mem::size_of::<*const ()>() - 1;

/// Using this inline capacity will result in a type with the same memory size as a builtin [String]
pub const STRING_SIZED_INLINE: usize = mem::size_of::<String>() - 2;

pub use crate::string::Str;

pub(crate) const BAD_SIZE_OR_ALIGNMENT: &str = "OOPS! It seems you are trying to create a custom `FlexStr` but have \
violated the invariants on size and alignment. It is recommended to only try and use `FlexStrBase` \
and pick a storage type with a size of exactly two machine words (16 bytes on 64-bit, 8 bytes on 32-bit). \
Creating a custom type based directly on the `FlexStr` union is possible, but it is difficult to calculate \
all the type parameters correctly and is therefore not recommended.";

/// Type that supplies internal padding to the internal union structures
#[derive(Clone, Copy)]
#[repr(transparent)]
pub(crate) struct Pad<const N: usize>([mem::MaybeUninit<u8>; N]);

impl<const N: usize> Pad<N> {
    // Must be const fn since we have some spots where we need that
    #[inline]
    pub(crate) const fn new() -> Self {
        // SAFETY: Padding, never actually used
        unsafe { Self(mem::MaybeUninit::uninit().assume_init()) }
    }
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
