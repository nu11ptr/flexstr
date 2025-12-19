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
#[cfg(all(feature = "std", feature = "osstr"))]
/// Module for `OsStr`-based strings
mod osstr;
#[cfg(all(feature = "std", feature = "path"))]
/// Module for `Path`-based strings
mod path;
#[cfg(feature = "str")]
/// Module for `str`-based strings
mod str;

pub use inline::{INLINE_CAPACITY, InlineFlexStr};

#[cfg(feature = "bytes")]
pub use bytes::{InlineBytes, LocalBytes, SharedBytes};
#[cfg(feature = "cstr")]
pub use cstr::{InlineCStr, LocalCStr, SharedCStr};
#[cfg(all(feature = "std", feature = "osstr"))]
pub use osstr::{InlineOsStr, LocalOsStr, SharedOsStr};
#[cfg(all(feature = "std", feature = "path"))]
pub use path::{InlinePath, LocalPath, SharedPath};
#[cfg(feature = "str")]
pub use str::{InlineStr, LocalStr, SharedStr};

use alloc::borrow::Cow;
#[cfg(feature = "cstr")]
use alloc::ffi::CString;
#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(not(feature = "std"))]
use alloc::{borrow::ToOwned, boxed::Box};
use alloc::{rc::Rc, sync::Arc};
#[cfg(feature = "cstr")]
use core::ffi::CStr;
use core::fmt;
use core::ops::Deref;
#[cfg(all(feature = "std", feature = "osstr"))]
use std::ffi::{OsStr, OsString};
#[cfg(all(feature = "std", feature = "path"))]
use std::path::{Path, PathBuf};

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// *** Macros ***

macro_rules! partial_eq_impl {
    ($type:ty, $str_type:ty) => {
        impl<'s, S: ?Sized + StringToFromBytes, R: RefCounted<S>> PartialEq<$type>
            for FlexStr<'s, S, R>
        where
            S: PartialEq<$str_type>,
        {
            fn eq(&self, other: &$type) -> bool {
                S::eq(self, other)
            }
        }

        impl<'s, S: ?Sized + StringToFromBytes, R: RefCounted<S>> PartialEq<FlexStr<'s, S, R>>
            for $type
        where
            S: PartialEq<$str_type>,
        {
            fn eq(&self, other: &FlexStr<'s, S, R>) -> bool {
                S::eq(other, self)
            }
        }
    };
}

pub(crate) use partial_eq_impl;

macro_rules! ref_counted_mut_impl {
    ($str_type:ty) => {
        // NOTE: Cannot be implemented generically because CloneToUninit is needed
        // as a bound to `S`, but is unstable.
        impl RefCountedMut<$str_type> for Arc<$str_type> {
            #[inline]
            fn to_mut(&mut self) -> &mut $str_type {
                Arc::make_mut(self)
            }

            #[inline]
            fn as_mut(&mut self) -> &mut $str_type {
                // PANIC SAFETY: We only use this when we know the Arc is newly created
                Arc::get_mut(self).expect("Arc is shared")
            }
        }

        // NOTE: Cannot be implemented generically because CloneToUninit is needed
        // as a bound to `S`, but is unstable.
        impl RefCountedMut<$str_type> for Rc<$str_type> {
            #[inline]
            fn to_mut(&mut self) -> &mut $str_type {
                Rc::make_mut(self)
            }

            #[inline]
            fn as_mut(&mut self) -> &mut $str_type {
                // PANIC SAFETY: We only use this when we know the Rc is newly created
                Rc::get_mut(self).expect("Rc is shared")
            }
        }
    };
}

pub(crate) use ref_counted_mut_impl;

// *** StringToFromBytes ***

/// Trait for string types that can be converted to and from bytes
pub trait StringToFromBytes: ToOwned + 'static {
    /// Convert bytes to a string type
    fn bytes_as_self(bytes: &[u8]) -> &Self;

    /// Convert a string type to bytes (excludes nul for CStr)
    #[inline]
    fn self_as_bytes(&self) -> &[u8] {
        self.self_as_raw_bytes()
    }

    /// Convert a string type to raw bytes (inludes nul for CStr)
    fn self_as_raw_bytes(&self) -> &[u8];
}

/// Marker trait for string types that don't provide conversion from bytes to mutable string reference
pub trait ImmutableBytes: StringToFromBytes {}

/// Trait for string types that can be converted from bytes to mutable string reference
pub trait StringFromBytesMut: StringToFromBytes {
    /// Convert bytes to a mutable string reference
    fn bytes_as_self_mut(bytes: &mut [u8]) -> &mut Self;
}

