#![cfg(feature = "std")]

mod impls;

use core::fmt::{Debug, Display, Formatter};
use std::error::Error;
use std::ffi::{CStr, CString};

pub use self::impls::*;
use crate::inner::FlexStrInner;
use crate::string::Str;

/// Empty C string constant
// This is the only way to get a const CStr that I can tell
// SAFETY: We visually inspect the below raw byte sequence and can see it has a trailing null
pub const EMPTY: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"\0") };

impl Str for CStr {
    type StringType = CString;
    type HeapType = [u8];
    type ConvertError = CStrNullError;

    #[inline]
    fn from_inline_data(bytes: &[u8]) -> &Self {
        // SAFETY: This data is pre-vetted to ensure it ends with a null byte
        unsafe { CStr::from_bytes_with_nul_unchecked(bytes) }
    }

    #[inline]
    fn from_heap_data(bytes: &Self::HeapType) -> &Self {
        Self::from_inline_data(bytes)
    }

    #[inline]
    fn try_from_raw_data(bytes: &[u8]) -> Result<&Self, Self::ConvertError> {
        try_from_raw(bytes)
    }

    #[inline(always)]
    fn empty(&self) -> Option<&'static Self> {
        // This is ok since this is a CStr which has an invariant that it MUST end with a null byte
        // so a length of 1 MUST be an empty CStr
        if self.length() == 1 {
            Some(EMPTY)
        } else {
            None
        }
    }

    #[inline(always)]
    fn length(&self) -> usize {
        // NOTE: This will include trailing null byte (this is storage, not usable chars)
        self.as_heap_type().len()
    }

    #[inline]
    fn as_heap_type(&self) -> &Self::HeapType {
        // TODO: Stdlib hints that it may change this to be non const time - might need a diff way?
        self.to_bytes_with_nul()
    }

    #[inline(always)]
    fn as_inline_ptr(&self) -> *const u8 {
        self.as_ptr() as *const u8
    }
}

/// This error is returned when trying to create a new [FlexStrBase] from a [&\[u8\]] sequence without
/// a trailing null
#[derive(Clone, Copy, Debug)]
pub enum CStrNullError {
    /// No required null byte was found
    NoNullByteFound,

    /// An interior null byte was found - the position is enclosed
    InteriorNullByte(usize),
}

impl Display for CStrNullError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            // TODO: Replace with 'flex_fmt'
            CStrNullError::InteriorNullByte(pos) => f.write_str(&format!(
                "The byte slice had an interior null byte (Pos: {pos})"
            )),
            CStrNullError::NoNullByteFound => {
                f.write_str("The byte slice had no trailing null byte")
            }
        }
    }
}

impl Error for CStrNullError {}

#[inline]
const fn try_from_raw(s: &[u8]) -> Result<&CStr, CStrNullError> {
    // We go through all this work just to make this const fn :-) If using stdlib it is a one liner
    // Didn't see any signs it would be made const fn anytime soon

    // Search string for null zero - `for` is not allowed in `const fn` functions unfortunately
    let mut idx = 0;
    let mut pos = None;

    while idx < s.len() {
        if s[idx] == b'\0' {
            pos = Some(idx);
            break;
        }

        idx += 1;
    }

    if let Some(pos) = pos {
        if pos == s.len() - 1 {
            // SAFETY: We manually verified it is valid just above
            let s = unsafe { CStr::from_bytes_with_nul_unchecked(s) };
            Ok(s)
        } else {
            // Interior null byte
            Err(CStrNullError::InteriorNullByte(pos))
        }
    } else {
        // No null byte
        Err(CStrNullError::NoNullByteFound)
    }
}

impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexCStr<'str, SIZE, BPAD, HPAD, HEAP>
{
    /// An empty ("") static constant string
    pub const EMPTY: Self = Self::from_static(EMPTY);

    /// Tries to create a wrapped static string literal from a raw byte slice. If it is successful, a
    /// [FlexCStr] will be created using static wrapped storage. If unsuccessful (because encoding is
    /// incorrect) a [CStrNullError] is returned. This is `const fn` so it can be used to initialize
    /// a constant at compile time with zero runtime cost.
    /// ```
    /// use flexstr::FlexStrCore;
    /// use flexstr::c_str::{CStrNullError, LocalCStr};
    ///
    /// const S: Result<LocalCStr, CStrNullError> = LocalCStr::try_from_static_raw(b"This is a valid CStr\0");
    /// assert!(S.unwrap().is_static());
    /// ```
    #[inline]
    pub const fn try_from_static_raw(s: &'static [u8]) -> Result<Self, CStrNullError> {
        // '?' not allowed in const fn
        match try_from_raw(s) {
            Ok(s) => Ok(Self(FlexStrInner::from_static(s))),
            Err(err) => Err(err),
        }
    }
}
