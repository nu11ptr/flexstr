#![cfg(feature = "std")]

use alloc::rc::Rc;
use alloc::sync::Arc;
use core::fmt::{Debug, Display, Formatter};
use core::mem;
use std::error::Error;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use paste::paste;

use crate::string::Str;
use crate::{define_flex_types, impl_flex_str};
use crate::{BorrowStr, FlexStrInner};

impl Str for CStr {
    type StringType = CString;
    type InlineType = c_char;

    #[inline]
    fn from_raw_data(bytes: &[Self::InlineType]) -> &Self {
        // SAFETY: This will always be prior vetted to ensure it ends with a null terminator
        unsafe { Self::from_ptr(bytes as *const [Self::InlineType] as *const Self::InlineType) }
    }

    #[inline]
    fn length(&self) -> usize {
        // TODO: Stdlib hints that it may change this to be non const time - might need a diff way
        // NOTE: This will include trailing null byte (this is storage, not usable chars)
        self.to_bytes_with_nul().len()
    }

    #[inline]
    fn as_pointer(&self) -> *const Self::InlineType {
        self.as_ptr()
    }
}

define_flex_types!("C", CStr);

impl_flex_str!(FlexCStr, CStr);

/// This error is returned when trying to create a new [FlexCStr] from a [&\[u8\]] sequence without
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

impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexCStr<'str, SIZE, BPAD, HPAD, HEAP>
{
    /// Creates a wrapped static string literal. This function is equivalent to using the macro and
    /// is `const fn` so it can be used to initialize a constant at compile time with zero runtime cost.
    /// ```
    /// use std::ffi::CStr;
    /// use flexstr::c_str::LocalCStr;
    ///
    /// let s: &'static CStr = CStr::from_bytes_with_nul(b"test\0").unwrap();
    /// const S: LocalCStr = LocalCStr::from_static(s);
    /// assert!(S.is_static());
    /// ```
    #[inline]
    pub const fn from_static(s: &'static CStr) -> Self {
        if Self::IS_VALID_SIZE {
            Self(FlexStrInner {
                static_str: mem::ManuallyDrop::new(BorrowStr::from_static(s)),
            })
        } else {
            panic!("{}", BAD_SIZE_OR_ALIGNMENT);
        }
    }

    /// Tries to create a wrapped static string literal from a raw byte slice. If it is successful, a
    /// [FlexCStr] will be created using static wrapped storage. If unsuccessful (because encoding is
    /// incorrect) a [CStrNullError] is returned. This is `const fn` so it can be used to initialize
    /// a constant at compile time with zero runtime cost.
    /// ```
    /// use flexstr::c_str::LocalCStr;
    ///
    /// const S: LocalCStr = LocalCStr::try_from_static_raw(b"This is a valid CStr\0").unwrap();
    /// assert!(S.is_static());
    /// ```
    #[inline]
    pub const fn try_from_static_raw(s: &'static [u8]) -> Result<Self, CStrNullError> {
        // We go through all this work just to make this const fn :-) If using stdlib it is a one liner

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
                Ok(Self::from_static(s))
            } else {
                // Interior null byte
                Err(CStrNullError::InteriorNullByte(pos))
            }
        } else {
            // No null byte
            Err(CStrNullError::NoNullByteFound)
        }
    }
}
