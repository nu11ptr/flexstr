use alloc::borrow::{Borrow, BorrowMut};
#[cfg(not(feature = "std"))]
use alloc::{boxed::Box, string::String};
use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut, Index, IndexMut};
use core::slice::SliceIndex;
#[cfg(feature = "std")]
use std::{io, net::ToSocketAddrs};

use crate::{StringFromBytesMut, StringLike, StringToFromBytes};

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

macro_rules! inline_partial_eq_impl {
    ($type:ty, $str_type:ty) => {
        impl<S: ?Sized + StringToFromBytes> PartialEq<$type> for InlineFlexStr<S>
        where
            S: PartialEq<$str_type>,
        {
            fn eq(&self, other: &$type) -> bool {
                S::eq(self, other)
            }
        }

        impl<S: ?Sized + StringToFromBytes> PartialEq<InlineFlexStr<S>> for $type
        where
            S: PartialEq<$str_type>,
        {
            fn eq(&self, other: &InlineFlexStr<S>) -> bool {
                S::eq(other, self)
            }
        }
    };
}

pub(crate) use inline_partial_eq_impl;

// This must be the size of the String type minus 2 bytes for the length and discriminator
/// The capacity of the [InlineFlexStr] type in bytes
pub const INLINE_CAPACITY: usize = size_of::<String>() - 2;

// *** StringTooLongForInlining ***

/// Error type returned when the string is too long for inline storage.
#[derive(Debug)]
pub struct TooLongForInlining {
    /// The length of the string
    pub length: usize,
    /// The capacity of the inline storage
    pub inline_capacity: usize,
}

impl fmt::Display for TooLongForInlining {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "string too long for inline storage: length={} inline_capacity={}",
            self.length, self.inline_capacity
        )
    }
}

impl core::error::Error for TooLongForInlining {}

// *** InlineFlexStr ***

#[doc(alias = "InlineStr")]
#[doc(alias = "InlineOsStr")]
#[doc(alias = "InlinePath")]
#[doc(alias = "InlineCStr")]
#[doc(alias = "InlineBytes")]
/// Inline bytes type - used to store small strings inline
#[derive(Debug)]
pub struct InlineFlexStr<S: ?Sized + StringToFromBytes> {
    inline: [u8; INLINE_CAPACITY],
    len: u8,
    marker: PhantomData<S>,
}

impl<S: ?Sized + StringToFromBytes> InlineFlexStr<S> {
    /// Attempt to create an inlined string from a borrowed string. Returns `None` if the string is too long.
    pub fn try_from_type(s: &S) -> Result<Self, TooLongForInlining> {
        let bytes = S::self_as_raw_bytes(s);

        if bytes.len() <= INLINE_CAPACITY {
            Ok(Self::from_bytes(bytes))
        } else {
            Err(TooLongForInlining {
                length: bytes.len(),
                inline_capacity: INLINE_CAPACITY,
            })
        }
    }

    #[cfg(feature = "safe")]
    pub(crate) fn from_bytes(s: &[u8]) -> Self {
        let mut inline = [0u8; INLINE_CAPACITY];
        let len = s.len();

        // PANIC SAFETY: Caller responsible for ensuring the slice is not too long
        inline[..len].copy_from_slice(&s[..len]);

        Self {
            inline,
            len: len as u8,
            marker: PhantomData,
        }
    }

    #[cfg(not(feature = "safe"))]
    pub(crate) fn from_bytes(slice: &[u8]) -> Self {
        // Create an uninitialized array
        let mut inline = [core::mem::MaybeUninit::<u8>::uninit(); INLINE_CAPACITY];
        let len = slice.len();

        // SAFETY: There be dragons here! I have carefully inspected the code to ensure that it is safe IF and ONLY IF
        // len <= INLINE_CAPACITY (this is verified by the caller! which is why this is pub(crate) only).
        // [u8; N] and [MaybeUninit<u8>; N] are guranteed per docs to have the same size and layout.
        let inline = unsafe {
            // Copy the slice data
            core::ptr::copy_nonoverlapping(slice.as_ptr(), inline.as_mut_ptr() as *mut u8, len);

            // Fill the rest with zeros
            core::ptr::write_bytes(inline.as_mut_ptr().add(len), 0u8, INLINE_CAPACITY - len);

            // Transmute to a regular array
            core::mem::transmute::<
                [core::mem::MaybeUninit<u8>; INLINE_CAPACITY],
                [u8; INLINE_CAPACITY],
            >(inline)
        };

        Self {
            inline,
            len: len as u8,
            marker: PhantomData,
        }
    }

