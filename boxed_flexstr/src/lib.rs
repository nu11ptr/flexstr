#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod boxed;
#[cfg(feature = "bytes")]
mod bytes;
#[cfg(feature = "cstr")]
mod cstr;
#[cfg(all(feature = "std", feature = "osstr"))]
mod osstr;
#[cfg(all(feature = "std", feature = "path"))]
mod path;
#[cfg(not(feature = "safe"))]
mod small_box;
#[cfg(feature = "str")]
mod str;

pub use boxed::BoxedFlexStr;
#[cfg(feature = "bytes")]
pub use bytes::BoxedBytes;
#[cfg(feature = "cstr")]
pub use cstr::BoxedCStr;
#[cfg(all(feature = "std", feature = "osstr"))]
pub use osstr::BoxedOsStr;
#[cfg(all(feature = "std", feature = "path"))]
pub use path::BoxedPath;
#[cfg(feature = "str")]
pub use str::BoxedStr;
