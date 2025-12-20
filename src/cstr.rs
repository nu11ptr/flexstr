use alloc::{borrow::Cow, ffi::CString, rc::Rc, sync::Arc};
use core::{
    ffi::{CStr, FromBytesWithNulError},
    fmt,
    str::FromStr,
};

use crate::{
    FlexStr, ImmutableBytes, InlineFlexStr, RefCounted, RefCountedMut, StringToFromBytes,
    inline::{INLINE_CAPACITY, TooLongForInlining, inline_partial_eq_impl},
    partial_eq_impl, ref_counted_mut_impl,
};

/// Local `CStr` type (NOTE: This can't be shared between threads)
pub type LocalCStr<'s> = FlexStr<'s, CStr, Rc<CStr>>;

/// Shared `CStr` type
pub type SharedCStr<'s> = FlexStr<'s, CStr, Arc<CStr>>;

/// Inline `CStr` type
pub type InlineCStr = InlineFlexStr<CStr>;

// NOTE: This one is a bit different because CString is just a Box<[u8]>. Instead of equal size,
// we should be at most one machine word larger.
const _: () = assert!(
    size_of::<Option<LocalCStr>>() <= size_of::<CString>() + size_of::<usize>(),
    "Option<LocalCStr> must be less than or equal to the size of CString plus one machine word"
);
const _: () = assert!(
    size_of::<Option<SharedCStr>>() <= size_of::<CString>() + size_of::<usize>(),
    "Option<SharedCStr> must be less than or equal to the size of CString plus one machine word"
);

// *** InteriorNulError ***

/// Error type returned when a C String has an interior NUL byte.
#[derive(Debug)]
pub struct InteriorNulError {
    /// The position of the interior NUL byte
    pub position: usize,
}

impl fmt::Display for InteriorNulError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Interior NUL byte found at position {}", self.position)
    }
}

impl core::error::Error for InteriorNulError {}

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

// *** FlexStr ***

impl<'s, R: RefCounted<CStr>> FlexStr<'s, CStr, R> {
    fn from_bytes_without_nul(bytes: &'s [u8]) -> Self {
        match InlineFlexStr::try_from_bytes_without_nul(bytes) {
            Ok(inline) => FlexStr::from_inline(inline),
            // Finally, fallback to creating a new CString so nul zero is appended
            Err(_) => FlexStr::from_owned(
                #[cfg(feature = "safe")]
                // PANIC SAFETY: We already tested for interior NUL bytes
                CString::new(bytes).expect("Unexpected interior NUL byte"),
                #[cfg(not(feature = "safe"))]
                // SAFETY: We already tested for interior NUL bytes
                unsafe {
                    CString::from_vec_unchecked(bytes.into())
                },
            ),
        }
    }

    /// Attempt to create a CStr from borrowed bytes with or without a trailing NUL byte.
    pub fn try_from_bytes_with_or_without_nul(bytes: &'s [u8]) -> Result<Self, InteriorNulError> {
        match CStr::from_bytes_with_nul(bytes) {
            // If it is already a valid CStr, then just borrow it
            Ok(cstr) => Ok(FlexStr::from_borrowed(cstr)),
            // Otherwise try and inline it, adding a nul zero
            Err(FromBytesWithNulError::NotNulTerminated) => Ok(Self::from_bytes_without_nul(bytes)),
            Err(FromBytesWithNulError::InteriorNul { position }) => {
                Err(InteriorNulError { position })
            }
        }
    }

    /// Borrow the CStr as bytes with a trailing NUL byte
    #[inline]
    pub fn as_bytes_with_nul(&self) -> &[u8] {
        self.as_raw_bytes()
    }
}

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

// *** StringToFromBytes ***

impl StringToFromBytes for CStr {
    #[cfg(feature = "safe")]
    #[inline]
    fn bytes_as_self(bytes: &[u8]) -> &Self {
        // PANIC SAFETY: We know the bytes are a valid CStr
        CStr::from_bytes_with_nul(bytes).expect("Missing NUL byte")
    }

    #[cfg(not(feature = "safe"))]
    #[inline]
    fn bytes_as_self(bytes: &[u8]) -> &Self {
        // SAFETY: We know the bytes are a valid CStr
        unsafe { CStr::from_bytes_with_nul_unchecked(bytes) }
    }

    #[inline]
    fn self_as_bytes(&self) -> &[u8] {
        self.to_bytes()
    }

    #[inline]
    fn self_as_raw_bytes(&self) -> &[u8] {
        self.to_bytes_with_nul()
    }
}

// *** ImmutableBytes ***

impl ImmutableBytes for CStr {}

// *** RefCountedMut ***

ref_counted_mut_impl!(CStr);

// *** From<CString> ***

// NOTE: Cannot be implemented generically because of impl<T> From<T> for T
impl<'s, R: RefCounted<CStr>> From<CString> for FlexStr<'s, CStr, R> {
    fn from(s: CString) -> Self {
        FlexStr::from_owned(s)
    }
}

// *** TryFrom for FlexStr ***

impl<'s, R: RefCounted<CStr>> TryFrom<&'s str> for FlexStr<'s, CStr, R> {
    type Error = InteriorNulError;

    #[inline]
    fn try_from(s: &'s str) -> Result<Self, Self::Error> {
        FlexStr::try_from_bytes_with_or_without_nul(s.as_bytes())
    }
}

impl<'s, R: RefCounted<CStr>> TryFrom<&'s [u8]> for FlexStr<'s, CStr, R> {
    type Error = InteriorNulError;

    #[inline]
    fn try_from(bytes: &'s [u8]) -> Result<Self, Self::Error> {
        FlexStr::try_from_bytes_with_or_without_nul(bytes)
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

partial_eq_impl!(CStr, CStr);
partial_eq_impl!(&CStr, CStr);
partial_eq_impl!(CString, CStr);
partial_eq_impl!(Cow<'s, CStr>, CStr);

inline_partial_eq_impl!(CStr, CStr);
inline_partial_eq_impl!(&CStr, CStr);
inline_partial_eq_impl!(CString, CStr);
inline_partial_eq_impl!(Cow<'_, CStr>, CStr);

// *** AsRef ***

impl<'s, S: ?Sized + StringToFromBytes, R: RefCounted<S>> AsRef<CStr> for FlexStr<'s, S, R>
where
    S: AsRef<CStr>,
{
    fn as_ref(&self) -> &CStr {
        self.as_ref_type().as_ref()
    }
}

impl<S: ?Sized + StringToFromBytes> AsRef<CStr> for InlineFlexStr<S>
where
    S: AsRef<CStr>,
{
    fn as_ref(&self) -> &CStr {
        self.as_ref_type().as_ref()
    }
}

// *** FromStr ***

impl<R: RefCounted<CStr>> FromStr for FlexStr<'static, CStr, R> {
    type Err = InteriorNulError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        FlexStr::try_from_bytes_with_or_without_nul(s.as_bytes()).map(FlexStr::into_owned)
    }
}

impl FromStr for InlineFlexStr<CStr> {
    type Err = TooLongOrNulError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        InlineFlexStr::try_from_bytes_with_or_without_nul(s.as_bytes())
    }
}