// *** RefCounted ***

/// Trait for storage that can be reference counted
pub trait RefCounted<S: ?Sized + StringToFromBytes>:
    Deref<Target = S> + for<'a> From<&'a S> + Clone
{
}

impl<S, R> RefCounted<S> for R
where
    R: Deref<Target = S> + for<'a> From<&'a S> + Clone,
    S: ?Sized + StringToFromBytes,
{
}

/// Trait for storage that can be reference counted and mutable
pub trait RefCountedMut<S: ?Sized + StringToFromBytes>: RefCounted<S> {
    /// Borrow the string as a mutable string reference, allocating and copying first, if needed.
    fn to_mut(&mut self) -> &mut S;

    /// Borrow the string as a mutable string reference. It will panic if the string is shared.
    fn as_mut(&mut self) -> &mut S;
}

// *** StringLike ***

/// Trait for string types that provide various operations
pub trait StringLike<S: ?Sized + StringToFromBytes>
where
    Self: Sized,
{
    /// Borrow a string reference as `&S`
    fn as_ref_type(&self) -> &S;

    /// Borrow the string as bytes
    fn as_bytes(&self) -> &[u8];

    /// Consume a string and convert it to an owned string. `S::to_owned` is called on Borrowed/Inlined/RefCounted variants.
    /// Boxed variants are converted directly into `S::Owned` (most likely without copy or allocation).
    fn into_owned_type(self) -> S::Owned
    where
        S::Owned: From<Box<S>>;

    /// Convert a string reference to an owned string. `S::to_owned` is called on all variants.
    fn to_owned_type(&self) -> S::Owned;

    /// Returns true if this is an empty string
    fn is_empty(&self) -> bool {
        self.as_bytes().is_empty()
    }

    /// Returns the length of this string in bytes
    fn len(&self) -> usize {
        self.as_bytes().len()
    }

    /// Borrow the string as an `&str`
    fn as_str(&self) -> &str
    where
        S: AsRef<str>,
    {
        self.as_ref_type().as_ref()
    }

    #[cfg(all(feature = "std", feature = "osstr"))]
    /// Borrow the string as an `&OsStr`
    fn as_os_str(&self) -> &OsStr
    where
        S: AsRef<OsStr>,
    {
        self.as_ref_type().as_ref()
    }

    #[cfg(all(feature = "std", feature = "path"))]
    /// Borrow the string as a `&Path`
    fn as_path(&self) -> &Path
    where
        S: AsRef<Path>,
    {
        self.as_ref_type().as_ref()
    }

    #[cfg(feature = "cstr")]
    /// Borrow the string as a `&CStr`
    fn as_c_str(&self) -> &CStr
    where
        S: AsRef<CStr>,
    {
        self.as_ref_type().as_ref()
    }

    /// Consume a string and convert it to a [String]
    fn into_string(self) -> String
    where
        S::Owned: Into<String> + From<Box<S>>,
    {
        self.into_owned_type().into()
    }

    #[cfg(all(feature = "std", feature = "osstr"))]
    /// Consume a string and convert it to an [OsString]
    fn into_os_string(self) -> OsString
    where
        S::Owned: Into<OsString> + From<Box<S>>,
    {
        self.into_owned_type().into()
    }

    #[cfg(all(feature = "std", feature = "path"))]
    /// Consume a string and convert it to a [PathBuf]
    fn into_path_buf(self) -> PathBuf
    where
        S::Owned: Into<PathBuf> + From<Box<S>>,
    {
        self.into_owned_type().into()
    }

    #[cfg(feature = "cstr")]
    /// Consume a string and convert it to a [CString]
    fn into_c_string(self) -> CString
    where
        S::Owned: Into<CString> + From<Box<S>>,
    {
        self.into_owned_type().into()
    }

    #[cfg(feature = "bytes")]
    /// Consume a string and convert it to a [`Vec<u8>`]
    fn into_vec_bytes(self) -> Vec<u8>
    where
        S::Owned: Into<Vec<u8>> + From<Box<S>>,
    {
        self.into_owned_type().into()
    }

    /// Convert a string reference to a [String]
    fn to_string(&self) -> String
    where
        S::Owned: Into<String>,
    {
        self.to_owned_type().into()
    }

    #[cfg(all(feature = "std", feature = "osstr"))]
    /// Convert a string reference to an [OsString]
    fn to_os_string(&self) -> OsString
    where
        S::Owned: Into<OsString>,
    {
        self.to_owned_type().into()
    }

    #[cfg(all(feature = "std", feature = "path"))]
    /// Convert a string reference to a [PathBuf]
    fn to_path_buf(&self) -> PathBuf
    where
        S::Owned: Into<PathBuf>,
    {
        self.to_owned_type().into()
    }

    #[cfg(feature = "cstr")]
    /// Convert a string reference to a [CString]
    fn to_c_string(&self) -> CString
    where
        S::Owned: Into<CString>,
    {
        self.to_owned_type().into()
    }

    #[cfg(feature = "bytes")]
    /// Convert a string reference to a [`Vec<u8>`]
    fn to_vec_bytes(&self) -> Vec<u8>
    where
        S::Owned: Into<Vec<u8>>,
    {
        self.to_owned_type().into()
    }
}

