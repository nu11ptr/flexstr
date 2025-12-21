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

//! Support crate for 'flexstr' and 'inline_flexstr'

extern crate alloc;

#[cfg(feature = "bytes")]
/// Module for byte-based strings (`[u8]`)
mod bytes;
#[cfg(feature = "cstr")]
/// Module for `CStr`-based strings
mod cstr;
#[cfg(all(feature = "std", feature = "osstr"))]
/// Module for `OsStr`-based strings
mod osstr;
#[cfg(all(feature = "std", feature = "path"))]
/// Module for `Path`-based strings
mod path;
#[cfg(feature = "str")]
/// Module for `str`-based strings
mod str;
mod traits;

#[cfg(feature = "cstr")]
pub use cstr::InteriorNulError;
pub use traits::{StringFromBytesMut, StringLike, StringToFromBytes};
