#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

//! A flexible, simple to use, immutable, clone-efficient [String] replacement for Rust

extern crate alloc;

pub mod custom;
mod inner;
mod storage;
mod string;
mod traits;

pub use crate::storage::{StorageType, WrongStorageType};
pub use crate::string::std_str::{
    BoxedStr, BoxedStrRef, FlexStr, LocalStr, LocalStrRef, SharedStr, SharedStrRef, EMPTY,
};
pub use crate::string::Utf8Error;
pub use crate::traits::FlexStrCore;

/// Provides support for [BStr](bstr::BStr)-based [FlexBStr](crate::b_str::FlexBStr) strings
#[cfg(feature = "b_str")]
#[cfg_attr(docsrs, doc(cfg(feature = "b_str")))]
pub mod b_str {
    pub use crate::string::b_str::{
        BoxedBStr, BoxedBStrRef, FlexBStr, LocalBStr, LocalBStrRef, SharedBStr, SharedBStrRef,
    };
}

/// Provides support for [CStr](std::ffi::CStr)-based [FlexCStr](crate::c_str::FlexCStr) strings
#[cfg(feature = "c_str")]
#[cfg_attr(docsrs, doc(cfg(feature = "c_str")))]
pub mod c_str {
    pub use crate::string::c_str::{
        BoxedCStr, BoxedCStrRef, CStrNulError, FlexCStr, LocalCStr, LocalCStrRef, SharedCStr,
        SharedCStrRef, EMPTY,
    };
}

/// Provides support for [OsStr](std::ffi::OsStr)-based [FlexOsStr](crate::os_str::FlexOsStr) strings
#[cfg(feature = "os_str")]
#[cfg_attr(docsrs, doc(cfg(feature = "os_str")))]
pub mod os_str {
    pub use crate::string::os_str::{
        BoxedOsStr, BoxedOsStrRef, FlexOsStr, LocalOsStr, LocalOsStrRef, SharedOsStr,
        SharedOsStrRef,
    };
}

/// Provides support for [Path](std::path::Path)-based [FlexPath](crate::path::FlexPath) strings
#[cfg(feature = "path")]
#[cfg_attr(docsrs, doc(cfg(feature = "path")))]
pub mod path {
    pub use crate::string::path::{
        BoxedPath, BoxedPathRef, FlexPath, LocalPath, LocalPathRef, SharedPath, SharedPathRef,
    };
}

/// Provides support for raw [\[u8\]](slice)-based [FlexRawStr](crate::raw_str::FlexRawStr) strings
#[cfg(feature = "raw_str")]
#[cfg_attr(docsrs, doc(cfg(feature = "raw_str")))]
pub mod raw_str {
    pub use crate::string::raw_str::{
        BoxedRawStr, BoxedRawStrRef, FlexRawStr, LocalRawStr, LocalRawStrRef, SharedRawStr,
        SharedRawStrRef, EMPTY,
    };
}

use crate::custom::BAD_SIZE_OR_ALIGNMENT;
use crate::storage::{BorrowStr, HeapStr, InlineStr, Storage};
use crate::string::Str;