// *** FlexStr ***

#[doc(alias = "SharedStr")]
#[doc(alias = "LocalStr")]
#[doc(alias = "SharedOsStr")]
#[doc(alias = "LocalOsStr")]
#[doc(alias = "SharedPath")]
#[doc(alias = "LocalPath")]
#[doc(alias = "SharedCStr")]
#[doc(alias = "LocalCStr")]
#[doc(alias = "SharedBytes")]
#[doc(alias = "LocalBytes")]
/// Flexible string type that can store a borrowed string, an inline string, a reference counted string, or a boxed string
#[derive(Debug)]
pub enum FlexStr<'s, S: ?Sized + StringToFromBytes, R: RefCounted<S>> {
    /// Borrowed string - borrowed strings are imported as `&S`
    Borrowed(&'s S),
    /// Inline string - owned strings that are small enough to be stored inline
    Inlined(InlineFlexStr<S>),
    /// Reference counted string - owned strings that are too large for inline storage
    RefCounted(R),
    /// Boxed string - heap allocated strings are imported as `Box<S>`
    Boxed(Box<S>),
}

impl<'s, S: ?Sized + StringToFromBytes, R: RefCounted<S>> FlexStr<'s, S, R>
where
    for<'a> &'a S: Default,
{
    /// Create a new empty string. This is a Borrowed variant.
    pub fn empty() -> FlexStr<'s, S, R> {
        FlexStr::Borrowed(Default::default())
    }
}

impl<'s, S: ?Sized + StringToFromBytes, R: RefCounted<S>> FlexStr<'s, S, R> {
    fn copy(&self) -> FlexStr<'s, S, R> {
        match self {
            FlexStr::Borrowed(s) => FlexStr::Borrowed(s),
            FlexStr::Inlined(s) => FlexStr::Inlined(*s),
            FlexStr::RefCounted(s) => FlexStr::RefCounted(s.clone()),
            FlexStr::Boxed(s) => FlexStr::copy_into_owned(s),
        }
    }
}

impl<'s, S: ?Sized + StringToFromBytes, R: RefCounted<S>> FlexStr<'s, S, R>
where
    Box<S>: From<S::Owned>,
{
    /// Create a new string from an owned string (most likely without copy or allocation).
    /// The result is a Boxed variant.
    pub fn from_owned(s: S::Owned) -> FlexStr<'static, S, R> {
        FlexStr::Boxed(s.into())
    }
}

impl<'s, S: ?Sized + StringToFromBytes, R: RefCounted<S>> FlexStr<'s, S, R> {
    /// Create a new string from a borrowed string. This is a const fn because it does not allocate
    /// and results in a Borrowed variant.
    pub const fn from_borrowed(s: &'s S) -> FlexStr<'s, S, R> {
        FlexStr::Borrowed(s)
    }

    /// Create a new string from an inline string. This results in an Inlined variant.
    pub fn from_inline(s: InlineFlexStr<S>) -> FlexStr<'s, S, R> {
        FlexStr::Inlined(s)
    }

