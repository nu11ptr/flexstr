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

// *** FlexStr ***

/// Flexible string type that can store a borrowed string, an inline string, a reference counted string, or a boxed string
#[derive(Debug)]
pub enum FlexStr<'s, S: ?Sized + StringOps, R: RefCounted<S>> {
    /// Borrowed string - borrowed strings are imported as `&S`
    Borrowed(&'s S),
    /// Inline string - owned strings that are small enough to be stored inline
    Inlined(InlineBytes),
    /// Reference counted string - owned strings that are too large for inline storage
    RefCounted(R),
    /// Boxed string - heap allocated strings are imported as `Box<S>`
    Boxed(Box<S>),
}

impl<'s, S: ?Sized + StringOps + 'static, R: RefCounted<S>> FlexStr<'s, S, R> {
    fn copy(&self) -> FlexStr<'s, S, R> {
        match self {
            FlexStr::Borrowed(s) => FlexStr::Borrowed(s),
            FlexStr::Inlined(s) => FlexStr::Inlined(s.clone()),
            FlexStr::RefCounted(s) => FlexStr::RefCounted(s.clone()),
            FlexStr::Boxed(s) => FlexStr::copy_into_owned(s),
        }
    }
}

impl<'s, S: ?Sized + StringOps, R: RefCounted<S>> FlexStr<'s, S, R>
where
    Box<S>: From<S::Owned>,
{
    /// Create a new string from an owned string (most likely without copy or allocation).
    /// The result is a Boxed variant.
    pub fn from_owned(s: S::Owned) -> FlexStr<'static, S, R> {
        FlexStr::Boxed(s.into())
    }
}

impl<'s, S: ?Sized + StringOps, R: RefCounted<S>> FlexStr<'s, S, R> {
    /// Create a new string from a borrowed string. This is a const fn because it does not allocate
    /// and results in a Borrowed variant.
    pub const fn from_borrowed(s: &'s S) -> FlexStr<'s, S, R> {
        FlexStr::Borrowed(s)
    }

    /// Returns true if this is a borrowed string
    pub fn is_borrowed(&self) -> bool {
        matches!(self, FlexStr::Borrowed(_))
    }

    /// Returns true if this is an inlined string
    pub fn is_inlined(&self) -> bool {
        matches!(self, FlexStr::Inlined(_))
    }

    /// Returns true if this is a reference counted string
    pub fn is_ref_counted(&self) -> bool {
        matches!(self, FlexStr::RefCounted(_))
    }

    /// Returns true if this is a boxed string
    pub fn is_boxed(&self) -> bool {
        matches!(self, FlexStr::Boxed(_))
    }

    /// Returns true if this is a string that is on the heap
    pub fn is_on_heap(&self) -> bool {
        matches!(self, FlexStr::RefCounted(_) | FlexStr::Boxed(_))
    }

    /// Returns true if this is a string that is off the heap
    pub fn is_off_heap(&self) -> bool {
        matches!(self, FlexStr::Borrowed(_) | FlexStr::Inlined(_))
    }

    fn copy_into_owned(s: &S) -> FlexStr<'static, S, R> {
        let bytes = S::self_as_bytes(s);

        if bytes.len() <= INLINE_CAPACITY {
            FlexStr::Inlined(InlineBytes::from_bytes(bytes))
        } else {
            FlexStr::RefCounted(s.into())
        }
    }

    /// Convert a string reference to an owned string. Inlined/RefCounted variants are cloned,
    /// Borrowed/Boxed variants are copied into a new Inlined or RefCounted owned string.
    pub fn to_owned(&self) -> FlexStr<'static, S, R> {
        match self {
            FlexStr::Borrowed(s) => FlexStr::copy_into_owned(s),
            FlexStr::Inlined(s) => FlexStr::Inlined(s.clone()),
            FlexStr::RefCounted(s) => FlexStr::RefCounted(s.clone()),
            FlexStr::Boxed(s) => FlexStr::copy_into_owned(s),
        }
    }

    /// Consume a string and convert it to an owned string. Inlined/RefCounted/Boxed variants
    /// are moved, Borrowed variants are copied into a new Inlined or RefCounted owned string.
    pub fn into_owned(self) -> FlexStr<'static, S, R> {
        match self {
            FlexStr::Borrowed(s) => FlexStr::copy_into_owned(s),
            FlexStr::Inlined(s) => FlexStr::Inlined(s),
            FlexStr::RefCounted(s) => FlexStr::RefCounted(s),
            FlexStr::Boxed(s) => FlexStr::Boxed(s),
        }
    }

    /// Borrow a string reference as `&S`
    pub fn as_borrowed_type(&self) -> &S {
        match self {
            FlexStr::Borrowed(s) => s,
            FlexStr::Inlined(s) => S::bytes_as_self(s),
            FlexStr::RefCounted(s) => s,
            FlexStr::Boxed(s) => s,
        }
    }

    /// Convert a string reference to an owned string. `S::to_owned` is called on all variants.
    pub fn to_owned_type(&self) -> S::Owned {
        match self {
            FlexStr::Borrowed(s) => <S as ToOwned>::to_owned(s),
            FlexStr::Inlined(s) => <S as ToOwned>::to_owned(S::bytes_as_self(s)),
            FlexStr::RefCounted(s) => <S as ToOwned>::to_owned(s),
            FlexStr::Boxed(s) => <S as ToOwned>::to_owned(s),
        }
    }

    /// Borrow the string as bytes
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            FlexStr::Borrowed(s) => S::self_as_bytes(s),
            FlexStr::Inlined(s) => s,
            FlexStr::RefCounted(s) => S::self_as_bytes(s),
            FlexStr::Boxed(s) => S::self_as_bytes(s),
        }
    }
}

