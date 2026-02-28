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

//! A flexible, simple to use, immutable, clone-efficient [String] replacement for Rust

#[cfg(all(feature = "safe", feature = "zeroize"))]
compile_error!(
    "The `zeroize` and `safe` features are incompatible. Secure memory zeroing requires unsafe code."
);

extern crate alloc;

#[doc = include_str!("../README.md")]
mod readme_tests {}

#[cfg(feature = "bytes")]
/// Module for byte-based strings (`[u8]`)
pub mod bytes;
#[cfg(feature = "cstr")]
/// Module for `CStr`-based strings
pub mod cstr;
mod flex;
#[cfg(all(feature = "std", feature = "osstr"))]
/// Module for `OsStr`-based strings
pub mod osstr;
#[cfg(all(feature = "std", feature = "path"))]
/// Module for `Path`-based strings
pub mod path;
#[cfg(feature = "str")]
/// Module for `str`-based strings
pub mod str;

#[cfg(feature = "bytes")]
pub use bytes::{LocalBytes, SharedBytes};
#[cfg(feature = "cstr")]
pub use cstr::{LocalCStr, SharedCStr};
pub use flex::{
    FlexStr, ImmutableBytes, IntoOptimizedFlexStr, RefCounted, RefCountedMut, ToOwnedFlexStr,
};
#[cfg(feature = "cstr")]
pub use flexstr_support::InteriorNulError;
pub use flexstr_support::StringLike;
#[cfg(all(feature = "std", feature = "osstr"))]
pub use osstr::{LocalOsStr, SharedOsStr};
#[cfg(all(feature = "std", feature = "path"))]
pub use path::{LocalPath, SharedPath};
#[cfg(feature = "str")]
pub use str::{LocalStr, SharedStr};
