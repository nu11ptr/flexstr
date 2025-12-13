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

#[cfg(not(feature = "std"))]
use alloc::{borrow::ToOwned, boxed::Box};
use core::ops::Deref;

use crate::inline::{INLINE_CAPACITY, InlineBytes};

// *** StringOps ***

pub trait StringOps: ToOwned {
    fn bytes_as_self(bytes: &[u8]) -> &Self;

    fn self_as_bytes(&self) -> &[u8];
}

// *** RefCounted ***

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

pub enum Flex<'s, S: ?Sized + StringOps, R: RefCounted<S>> {
    Borrowed(&'s S),
    Inlined(InlineBytes),
    RefCounted(R),
    Boxed(Box<S>),
}

impl<'s, S: StringOps + 'static, R: RefCounted<S>> Flex<'s, S, R> {
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
    pub fn from_owned(s: S::Owned) -> Flex<'static, S, R> {
        Flex::Boxed(s.into())
    }
}

impl<'s, S: ?Sized + StringOps, R: RefCounted<S>> Flex<'s, S, R> {
    pub fn from_borrowed(s: &'s S) -> Flex<'s, S, R> {
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

    pub fn to_owned(&self) -> Flex<'static, S, R> {
        match self {
            Flex::Borrowed(s) => Flex::copy_into_owned(s),
            Flex::Inlined(s) => Flex::Inlined(s.clone()),
            Flex::RefCounted(s) => Flex::RefCounted(s.clone()),
            Flex::Boxed(s) => Flex::copy_into_owned(s),
        }
    }

    pub fn into_owned(self) -> Flex<'static, S, R> {
        match self {
            Flex::Borrowed(s) => Flex::copy_into_owned(s),
            Flex::Inlined(s) => Flex::Inlined(s),
            Flex::RefCounted(s) => Flex::RefCounted(s),
            Flex::Boxed(s) => Flex::Boxed(s),
        }
    }

    pub fn as_borrowed_type(&self) -> &S {
        match self {
            Flex::Borrowed(s) => s,
            Flex::Inlined(s) => S::bytes_as_self(s),
            Flex::RefCounted(s) => s,
            Flex::Boxed(s) => s,
        }
    }

    pub fn to_owned_type(&self) -> S::Owned {
        match self {
            Flex::Borrowed(s) => <S as ToOwned>::to_owned(s),
            Flex::Inlined(s) => <S as ToOwned>::to_owned(S::bytes_as_self(&s)),
            Flex::RefCounted(s) => <S as ToOwned>::to_owned(&s),
            Flex::Boxed(s) => <S as ToOwned>::to_owned(s),
        }
    }
}

impl<'s, S: StringOps, R: RefCounted<S>> Flex<'s, S, R>
where
    S::Owned: From<Box<S>>,
{
    pub fn into_owned_type(self) -> S::Owned {
        match self {
            Flex::Borrowed(s) => <S as ToOwned>::to_owned(s),
            Flex::Inlined(s) => <S as ToOwned>::to_owned(S::bytes_as_self(&s)),
            Flex::RefCounted(s) => <S as ToOwned>::to_owned(&s),
            Flex::Boxed(s) => s.into(),
        }
    }
}

// *** From<&S> ***

impl<'s, S: StringOps, R: RefCounted<S>> From<&'s S> for Flex<'s, S, R> {
    #[inline(always)]
    fn from(s: &'s S) -> Self {
        Flex::from_borrowed(s)
    }
}

// *** Clone ***

impl<'s, S: StringOps + 'static, R: RefCounted<S>> Clone for Flex<'s, S, R> {
    #[inline(always)]
    fn clone(&self) -> Self {
        self.copy()
    }
}

// *** AsRef<S> ***

impl<'s, S: StringOps, R: RefCounted<S>> AsRef<S> for Flex<'s, S, R> {
    #[inline(always)]
    fn as_ref(&self) -> &S {
        self.as_borrowed_type()
    }
}

// *** Deref<Target = S> ***

impl<'s, S: StringOps, R: RefCounted<S>> Deref for Flex<'s, S, R> {
    type Target = S;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_borrowed_type()
    }
}
