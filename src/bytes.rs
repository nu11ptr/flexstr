#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use alloc::{borrow::Cow, rc::Rc, sync::Arc};

use crate::{
    FlexStr, InlineFlexStr, RefCounted, RefCountedMut, StringFromBytesMut, StringToFromBytes,
    inline::inline_partial_eq_impl, partial_eq_impl, ref_counted_mut_impl,
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

// *** RefCountedMut ***

ref_counted_mut_impl!([u8]);

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

// *** PartialEq ***

partial_eq_impl!([u8], [u8]);
partial_eq_impl!(&[u8], [u8]);
partial_eq_impl!(Vec<u8>, [u8]);
partial_eq_impl!(Cow<'s, [u8]>, [u8]);

inline_partial_eq_impl!([u8], [u8]);
inline_partial_eq_impl!(&[u8], [u8]);
inline_partial_eq_impl!(Vec<u8>, [u8]);
inline_partial_eq_impl!(Cow<'_, [u8]>, [u8]);

// *** AsRef ***

impl<'s, S: ?Sized + StringToFromBytes, R: RefCounted<S>> AsRef<[u8]> for FlexStr<'s, S, R>
where
    S: AsRef<[u8]>,
{
    fn as_ref(&self) -> &[u8] {
        self.as_ref_type().as_ref()
    }
}

impl<S: ?Sized + StringToFromBytes> AsRef<[u8]> for InlineFlexStr<S>
where
    S: AsRef<[u8]>,
{
    fn as_ref(&self) -> &[u8] {
        self.as_ref_type().as_ref()
    }
}