    /// Create a new string from a reference counted string. This results in a RefCounted variant.
    pub fn from_ref_counted(s: R) -> FlexStr<'s, S, R> {
        FlexStr::RefCounted(s)
    }

    /// Create a new string from a boxed string. This results in a Boxed variant.
    pub fn from_boxed(s: Box<S>) -> FlexStr<'s, S, R> {
        FlexStr::Boxed(s)
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
        let bytes = S::self_as_raw_bytes(s);

        if bytes.len() <= INLINE_CAPACITY {
            FlexStr::Inlined(InlineFlexStr::from_bytes(bytes))
        } else {
            FlexStr::RefCounted(s.into())
        }
    }

    /// Optimize the string variant. This is a no-op for Inlined variants. Borrowed strings will
    /// attempt to inline, but otherwise be left as borrowed. RefCounted strings will attempt to
    /// inline, but otherwise be left as ref counted. Boxed strings will attempt to inline, but
    /// otherwise be converted to a ref counted string.
    pub fn optimize(self) -> FlexStr<'s, S, R> {
        match self {
            // Borrowed and inlined strings are already optimized
            orig @ FlexStr::Borrowed(_) | orig @ FlexStr::Inlined(_) => orig,
            // There is probably a reason this is ref counted, but we can try to inline it
            FlexStr::RefCounted(s) => {
                let bytes = S::self_as_raw_bytes(&s);

                if bytes.len() <= INLINE_CAPACITY {
                    FlexStr::Inlined(InlineFlexStr::from_bytes(bytes))
                } else {
                    FlexStr::RefCounted(s)
                }
            }
            // This should be inlined or ref counted
            FlexStr::Boxed(s) => {
                let bytes = S::self_as_raw_bytes(&s);

                if bytes.len() <= INLINE_CAPACITY {
                    FlexStr::Inlined(InlineFlexStr::from_bytes(bytes))
                } else {
                    FlexStr::RefCounted((&*s).into())
                }
            }
        }
    }

    /// Convert a string reference to an owned string. Inlined/RefCounted variants are cloned,
    /// Borrowed/Boxed variants are copied into a new Inlined or RefCounted owned string.
    pub fn to_owned(&self) -> FlexStr<'static, S, R> {
        match self {
            FlexStr::Borrowed(s) => FlexStr::copy_into_owned(s),
            FlexStr::Inlined(s) => FlexStr::Inlined(*s),
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
    pub fn as_ref_type(&self) -> &S {
        match self {
            FlexStr::Borrowed(s) => s,
            FlexStr::Inlined(s) => s,
            FlexStr::RefCounted(s) => s,
            FlexStr::Boxed(s) => s,
        }
    }

    /// Convert a string reference to an owned string. `S::to_owned` is called on all variants.
    pub fn to_owned_type(&self) -> S::Owned {
        match self {
            FlexStr::Borrowed(s) => <S as ToOwned>::to_owned(s),
            FlexStr::Inlined(s) => <S as ToOwned>::to_owned(s),
            FlexStr::RefCounted(s) => <S as ToOwned>::to_owned(s),
            FlexStr::Boxed(s) => <S as ToOwned>::to_owned(s),
        }
    }

    /// Borrow the string as a raw byte slice (NOTE: includes trailing NUL for CStr)
    pub fn as_raw_bytes(&self) -> &[u8] {
        match self {
            FlexStr::Borrowed(s) => S::self_as_raw_bytes(s),
            FlexStr::Inlined(s) => s.as_raw_bytes(),
            FlexStr::RefCounted(s) => S::self_as_raw_bytes(s),
            FlexStr::Boxed(s) => S::self_as_raw_bytes(s),
        }
    }

    /// Borrow the string as bytes
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            FlexStr::Borrowed(s) => S::self_as_bytes(s),
            FlexStr::Inlined(s) => s.as_bytes(),
            FlexStr::RefCounted(s) => S::self_as_bytes(s),
            FlexStr::Boxed(s) => S::self_as_bytes(s),
        }
    }
}

impl<'s, S: ?Sized + StringToFromBytes, R: RefCounted<S>> FlexStr<'s, S, R>
where
    S::Owned: From<Box<S>>,
{
    /// Consume a string and convert it to an owned string. `S::to_owned` is called on Borrowed/Inlined/RefCounted variants.
    /// Boxed variants are converted directly into `S::Owned` (most likely without copy or allocation).
    pub fn into_owned_type(self) -> S::Owned {
        match self {
            FlexStr::Borrowed(s) => <S as ToOwned>::to_owned(s),
            FlexStr::Inlined(s) => <S as ToOwned>::to_owned(&s),
            FlexStr::RefCounted(s) => <S as ToOwned>::to_owned(&s),
            FlexStr::Boxed(s) => s.into(),
        }
    }
}

