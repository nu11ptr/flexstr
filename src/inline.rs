#[cfg(not(feature = "std"))]
use alloc::string::String;
use core::marker::PhantomData;
use core::ops::Deref;

use crate::StringOps;

// This must be the size of the String type minus 2 bytes for the length and discriminator
/// The capacity of the inline bytes type
pub const INLINE_CAPACITY: usize = size_of::<String>() - 2;

/// Inline bytes type - used to store small strings inline
#[derive(Debug)]
pub struct InlineStr<S: ?Sized + StringOps> {
    inline: [u8; INLINE_CAPACITY],
    len: u8,
    marker: PhantomData<S>,
}

impl<S: ?Sized + StringOps> InlineStr<S> {
    /// Attempt to create an inlined string from a borrowed string. Returns `None` if the string is too long.
    pub fn try_from_type(s: &S) -> Result<Self, &S> {
        let bytes = S::self_as_raw_bytes(s);

        if bytes.len() <= INLINE_CAPACITY {
            Ok(Self::from_bytes(bytes))
        } else {
            Err(s)
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
                [std::mem::MaybeUninit<u8>; INLINE_CAPACITY],
                [u8; INLINE_CAPACITY],
            >(inline)
        };

        Self {
            inline,
            len: len as u8,
            marker: PhantomData,
        }
    }

    /// Returns true if this is an empty string
    pub fn is_empty(&self) -> bool {
        self.as_bytes().is_empty()
    }

    /// Returns the length of this string in bytes
    pub fn len(&self) -> usize {
        self.as_bytes().len()
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

    /// Borrow a string reference as `&S`
    pub fn as_borrowed_type(&self) -> &S {
        S::bytes_as_self(self.as_raw_bytes())
    }

    /// Borrow the inline bytes as bytes
    pub fn as_bytes(&self) -> &[u8] {
        S::self_as_bytes(self.as_borrowed_type())
    }
}

// *** Clone ***

impl<S: ?Sized + StringOps> Clone for InlineStr<S> {
    fn clone(&self) -> Self {
        Self {
            inline: self.inline,
            len: self.len,
            marker: PhantomData,
        }
    }
}

// *** AsRef<S> ***

impl<S: ?Sized + StringOps> AsRef<S> for InlineStr<S> {
    #[inline(always)]
    fn as_ref(&self) -> &S {
        self.as_borrowed_type()
    }
}

// *** Deref<Target = S> ***

impl<S: ?Sized + StringOps> Deref for InlineStr<S> {
    type Target = S;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_borrowed_type()
    }
}
