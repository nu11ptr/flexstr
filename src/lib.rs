#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

//! A flexible, simple to use, immutable, clone-efficient [String] replacement for Rust

extern crate alloc;

#[doc = include_str!("../README.md")]
mod readme_tests {}

#[cfg(feature = "bytes")]
/// Module for byte-based strings (`[u8]`)
mod bytes;
#[cfg(feature = "cstr")]
/// Module for `CStr`-based strings
mod cstr;
/// Module for inline strings
mod inline;
#[cfg(feature = "osstr")]
/// Module for `OsStr`-based strings
mod osstr;
#[cfg(feature = "path")]
/// Module for `Path`-based strings
mod path;
#[cfg(feature = "str")]
/// Module for `str`-based strings
mod str;

pub use inline::{INLINE_CAPACITY, InlineBytes};

#[cfg(feature = "bytes")]
pub use bytes::{LocalBytes, SharedBytes};
#[cfg(feature = "cstr")]
pub use cstr::{LocalCStr, SharedCStr};
#[cfg(feature = "osstr")]
pub use osstr::{LocalOsStr, SharedOsStr};
#[cfg(feature = "path")]
pub use path::{LocalPath, SharedPath};
#[cfg(feature = "str")]
pub use str::{LocalStr, SharedStr};

#[cfg(not(feature = "std"))]
use alloc::{borrow::ToOwned, boxed::Box};
use alloc::{rc::Rc, sync::Arc};
use core::fmt;
use core::ops::Deref;

// *** StringOps ***

/// Trait for string types that can be converted to and from bytes
pub trait StringOps: ToOwned {
    /// Convert bytes to a string type
    fn bytes_as_self(bytes: &[u8]) -> &Self;

    /// Convert a string type to bytes
    fn self_as_bytes(&self) -> &[u8];
}

// *** RefCounted ***

/// Trait for storage that can be reference counted
pub trait RefCounted<S: ?Sized + StringOps>:
    Deref<Target = S> + for<'a> From<&'a S> + Clone
{
}

impl<S, R> RefCounted<S> for R
where
    R: Deref<Target = S> + for<'a> From<&'a S> + Clone,
    S: ?Sized + StringOps,
{
}

// *** Flex ***

/// Flexible string type that can store a borrowed string, an inline string, a reference counted string, or a boxed string
#[derive(Debug)]
pub enum Flex<'s, S: ?Sized + StringOps, R: RefCounted<S>> {
    /// Borrowed string - borrowed strings are imported as `&S`
    Borrowed(&'s S),
    /// Inline string - owned strings that are small enough to be stored inline
    Inlined(InlineBytes),
    /// Reference counted string - owned strings that are too large for inline storage
    RefCounted(R),
    /// Boxed string - heap allocated strings are imported as `Box<S>`
    Boxed(Box<S>),
}

impl<'s, S: ?Sized + StringOps + 'static, R: RefCounted<S>> Flex<'s, S, R> {
    fn copy(&self) -> Flex<'s, S, R> {
        match self {
            Flex::Borrowed(s) => Flex::Borrowed(s),
            Flex::Inlined(s) => Flex::Inlined(s.clone()),
            Flex::RefCounted(s) => Flex::RefCounted(s.clone()),
            Flex::Boxed(s) => Flex::copy_into_owned(s),
        }
    }
}

impl<'s, S: ?Sized + StringOps, R: RefCounted<S>> Flex<'s, S, R>
where
    Box<S>: From<S::Owned>,
{
    /// Create a new string from an owned string (most likely without copy or allocation).
    /// The result is a Boxed variant.
    pub fn from_owned(s: S::Owned) -> Flex<'static, S, R> {
        Flex::Boxed(s.into())
    }
}

impl<'s, S: ?Sized + StringOps, R: RefCounted<S>> Flex<'s, S, R> {
    /// Create a new string from a borrowed string. This is a const fn because it does not allocate
    /// and results in a Borrowed variant.
    pub const fn from_borrowed(s: &'s S) -> Flex<'s, S, R> {
        Flex::Borrowed(s)
    }

    fn copy_into_owned(s: &S) -> Flex<'static, S, R> {
        let bytes = S::self_as_bytes(s);

        if bytes.len() <= INLINE_CAPACITY {
            Flex::Inlined(InlineBytes::from_bytes(bytes))
        } else {
            Flex::RefCounted(s.into())
        }
    }

    /// Convert a string reference to an owned string. Inlined/RefCounted variants are cloned,
    /// Borrowed/Boxed variants are copied into a new Inlined or RefCounted owned string.
    pub fn to_owned(&self) -> Flex<'static, S, R> {
        match self {
            Flex::Borrowed(s) => Flex::copy_into_owned(s),
            Flex::Inlined(s) => Flex::Inlined(s.clone()),
            Flex::RefCounted(s) => Flex::RefCounted(s.clone()),
            Flex::Boxed(s) => Flex::copy_into_owned(s),
        }
    }

    /// Consume a string and convert it to an owned string. Inlined/RefCounted/Boxed variants
    /// are moved, Borrowed variants are copied into a new Inlined or RefCounted owned string.
    pub fn into_owned(self) -> Flex<'static, S, R> {
        match self {
            Flex::Borrowed(s) => Flex::copy_into_owned(s),
            Flex::Inlined(s) => Flex::Inlined(s),
            Flex::RefCounted(s) => Flex::RefCounted(s),
            Flex::Boxed(s) => Flex::Boxed(s),
        }
    }

    /// Borrow a string reference as `&S`
    pub fn as_borrowed_type(&self) -> &S {
        match self {
            Flex::Borrowed(s) => s,
            Flex::Inlined(s) => S::bytes_as_self(s),
            Flex::RefCounted(s) => s,
            Flex::Boxed(s) => s,
        }
    }

    /// Convert a string reference to an owned string. `S::to_owned` is called on all variants.
    pub fn to_owned_type(&self) -> S::Owned {
        match self {
            Flex::Borrowed(s) => <S as ToOwned>::to_owned(s),
            Flex::Inlined(s) => <S as ToOwned>::to_owned(S::bytes_as_self(s)),
            Flex::RefCounted(s) => <S as ToOwned>::to_owned(s),
            Flex::Boxed(s) => <S as ToOwned>::to_owned(s),
        }
    }

    /// Borrow the string as bytes
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Flex::Borrowed(s) => S::self_as_bytes(s),
            Flex::Inlined(s) => s,
            Flex::RefCounted(s) => S::self_as_bytes(s),
            Flex::Boxed(s) => S::self_as_bytes(s),
        }
    }
}

