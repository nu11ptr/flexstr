#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(
    all(
        not(all(feature = "win_min_unsafe", target_family = "windows")),
        feature = "safe"
    ),
    forbid(unsafe_code)
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

//! A simple to use, copy/clone-efficient inline string type for Rust

extern crate alloc;

#[cfg(feature = "bytes")]
/// Module for byte-based strings (`[u8]`)
mod bytes;
#[cfg(feature = "cstr")]
/// Module for `CStr`-based strings
mod cstr;
/// Module for inline strings
mod inline;
#[cfg(all(feature = "std", feature = "osstr"))]
/// Module for `OsStr`-based strings
mod osstr;
#[cfg(all(feature = "std", feature = "path"))]
/// Module for `Path`-based strings
mod path;
#[cfg(feature = "str")]
/// Module for `str`-based strings
mod str;

#[cfg(feature = "bytes")]
pub use bytes::InlineBytes;
#[cfg(feature = "cstr")]
pub use cstr::{InlineCStr, TooLongOrNulError};
#[cfg(all(feature = "std", feature = "osstr"))]
pub use osstr::InlineOsStr;
#[cfg(all(feature = "std", feature = "path"))]
pub use path::InlinePath;
#[cfg(feature = "str")]
pub use str::{InlineStr, TooLongOrUtf8Error};

pub use inline::{INLINE_CAPACITY, InlineFlexStr, TooLongForInlining};