// NOTE: This implementation is the "slow path" for types that are ImmutableBytes (iow, cannot mutate their raw bytes)
impl<'s, S: ?Sized + ImmutableBytes, R: RefCountedMut<S>> FlexStr<'s, S, R> {
    /// Borrow the string as a mutable string reference, converting if needed. If the string is Borrowed,
    /// it is made into a reference counted string first. RefCounted variants will allocate + copy
    /// if they are shared. In all other cases, the string is borrowed as a mutable reference
    /// directly.
    pub fn to_mut_type_fallback(&mut self) -> &mut S {
        match self {
            // Borrowed strings can't be made mutable - we need to own it first
            // ImmutableBytes strings can't mutate inlined strings, so ref count it
            FlexStr::Borrowed(s) => {
                *self = FlexStr::RefCounted((&**s).into());
                match self {
                    // We know this is brand new, so it is safe to share mutably immediately
                    FlexStr::RefCounted(s) => s.as_mut(),
                    _ => unreachable!("Unexpected variant"),
                }
            }
            // ImmutableBytes strings must be converted before being made mutable
            FlexStr::Inlined(s) => {
                *self = FlexStr::RefCounted((&**s).into());
                match self {
                    // We know this is brand new, so it is safe to share mutably immediately
                    FlexStr::RefCounted(s) => s.as_mut(),
                    _ => unreachable!("Unexpected variant"),
                }
            }
            // Since this might be shared, we need to check before just sharing as mutable
            FlexStr::RefCounted(s) => s.to_mut(),
            // Boxed strings can be made mutable directly
            FlexStr::Boxed(s) => s.as_mut(),
        }
    }
}

// NOTE: This implementation is the "fast path" for types that provide direct mutable access to their bytes
impl<'s, S: ?Sized + StringFromBytesMut, R: RefCountedMut<S>> FlexStr<'s, S, R> {
    /// Borrow the string as a mutable string reference, converting if needed. If the string is borrowed,
    /// it is made into an owned string first. RefCounted variants will allocate + copy
    /// if they are shared. In all other cases, the string is borrowed as a mutable reference
    /// directly.
    pub fn to_mut_type(&mut self) -> &mut S {
        match self {
            // Borrowed strings can't be made mutable - we need to own it first
            FlexStr::Borrowed(s) => {
                *self = FlexStr::copy_into_owned(s);
                // copy_into_owned will never return a borrowed variant
                match self {
                    FlexStr::Inlined(s) => s.as_mut_type(),
                    FlexStr::RefCounted(s) => s.as_mut(),
                    _ => {
                        // PANIC SAFETY: copy_into_owned will never return a borrowed/boxed variant
                        unreachable!("Unexpected borrowed/boxed variant");
                    }
                }
            }
            // Inlined strings can be made mutable directly
            FlexStr::Inlined(s) => s.as_mut_type(),
            // Since this might be shared, we need to check before just sharing as mutable
            FlexStr::RefCounted(s) => s.to_mut(),
            // Boxed strings can be made mutable directly
            FlexStr::Boxed(s) => s.as_mut(),
        }
    }
}

