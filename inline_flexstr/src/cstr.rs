use alloc::{borrow::Cow, ffi::CString};
use core::{
    ffi::{CStr, FromBytesWithNulError},
    fmt,
    str::FromStr,
};

use crate::inline::{INLINE_CAPACITY, InlineFlexStr, TooLongForInlining, inline_partial_eq_impl};

use flexstr_support::{InteriorNulError, StringToFromBytes};

/// Inline `CStr` type
pub type InlineCStr = InlineFlexStr<CStr>;

// *** TooLongOrNulError ***

/// Error type returned when a C String is too long for inline storage or has an interior NUL byte.
#[derive(Debug)]
pub enum TooLongOrNulError {
    /// The C String is too long for inline storage
    TooLong(TooLongForInlining),
    /// The C String has an interior NUL byte
    NulError(InteriorNulError),
}

impl fmt::Display for TooLongOrNulError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TooLongOrNulError::TooLong(e) => e.fmt(f),
            TooLongOrNulError::NulError(e) => e.fmt(f),
        }
    }
}

impl core::error::Error for TooLongOrNulError {}

// *** InlineFlexStr ***

impl InlineFlexStr<CStr> {
    fn try_from_bytes_without_nul(bytes: &[u8]) -> Result<Self, TooLongOrNulError> {
        if bytes.len() < INLINE_CAPACITY {
            let mut inline = Self::from_bytes(bytes);
            inline.append_nul_zero();
            Ok(inline)
        } else {
            Err(TooLongOrNulError::TooLong(TooLongForInlining {
                length: bytes.len(),
                inline_capacity: INLINE_CAPACITY,
            }))
        }
    }

    /// Attempt to create an inlined string from borrowed bytes with or without a trailing NUL byte.
    pub fn try_from_bytes_with_or_without_nul(bytes: &[u8]) -> Result<Self, TooLongOrNulError> {
        match CStr::from_bytes_with_nul(bytes) {
            Ok(cstr) => Self::try_from_type(cstr).map_err(TooLongOrNulError::TooLong),
            Err(FromBytesWithNulError::NotNulTerminated) => Self::try_from_bytes_without_nul(bytes),
            Err(FromBytesWithNulError::InteriorNul { position }) => {
                Err(TooLongOrNulError::NulError(InteriorNulError { position }))
            }
        }
    }

    /// Borrow the CStr as bytes with a trailing NUL byte
    #[inline]
    pub fn as_bytes_with_nul(&self) -> &[u8] {
        self.as_raw_bytes()
    }
}

// *** TryFrom for InlineFlexStr ***

// NOTE: Cannot be implemented generically because of impl<T, U> TryFrom<U> for T where U: Into<T>
impl<'s> TryFrom<&'s CStr> for InlineFlexStr<CStr> {
    type Error = TooLongForInlining;

    #[inline]
    fn try_from(s: &'s CStr) -> Result<Self, Self::Error> {
        InlineFlexStr::try_from_type(s)
    }
}

impl<'s> TryFrom<&'s str> for InlineFlexStr<CStr> {
    type Error = TooLongOrNulError;

    #[inline]
    fn try_from(s: &'s str) -> Result<Self, Self::Error> {
        InlineFlexStr::try_from_bytes_with_or_without_nul(s.as_bytes())
    }
}

impl<'s> TryFrom<&'s [u8]> for InlineFlexStr<CStr> {
    type Error = TooLongOrNulError;

    #[inline]
    fn try_from(bytes: &'s [u8]) -> Result<Self, Self::Error> {
        InlineFlexStr::try_from_bytes_with_or_without_nul(bytes)
    }
}

// *** PartialEq ***

inline_partial_eq_impl!(CStr, CStr);
inline_partial_eq_impl!(&CStr, CStr);
inline_partial_eq_impl!(CString, CStr);
inline_partial_eq_impl!(Cow<'_, CStr>, CStr);

// *** AsRef ***

impl<S: ?Sized + StringToFromBytes> AsRef<CStr> for InlineFlexStr<S>
where
    S: AsRef<CStr>,
{
    fn as_ref(&self) -> &CStr {
        self.as_ref_type().as_ref()
    }
}

// *** FromStr ***

impl FromStr for InlineFlexStr<CStr> {
    type Err = TooLongOrNulError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        InlineFlexStr::try_from_bytes_with_or_without_nul(s.as_bytes())
    }
}
