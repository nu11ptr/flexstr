use alloc::{borrow::Cow, ffi::CString, rc::Rc, sync::Arc};
use core::{
    ffi::{CStr, FromBytesWithNulError},
    str::FromStr,
};

use crate::{
    FlexStr, ImmutableBytes, RefCounted, RefCountedMut, partial_eq_impl, ref_counted_mut_impl,
};

use flexstr_support::{InteriorNulError, StringToFromBytes};
use inline_flexstr::{InlineFlexStr, TooLongOrNulError};

/// Local `CStr` type (NOTE: This can't be shared between threads)
pub type LocalCStr<'s> = FlexStr<'s, CStr, Rc<CStr>>;

/// Shared `CStr` type
pub type SharedCStr<'s> = FlexStr<'s, CStr, Arc<CStr>>;

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

// *** FlexStr ***

impl<'s, R: RefCounted<CStr>> FlexStr<'s, CStr, R> {
    fn from_bytes_without_nul(bytes: &'s [u8]) -> Self {
        // NOTE: This will scan the string for interior NUL bytes _twice_. Consider optionally
        // making InlineFlexStr::try_from_bytes_without_nul unsafe and using it conditionally here.
        match InlineFlexStr::try_from_bytes_with_or_without_nul(bytes) {
            Ok(inline) => FlexStr::from_inline(inline),
            // Finally, fallback to creating a new CString so nul zero is appended
            Err(TooLongOrNulError::TooLong(_)) => FlexStr::from_owned(
                #[cfg(feature = "safe")]
                // PANIC SAFETY: We already tested for interior NUL bytes
                CString::new(bytes).expect("Unexpected interior NUL byte"),
                #[cfg(not(feature = "safe"))]
                // SAFETY: We already tested for interior NUL bytes
                unsafe {
                    CString::from_vec_unchecked(bytes.into())
                },
            ),
            // PANIC SAFETY: We already tested for interior NUL bytes from the function that called this one
            Err(TooLongOrNulError::NulError(e)) => {
                unreachable!("Interior NUL byte found at position {}", e.position)
            }
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

// *** PartialEq ***

partial_eq_impl!(CStr, CStr);
partial_eq_impl!(&CStr, CStr);
partial_eq_impl!(CString, CStr);
partial_eq_impl!(Cow<'s, CStr>, CStr);

// *** AsRef ***

impl<'s, S: ?Sized + StringToFromBytes, R: RefCounted<S>> AsRef<CStr> for FlexStr<'s, S, R>
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
