#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]

//#![warn(missing_docs)]

extern crate alloc;

#[cfg(feature = "bytes")]
pub mod bytes;
#[cfg(feature = "cstr")]
pub mod cstr;
pub mod inline;
#[cfg(feature = "osstr")]
pub mod osstr;
#[cfg(feature = "path")]
pub mod path;
#[cfg(feature = "str")]
pub mod str;

#[cfg(feature = "str")]
pub use str::*;

use crate::inline::InlineBytes;
use alloc::borrow::ToOwned;

pub enum Flex<'s, T: ?Sized, S> {
    Borrowed(&'s T),
    Inlined(InlineBytes<T>),
    Stored(S),
}

impl<'s, T, S> From<&'s T> for Flex<'s, T, S> {
    fn from(s: &'s T) -> Self {
        Flex::Borrowed(s)
    }
}

impl<'s, T, S: for<'a> From<&'a T> + Clone> ToOwned for Flex<'s, T, S> {
    type Owned = Flex<'s, T, S>;

    fn to_owned(&self) -> Self::Owned {
        match self {
            Flex::Borrowed(s) => Flex::Stored(S::from(s)),
            Flex::Inlined(s) => Flex::Inlined(s.clone()),
            Flex::Stored(s) => Flex::Stored(s.clone()),
        }
    }
}