impl<'s, S: ?Sized + StringToFromBytes> FlexStr<'s, S, Arc<S>>
where
    Arc<S>: for<'a> From<&'a S>,
    Rc<S>: for<'a> From<&'a S>,
{
    /// Convert a shared string reference to a local string. The Borrowed/Inlined variants are copied,
    /// RefCounted is copied into a new allocation, and Boxed is copied into an Inlined or RefCounted variant.
    pub fn to_local(&self) -> FlexStr<'s, S, Rc<S>> {
        match self {
            FlexStr::Borrowed(s) => FlexStr::Borrowed(s),
            FlexStr::Inlined(s) => FlexStr::Inlined(*s),
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

impl<'s, S: ?Sized + StringToFromBytes> FlexStr<'s, S, Rc<S>>
where
    Rc<S>: for<'a> From<&'a S>,
    Arc<S>: for<'a> From<&'a S>,
{
    /// Convert a local string reference to a shared string. The Borrowed/Inlined variants are copied,
    /// RefCounted is copied into a new allocation, and Boxed is copied into an Inlined or RefCounted variant.
    pub fn to_shared(&self) -> FlexStr<'s, S, Arc<S>> {
        match self {
            FlexStr::Borrowed(s) => FlexStr::Borrowed(s),
            FlexStr::Inlined(s) => FlexStr::Inlined(*s),
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

// *** StringLike ***

impl<S: ?Sized + StringToFromBytes, R: RefCounted<S>> StringLike<S> for FlexStr<'_, S, R> {
    fn as_ref_type(&self) -> &S {
        <Self>::as_ref_type(self)
    }

    fn as_bytes(&self) -> &[u8] {
        <Self>::as_bytes(self)
    }

    fn into_owned_type(self) -> S::Owned
    where
        S::Owned: From<Box<S>>,
    {
        <Self>::into_owned_type(self)
    }

    fn to_owned_type(&self) -> S::Owned {
        <Self>::to_owned_type(self)
    }
}

// *** Default ***

impl<'s, S: ?Sized + StringToFromBytes, R: RefCounted<S>> Default for FlexStr<'s, S, R>
where
    for<'a> &'a S: Default,
{
    /// Create a new string from a default value
    fn default() -> FlexStr<'s, S, R> {
        FlexStr::empty()
    }
}

// *** From ***

impl<'s, S: ?Sized + StringToFromBytes, R: RefCounted<S>> From<&'s S> for FlexStr<'s, S, R> {
    fn from(s: &'s S) -> Self {
        FlexStr::from_borrowed(s)
    }
}

// NOTE: Could not be implemented more generically because of From<S::Owned>
impl<'s, S: ?Sized + StringToFromBytes> From<Rc<S>> for FlexStr<'s, S, Rc<S>>
where
    Rc<S>: for<'a> From<&'a S>,
{
    fn from(s: Rc<S>) -> Self {
        FlexStr::from_ref_counted(s)
    }
}

// NOTE: Could not be implemented more generically because of From<S::Owned>
impl<'s, S: ?Sized + StringToFromBytes> From<Arc<S>> for FlexStr<'s, S, Arc<S>>
where
    Arc<S>: for<'a> From<&'a S>,
{
    fn from(s: Arc<S>) -> Self {
        FlexStr::from_ref_counted(s)
    }
}

impl<'s, S: ?Sized + StringToFromBytes, R: RefCounted<S>> From<Box<S>> for FlexStr<'s, S, R> {
    fn from(s: Box<S>) -> Self {
        FlexStr::from_boxed(s)
    }
}

impl<'s, S: ?Sized + StringToFromBytes, R: RefCounted<S>> From<InlineFlexStr<S>>
    for FlexStr<'s, S, R>
{
    fn from(s: InlineFlexStr<S>) -> Self {
        FlexStr::from_inline(s)
    }
}

impl<'s, S: ?Sized + StringToFromBytes, R: RefCounted<S>> From<Cow<'s, S>> for FlexStr<'s, S, R>
where
    Box<S>: From<S::Owned>,
{
    fn from(s: Cow<'s, S>) -> Self {
        match s {
            Cow::Borrowed(s) => FlexStr::from_borrowed(s),
            Cow::Owned(s) => FlexStr::from_owned(s),
        }
    }
}

// *** Clone ***

impl<'s, S: ?Sized + StringToFromBytes, R: RefCounted<S>> Clone for FlexStr<'s, S, R> {
    fn clone(&self) -> Self {
        self.copy()
    }
}

// *** Deref<Target = S> ***

impl<'s, S: ?Sized + StringToFromBytes, R: RefCounted<S>> Deref for FlexStr<'s, S, R> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        self.as_ref_type()
    }
}

// *** Display ***

impl<'s, S: ?Sized + StringToFromBytes, R: RefCounted<S>> fmt::Display for FlexStr<'s, S, R>
where
    S: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        S::fmt(self.as_ref_type(), f)
    }
}

// *** PartialEq ***

impl<'s, S: ?Sized + StringToFromBytes, R: RefCounted<S>> PartialEq for FlexStr<'s, S, R>
where
    S: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        S::eq(self.as_ref_type(), other.as_ref_type())
    }
}

// *** Serialize ***

#[cfg(feature = "serde")]
impl<'s, S: ?Sized + StringToFromBytes, R: RefCounted<S>> Serialize for FlexStr<'s, S, R>
where
    S: Serialize,
{
    fn serialize<SER: Serializer>(&self, serializer: SER) -> Result<SER::Ok, SER::Error> {
        S::serialize(self.as_ref_type(), serializer)
    }
}

// *** Deserialize ***

#[cfg(feature = "serde")]
impl<'de, S: ?Sized + StringToFromBytes, R: RefCounted<S>> Deserialize<'de>
    for FlexStr<'static, S, R>
where
    Box<S>: Deserialize<'de>,
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // TODO: See TODO in InlineFlexStr::deserialize for more details.
        // This one isn't as egregious since Boxed isn't inherently wrong here.
        Box::deserialize(deserializer)
            .map(FlexStr::Boxed)
            .map(FlexStr::optimize)
    }
}