impl<'s, S: ?Sized + StringOps, R: RefCounted<S>> FlexStr<'s, S, R>
where
    S::Owned: From<Box<S>>,
{
    /// Consume a string and convert it to an owned string. `S::to_owned` is called on Borrowed/Inlined/RefCounted variants.
    /// Boxed variants are converted directly into `S::Owned` (most likely without copy or allocation).
    pub fn into_owned_type(self) -> S::Owned {
        match self {
            FlexStr::Borrowed(s) => <S as ToOwned>::to_owned(s),
            FlexStr::Inlined(s) => <S as ToOwned>::to_owned(S::bytes_as_self(&s)),
            FlexStr::RefCounted(s) => <S as ToOwned>::to_owned(&s),
            FlexStr::Boxed(s) => s.into(),
        }
    }
}

impl<'s, S: ?Sized + StringOps + 'static> FlexStr<'s, S, Arc<S>>
where
    Arc<S>: for<'a> From<&'a S>,
    Rc<S>: for<'a> From<&'a S>,
{
    /// Convert a shared string reference to a local string. The Borrowed/Inlined variants are copied,
    /// RefCounted is copied into a new allocation, and Boxed is copied into an Inlined or RefCounted variant.
    pub fn to_local(&self) -> FlexStr<'s, S, Rc<S>> {
        match self {
            FlexStr::Borrowed(s) => FlexStr::Borrowed(s),
            FlexStr::Inlined(s) => FlexStr::Inlined(s.clone()),
            FlexStr::RefCounted(s) => FlexStr::RefCounted(Rc::from(s)),
            FlexStr::Boxed(s) => FlexStr::copy_into_owned(s),
        }
    }

    /// Consume a shared string and convert it to a local string. The Borrowed/Inlined/Boxed variants are moved,
    /// and RefCounted is copied into a new allocation.
    pub fn into_local(self) -> FlexStr<'s, S, Rc<S>> {
        match self {
            FlexStr::Borrowed(s) => FlexStr::Borrowed(s),
            FlexStr::Inlined(s) => FlexStr::Inlined(s),
            FlexStr::RefCounted(s) => FlexStr::RefCounted(Rc::from(&s)),
            FlexStr::Boxed(s) => FlexStr::Boxed(s),
        }
    }
}

impl<'s, S: ?Sized + StringOps + 'static> FlexStr<'s, S, Rc<S>>
where
    Rc<S>: for<'a> From<&'a S>,
    Arc<S>: for<'a> From<&'a S>,
{
    /// Convert a local string reference to a shared string. The Borrowed/Inlined variants are copied,
    /// RefCounted is copied into a new allocation, and Boxed is copied into an Inlined or RefCounted variant.
    pub fn to_shared(&self) -> FlexStr<'s, S, Arc<S>> {
        match self {
            FlexStr::Borrowed(s) => FlexStr::Borrowed(s),
            FlexStr::Inlined(s) => FlexStr::Inlined(s.clone()),
            FlexStr::RefCounted(s) => FlexStr::RefCounted(Arc::from(s)),
            FlexStr::Boxed(s) => FlexStr::copy_into_owned(s),
        }
    }

    /// Consume a local string and convert it to a shared string. The Borrowed/Inlined/Boxed variants are moved,
    /// and RefCounted is copied into a new allocation.
    pub fn into_shared(self) -> FlexStr<'s, S, Arc<S>> {
        match self {
            FlexStr::Borrowed(s) => FlexStr::Borrowed(s),
            FlexStr::Inlined(s) => FlexStr::Inlined(s),
            FlexStr::RefCounted(s) => FlexStr::RefCounted(Arc::from(&s)),
            FlexStr::Boxed(s) => FlexStr::Boxed(s),
        }
    }
}

// *** From<&S> ***

impl<'s, S: ?Sized + StringOps, R: RefCounted<S>> From<&'s S> for FlexStr<'s, S, R> {
    #[inline(always)]
    fn from(s: &'s S) -> Self {
        FlexStr::from_borrowed(s)
    }
}

// *** Clone ***

impl<'s, S: ?Sized + StringOps + 'static, R: RefCounted<S>> Clone for FlexStr<'s, S, R> {
    #[inline(always)]
    fn clone(&self) -> Self {
        self.copy()
    }
}

// *** AsRef<S> ***

impl<'s, S: ?Sized + StringOps, R: RefCounted<S>> AsRef<S> for FlexStr<'s, S, R> {
    #[inline(always)]
    fn as_ref(&self) -> &S {
        self.as_borrowed_type()
    }
}

// *** Deref<Target = S> ***

impl<'s, S: ?Sized + StringOps, R: RefCounted<S>> Deref for FlexStr<'s, S, R> {
    type Target = S;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_borrowed_type()
    }
}

// *** Display ***

impl<'s, S: ?Sized + StringOps, R: RefCounted<S>> fmt::Display for FlexStr<'s, S, R>
where
    S: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        S::fmt(self.as_borrowed_type(), f)
    }
}
