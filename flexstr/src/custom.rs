//! This module is useful for defining custom string types and heap storage backends

use alloc::string::String;
use core::mem;

/// Padding the size of a pointer for this platform minus one
pub const PTR_SIZED_PAD: usize = mem::size_of::<*const ()>() - 1;

/// Using this inline capacity will result in a type with the same memory size as a builtin [String]
pub const STRING_SIZED_INLINE: usize = mem::size_of::<String>() - 2;

pub use crate::storage::Storage;
pub use crate::string::Str;

pub(crate) const BAD_SIZE_OR_ALIGNMENT: &str = "OOPS! It seems you are trying to create a custom `FlexStr` but have \
violated the invariants on size and alignment. It is recommended to only try and use `FlexStr3USize` \
and pick a storage type with a size of exactly two machine words (16 bytes on 64-bit, 8 bytes on 32-bit). \
Creating a custom sized type is possible, but it is difficult to calculate alll the type parameters \
correctly and is therefore not recommended.";

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

pub use crate::string::std_str::FlexStr3USize;

/// Provides support for custom [BStr](bstr::BStr)-based [FlexBStr](crate::b_str::FlexBStr) strings
#[cfg(feature = "b_str")]
#[cfg_attr(docsrs, doc(cfg(feature = "b_str")))]
pub mod b_str {
    pub use crate::string::b_str::FlexBStr3USize;
}

/// Provides support for custom [CStr](std::ffi::CStr)-based [FlexCStr](crate::c_str::FlexCStr) strings
#[cfg(feature = "c_str")]
#[cfg_attr(docsrs, doc(cfg(feature = "c_str")))]
pub mod c_str {
    pub use crate::string::c_str::FlexCStr3USize;
}

/// Provides support for custom [OsStr](std::ffi::OsStr)-based [FlexOsStr](crate::os_str::FlexOsStr) strings
#[cfg(feature = "os_str")]
#[cfg_attr(docsrs, doc(cfg(feature = "os_str")))]
pub mod os_str {
    pub use crate::string::os_str::FlexOsStr3USize;
}

/// Provides support for custom [Path](std::path::Path)-based [FlexPath](crate::path::FlexPath) strings
#[cfg(feature = "path")]
#[cfg_attr(docsrs, doc(cfg(feature = "path")))]
pub mod path {
    pub use crate::string::path::FlexPath3USize;
}

/// Provides support for custom raw [`[u8]`](slice)-based [FlexRawStr](crate::raw_str::FlexRawStr) strings
#[cfg(feature = "raw_str")]
#[cfg_attr(docsrs, doc(cfg(feature = "raw_str")))]
pub mod raw_str {
    pub use crate::string::raw_str::FlexRawStr3USize;
}