    #[cfg(all(feature = "safe", feature = "cstr"))]
    #[inline(always)]
    pub(crate) fn append_nul_zero(&mut self) {
        // PANIC SAFETY: We know the length is valid and at least one byte shorter than the capacity
        self.inline[self.len as usize] = 0;
        self.len += 1;
    }

    #[cfg(all(not(feature = "safe"), feature = "cstr"))]
    #[inline(always)]
    pub(crate) fn append_nul_zero(&mut self) {
        // SAFETY: We know the length is valid and at least one byte shorter than the capacity
        unsafe {
            *self.inline.get_unchecked_mut(self.len as usize) = 0;
        }
        self.len += 1;
    }

    #[cfg(feature = "safe")]
    /// Borrow the inline bytes as a raw byte slice (NOTE: includes trailing NUL for CStr)
    pub fn as_raw_bytes(&self) -> &[u8] {
        &self.inline[..self.len as usize]
    }

    #[cfg(not(feature = "safe"))]
    /// Borrow the inline bytes as a raw byte slice (NOTE: includes trailing NUL for CStr)
    pub fn as_raw_bytes(&self) -> &[u8] {
        // SAFETY: The length cannot be changed after initialization, so we know it is valid
        unsafe { self.inline.get_unchecked(..self.len as usize) }
    }

    #[cfg(feature = "safe")]
    fn as_raw_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.inline[..self.len as usize]
    }

    #[cfg(not(feature = "safe"))]
    fn as_raw_bytes_mut(&mut self) -> &mut [u8] {
        // SAFETY: The length cannot be changed after initialization, so we know it is valid
        unsafe { self.inline.get_unchecked_mut(..self.len as usize) }
    }

    /// Borrow a string reference as `&S`
    pub fn as_ref_type(&self) -> &S {
        S::bytes_as_self(self.as_raw_bytes())
    }

    /// Borrow the inline bytes as bytes
    pub fn as_bytes(&self) -> &[u8] {
        S::self_as_bytes(self.as_ref_type())
    }

    /// Consume a string and convert it to an owned string.
    pub fn into_owned_type(self) -> S::Owned
    where
        S::Owned: From<Box<S>>,
    {
        self.to_owned_type()
    }

    /// Convert a string reference to an owned string.
    pub fn to_owned_type(&self) -> S::Owned {
        self.as_ref_type().to_owned()
    }
}

impl<S: ?Sized + StringFromBytesMut> InlineFlexStr<S> {
    /// Borrow the inline string as a mutable string reference
    pub fn as_mut_type(&mut self) -> &mut S {
        S::bytes_as_self_mut(self.as_raw_bytes_mut())
    }
}

// *** StringLike ***

impl<S: ?Sized + StringToFromBytes + 'static> StringLike<S> for InlineFlexStr<S> {
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

impl<S: ?Sized + StringToFromBytes> Default for InlineFlexStr<S>
where
    for<'a> &'a S: Default,
{
    fn default() -> Self {
        Self::from_bytes(S::self_as_raw_bytes(Default::default()))
    }
}

// *** Copy ***

impl<S: ?Sized + StringToFromBytes> Copy for InlineFlexStr<S> {}

// *** Clone ***

impl<S: ?Sized + StringToFromBytes> Clone for InlineFlexStr<S> {
    fn clone(&self) -> Self {
        *self
    }
}

// *** Hash ***

impl<S: ?Sized + StringToFromBytes> Hash for InlineFlexStr<S>
where
    S: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_ref_type().hash(state);
    }
}

// *** AsMut ***

impl<S: ?Sized + StringFromBytesMut> AsMut<S> for InlineFlexStr<S> {
    #[inline]
    fn as_mut(&mut self) -> &mut S {
        self.as_mut_type()
    }
}