impl<'s, S: ?Sized + StringOps, R: RefCounted<S>> Flex<'s, S, R>
where
    S::Owned: From<Box<S>>,
{
    /// Consume a string and convert it to an owned string. `S::to_owned` is called on Borrowed/Inlined/RefCounted variants.
    /// Boxed variants are converted directly into `S::Owned` (most likely without copy or allocation).
    pub fn into_owned_type(self) -> S::Owned {
        match self {
            Flex::Borrowed(s) => <S as ToOwned>::to_owned(s),
            Flex::Inlined(s) => <S as ToOwned>::to_owned(S::bytes_as_self(&s)),
            Flex::RefCounted(s) => <S as ToOwned>::to_owned(&s),
            Flex::Boxed(s) => s.into(),
        }
    }
}

impl<'s, S: ?Sized + StringOps + 'static> Flex<'s, S, Arc<S>>
where
    Arc<S>: for<'a> From<&'a S>,
    Rc<S>: for<'a> From<&'a S>,
{
    /// Convert a shared string reference to a local string. The Borrowed/Inlined variants are copied,
    /// RefCounted is copied into a new allocation, and Boxed is copied into an Inlined or RefCounted variant.
    pub fn to_local(&self) -> Flex<'s, S, Rc<S>> {
        match self {
            Flex::Borrowed(s) => Flex::Borrowed(s),
            Flex::Inlined(s) => Flex::Inlined(s.clone()),
            Flex::RefCounted(s) => Flex::RefCounted(Rc::from(s)),
            Flex::Boxed(s) => Flex::copy_into_owned(s),
        }
    }

    /// Consume a shared string and convert it to a local string. The Borrowed/Inlined/Boxed variants are moved,
    /// and RefCounted is copied into a new allocation.
    pub fn into_local(self) -> Flex<'s, S, Rc<S>> {
        match self {
            Flex::Borrowed(s) => Flex::Borrowed(s),
            Flex::Inlined(s) => Flex::Inlined(s),
            Flex::RefCounted(s) => Flex::RefCounted(Rc::from(&s)),
            Flex::Boxed(s) => Flex::Boxed(s),
        }
    }
}

impl<'s, S: ?Sized + StringOps + 'static> Flex<'s, S, Rc<S>>
where
    Rc<S>: for<'a> From<&'a S>,
    Arc<S>: for<'a> From<&'a S>,
{
    /// Convert a local string reference to a shared string. The Borrowed/Inlined variants are copied,
    /// RefCounted is copied into a new allocation, and Boxed is copied into an Inlined or RefCounted variant.
    pub fn to_shared(&self) -> Flex<'s, S, Arc<S>> {
        match self {
            Flex::Borrowed(s) => Flex::Borrowed(s),
            Flex::Inlined(s) => Flex::Inlined(s.clone()),
            Flex::RefCounted(s) => Flex::RefCounted(Arc::from(s)),
            Flex::Boxed(s) => Flex::copy_into_owned(s),
        }
    }

    /// Consume a local string and convert it to a shared string. The Borrowed/Inlined/Boxed variants are moved,
    /// and RefCounted is copied into a new allocation.
    pub fn into_shared(self) -> Flex<'s, S, Arc<S>> {
        match self {
            Flex::Borrowed(s) => Flex::Borrowed(s),
            Flex::Inlined(s) => Flex::Inlined(s),
            Flex::RefCounted(s) => Flex::RefCounted(Arc::from(&s)),
            Flex::Boxed(s) => Flex::Boxed(s),
        }
    }
}

// *** From<&S> ***

impl<'s, S: ?Sized + StringOps, R: RefCounted<S>> From<&'s S> for Flex<'s, S, R> {
    #[inline(always)]
    fn from(s: &'s S) -> Self {
        Flex::from_borrowed(s)
    }
}

// *** Clone ***

impl<'s, S: ?Sized + StringOps + 'static, R: RefCounted<S>> Clone for Flex<'s, S, R> {
    #[inline(always)]
    fn clone(&self) -> Self {
        self.copy()
    }
}

// *** AsRef<S> ***

impl<'s, S: ?Sized + StringOps, R: RefCounted<S>> AsRef<S> for Flex<'s, S, R> {
    #[inline(always)]
    fn as_ref(&self) -> &S {
        self.as_borrowed_type()
    }
}

// *** Deref<Target = S> ***

impl<'s, S: ?Sized + StringOps, R: RefCounted<S>> Deref for Flex<'s, S, R> {
    type Target = S;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_borrowed_type()
    }
}

// *** Display ***

impl<'s, S: ?Sized + StringOps, R: RefCounted<S>> fmt::Display for Flex<'s, S, R>
where
    S: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        S::fmt(self.as_borrowed_type(), f)
    }
}
