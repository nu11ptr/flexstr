#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use alloc::{rc::Rc, sync::Arc};

use crate::{
    FlexStr, InlineFlexStr, RefCounted, RefCountedMut, StringFromBytesMut, StringToFromBytes,
};

/// Local `[u8]` type (NOTE: This can't be shared between threads)
pub type LocalBytes<'s> = FlexStr<'s, [u8], Rc<[u8]>>;

/// Shared `[u8]` type
pub type SharedBytes<'s> = FlexStr<'s, [u8], Arc<[u8]>>;

/// Inline `[u8]` type
pub type InlineBytes = InlineFlexStr<[u8]>;

const _: () = assert!(
    size_of::<Option<LocalBytes>>() <= size_of::<Vec<u8>>(),
    "Option<LocalBytes> must be less than or equal to the size of Vec<u8>"
);
const _: () = assert!(
    size_of::<Option<SharedBytes>>() <= size_of::<Vec<u8>>(),
    "Option<SharedBytes> must be less than or equal to the size of Vec<u8>"
);

// *** StringToFromBytes ***

impl StringToFromBytes for [u8] {
    #[inline]
    fn bytes_as_self(bytes: &[u8]) -> &Self {
        bytes
    }

    #[inline]
    fn self_as_raw_bytes(&self) -> &[u8] {
        self
    }
}

// *** StringFromBytesMut ***

impl StringFromBytesMut for [u8] {
    #[inline]
    fn bytes_as_self_mut(bytes: &mut [u8]) -> &mut Self {
        bytes
    }
}

// *** From<Vec<u8>> ***

// NOTE: Cannot be implemented generically because of impl<T> From<T> for T
impl<'s, R: RefCounted<[u8]>> From<Vec<u8>> for FlexStr<'s, [u8], R> {
    fn from(v: Vec<u8>) -> Self {
        FlexStr::from_owned(v)
    }
}

// *** TryFrom<&[u8]> for InlineFlexStr ***

// NOTE: Cannot be implemented generically because of impl<T, U> TryFrom<U> for T where U: Into<T>
impl<'s> TryFrom<&'s [u8]> for InlineFlexStr<[u8]> {
    type Error = &'s [u8];

    #[inline]
    fn try_from(s: &'s [u8]) -> Result<Self, Self::Error> {
        InlineFlexStr::try_from_type(s)
    }
}

// *** RefCountedMut ***

// NOTE: Cannot be implemented generically because CloneToUninit is needed
// as a bound to `S`, but is unstable.
impl RefCountedMut<[u8]> for Arc<[u8]> {
    #[inline]
    fn to_mut(&mut self) -> &mut [u8] {
        Arc::make_mut(self)
    }

    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        // PANIC SAFETY: We only use this when we know the Arc is newly created
        Arc::get_mut(self).expect("Arc is shared")
    }
}

// NOTE: Cannot be implemented generically because CloneToUninit is needed
// as a bound to `S`, but is unstable.
impl RefCountedMut<[u8]> for Rc<[u8]> {
    #[inline]
    fn to_mut(&mut self) -> &mut [u8] {
        Rc::make_mut(self)
    }

    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        // PANIC SAFETY: We only use this when we know the Rc is newly created
        Rc::get_mut(self).expect("Rc is shared")
    }
}