// *** Deref<Target = S> ***

impl<S: ?Sized + StringToFromBytes> Deref for InlineFlexStr<S> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        self.as_ref_type()
    }
}

// *** DerefMut ***

impl<S: ?Sized + StringFromBytesMut> DerefMut for InlineFlexStr<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_type()
    }
}

// *** Display ***

impl<S: ?Sized + StringToFromBytes> fmt::Display for InlineFlexStr<S>
where
    S: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        S::fmt(self.as_ref_type(), f)
    }
}

// *** Borrow / BorrowMut ***

impl<S: ?Sized + StringToFromBytes> Borrow<S> for InlineFlexStr<S> {
    fn borrow(&self) -> &S {
        self.as_ref_type()
    }
}

impl<S: ?Sized + StringFromBytesMut> BorrowMut<S> for InlineFlexStr<S> {
    fn borrow_mut(&mut self) -> &mut S {
        self.as_mut_type()
    }
}

// *** PartialEq / Eq ***

impl<S: ?Sized + StringToFromBytes> PartialEq for InlineFlexStr<S>
where
    S: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        S::eq(self.as_ref_type(), other.as_ref_type())
    }
}

impl<S: ?Sized + StringToFromBytes> Eq for InlineFlexStr<S> where S: Eq {}

// *** PartialOrd / Ord ***

impl<S: ?Sized + StringToFromBytes> PartialOrd for InlineFlexStr<S>
where
    S: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        S::partial_cmp(self.as_ref_type(), other.as_ref_type())
    }
}

impl<S: ?Sized + StringToFromBytes> Ord for InlineFlexStr<S>
where
    S: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        S::cmp(self.as_ref_type(), other.as_ref_type())
    }
}

// *** Index / IndexMut ***

impl<S: ?Sized + StringToFromBytes, I: SliceIndex<S>> Index<I> for InlineFlexStr<S>
where
    S: Index<I>,
{
    type Output = S::Output;

    fn index(&self, index: I) -> &Self::Output {
        S::index(self.as_ref_type(), index)
    }
}

impl<S: ?Sized + StringFromBytesMut, I: SliceIndex<S>> IndexMut<I> for InlineFlexStr<S>
where
    S: IndexMut<I>,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        S::index_mut(self.as_mut_type(), index)
    }
}

// *** ToSocketAddrs ***

#[cfg(feature = "std")]
impl<S: ?Sized + StringToFromBytes> ToSocketAddrs for InlineFlexStr<S>
where
    S: ToSocketAddrs,
{
    type Iter = <S as ToSocketAddrs>::Iter;

    fn to_socket_addrs(&self) -> io::Result<<S as ToSocketAddrs>::Iter> {
        self.as_ref_type().to_socket_addrs()
    }
}

// *** Serialize ***

#[cfg(feature = "serde")]
impl<S: ?Sized + StringToFromBytes> Serialize for InlineFlexStr<S>
where
    S: Serialize,
{
    fn serialize<SER: Serializer>(&self, serializer: SER) -> Result<SER::Ok, SER::Error> {
        S::serialize(self.as_ref_type(), serializer)
    }
}

// *** Deserialize ***

#[cfg(feature = "serde")]
impl<'de, S: ?Sized + StringToFromBytes> Deserialize<'de> for InlineFlexStr<S>
where
    Box<S>: Deserialize<'de>,
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // TODO: This is inefficent, we should ideally deserialize directly into the InlineFlexStr type.
        // However, Deserialize is not implmented for all types of &S, so likely that would mean
        // a non-generic implementation for each type of S, likely via a Visitor pattern. That also
        // means we'd have to understand how serde serializes each type, and this might be brittle if
        // that ever changes (for example, OsStr is a bit special). For now, this is a quick way to
        // make it work, albeit at the cost of an allocation and a copy.
        let s = Box::deserialize(deserializer)?;

        InlineFlexStr::try_from_type(&*s).map_err(|_| {
            let bytes = S::self_as_raw_bytes(&*s);
            serde::de::Error::invalid_length(bytes.len(), &"string too long for inline storage")
        })
    }
}
