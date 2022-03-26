#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

//! A flexible, simple to use, immutable, clone-efficient [String] replacement for Rust

extern crate alloc;

pub mod custom;
mod storage;
mod string;

use core::mem;

use crate::custom::{Size, TwoWordHeapStringSize};
use crate::storage::{BorrowStr, HeapStr, InlineStr};
use crate::string::Str;

/// A flexible string type that transparently wraps a string literal, inline string, a heap allocated type,
/// or a borrowed string (with appropriate lifetime)
///
/// # Note
/// It is not generally recommended to try and create direct custom concrete types of `FlexStr` as it
/// is complicated to calculate the correct sizes of all the generic type parameters. However, be aware
/// that a runtime panic will be issued on creation if incorrect, so if you are able to create a string
/// of your custom type, your parameters were of correct size/alignment.
pub union FlexStr<'str, SIZE, HEAP, STR>
where
    STR: Str + ?Sized + 'static,
    SIZE: Size<STR>,
{
    static_str: mem::ManuallyDrop<BorrowStr<SIZE, STR, &'static STR>>,
    inline_str: mem::ManuallyDrop<InlineStr<SIZE, STR>>,
    heap_str: mem::ManuallyDrop<HeapStr<SIZE, HEAP, STR>>,
    borrow_str: mem::ManuallyDrop<BorrowStr<SIZE, STR, &'str STR>>,
}

pub use crate::string::std_str::{
    BoxedStr, BoxedStrRef, LocalStr, LocalStrRef, SharedStr, SharedStrRef,
};

/// Provides support for [BStr](bstr::BStr)-based [FlexStr] strings
#[cfg(feature = "bstr")]
pub mod b_str {
    pub use crate::string::b_str::{
        BoxedBStr, BoxedBStrRef, LocalBStr, LocalBStrRef, SharedBStr, SharedBStrRef,
    };
}

/// Provides support for [CStr](std::ffi::CStr)-based [FlexStr] strings
#[cfg(feature = "std")]
pub mod c_str {
    pub use crate::string::c_str::{
        BoxedCStr, BoxedCStrRef, LocalCStr, LocalCStrRef, SharedCStr, SharedCStrRef,
    };
}

/// Provides support for [OsStr](std::ffi::OsStr)-based [FlexStr] strings
#[cfg(feature = "std")]
pub mod os_str {
    pub use crate::string::os_str::{
        BoxedOsStr, BoxedOsStrRef, LocalOsStr, LocalOsStrRef, SharedOsStr, SharedOsStrRef,
    };
}

/// Provides support for raw [`[u8]`](slice)-based [FlexStr] strings
pub mod raw_str {
    pub use crate::string::raw_str::{
        BoxedRawStr, BoxedRawStrRef, LocalRawStr, LocalRawStrRef, SharedRawStr, SharedRawStrRef,
    };
}
