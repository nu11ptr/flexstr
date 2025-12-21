use alloc::borrow::Cow;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use core::str::FromStr;

use crate::inline::{InlineFlexStr, TooLongForInlining, inline_partial_eq_impl};

use flexstr_support::StringToFromBytes;

/// Inline `[u8]` type
pub type InlineBytes = InlineFlexStr<[u8]>;

// *** TryFrom for InlineFlexStr ***

// NOTE: Cannot be implemented generically because of impl<T, U> TryFrom<U> for T where U: Into<T>
impl<'s> TryFrom<&'s [u8]> for InlineFlexStr<[u8]> {
    type Error = TooLongForInlining;

    #[inline]
    fn try_from(s: &'s [u8]) -> Result<Self, Self::Error> {
        InlineFlexStr::try_from_type(s)
    }
}

impl<'s> TryFrom<&'s str> for InlineFlexStr<[u8]> {
    type Error = TooLongForInlining;

    #[inline]
    fn try_from(s: &'s str) -> Result<Self, Self::Error> {
        InlineFlexStr::try_from_type(s.as_bytes())
    }
}

// *** PartialEq ***

inline_partial_eq_impl!([u8], [u8]);
inline_partial_eq_impl!(&[u8], [u8]);
inline_partial_eq_impl!(Vec<u8>, [u8]);
inline_partial_eq_impl!(Cow<'_, [u8]>, [u8]);

// *** AsRef ***

impl<S: ?Sized + StringToFromBytes> AsRef<[u8]> for InlineFlexStr<S>
where
    S: AsRef<[u8]>,
{
    fn as_ref(&self) -> &[u8] {
        self.as_ref_type().as_ref()
    }
}

// *** FromStr ***

impl FromStr for InlineFlexStr<[u8]> {
    type Err = TooLongForInlining;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        InlineFlexStr::try_from_type(s.as_bytes())
    }
}
