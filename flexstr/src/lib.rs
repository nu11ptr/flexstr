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

pub use crate::string::std_str::{
    BoxedStr, BoxedStrRef, FlexStr, LocalStr, LocalStrRef, SharedStr, SharedStrRef, EMPTY,
};
pub use crate::traits::{FlexStrCore, FlexStrCoreRef};

/// Provides support for [BStr](bstr::BStr)-based [FlexBStr] strings
#[cfg(feature = "bstr")]
#[cfg_attr(docsrs, doc(cfg(feature = "bstr")))]
pub mod b_str {
    pub use crate::string::b_str::{
        BoxedBStr, BoxedBStrRef, FlexBStr, LocalBStr, LocalBStrRef, SharedBStr, SharedBStrRef,
    };
}

/// Provides support for [CStr](std::ffi::CStr)-based [FlexCStr] strings
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod c_str {
    pub use crate::string::c_str::{
        BoxedCStr, BoxedCStrRef, CStrNullError, FlexCStr, LocalCStr, LocalCStrRef, SharedCStr,
        SharedCStrRef, EMPTY,
    };
}

/// Provides support for [OsStr](std::ffi::OsStr)-based [FlexOsStr] strings
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod os_str {
    pub use crate::string::os_str::{
        BoxedOsStr, BoxedOsStrRef, FlexOsStr, LocalOsStr, LocalOsStrRef, SharedOsStr,
        SharedOsStrRef,
    };
}

/// Provides support for [Path](std::path::Path)-based [FlexPath] strings
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod path {
    pub use crate::string::path::{
        BoxedPath, BoxedPathRef, FlexPath, LocalPath, LocalPathRef, SharedPath, SharedPathRef,
    };
}

/// Provides support for raw [\[u8\]](slice)-based [FlexRawStr] strings
pub mod raw_str {
    pub use crate::string::raw_str::{
        BoxedRawStr, BoxedRawStrRef, FlexRawStr, LocalRawStr, LocalRawStrRef, SharedRawStr,
        SharedRawStrRef, EMPTY,
    };
}

use crate::custom::BAD_SIZE_OR_ALIGNMENT;
use crate::storage::{BorrowStr, HeapStr, InlineStr, Storage, StorageType};
use crate::string::Str;
