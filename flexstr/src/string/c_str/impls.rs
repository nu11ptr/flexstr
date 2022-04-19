#![cfg(feature = "std")]

use alloc::rc::Rc;
use alloc::sync::Arc;
use std::ffi::CStr;

use paste::paste;

use crate::inner::FlexStrInner;
use crate::string::c_str::{try_from_raw, CStrNullError};
use crate::traits::private;
use crate::traits::private::FlexStrCoreInner;
use crate::{define_flex_types, FlexStrCore, FlexStrCoreRef, Storage};

/// Empty C string constant
// This is the only way to get a const CStr that I can tell
// SAFETY: We visually inspect the below raw byte sequence and can see it has a trailing null
pub const EMPTY: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"\0") };

define_flex_types!("CStr", CStr, [u8]);

macro_rules! impl_body {
    () => {
        /// An empty ("") static constant string
        pub const EMPTY: Self = Self::from_static(EMPTY);

        /// Creates a wrapped static string literal. This function is equivalent to using the macro and
        /// is `const fn` so it can be used to initialize a constant at compile time with zero runtime cost.
        #[inline(always)]
        pub const fn from_static(s: &'static CStr) -> Self {
            Self(FlexStrInner::from_static(s))
        }

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
    };
}

// *** FlexCStr ***

impl<const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexCStr<SIZE, BPAD, HPAD, HEAP>
{
    impl_body!();
}

impl<const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexStrCore<'static, SIZE, BPAD, HPAD, HEAP, CStr> for FlexCStr<SIZE, BPAD, HPAD, HEAP>
where
    HEAP: Storage<CStr>,
{
    #[inline(always)]
    fn as_str_type(&self) -> &CStr {
        self.inner().as_str_type()
    }
}

// *** FlexCStrRef ***

impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexCStrRef<'str, SIZE, BPAD, HPAD, HEAP>
{
    impl_body!();
}

impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexStrCore<'str, SIZE, BPAD, HPAD, HEAP, CStr> for FlexCStrRef<'str, SIZE, BPAD, HPAD, HEAP>
where
    HEAP: Storage<CStr>,
{
    #[inline(always)]
    fn as_str_type(&self) -> &CStr {
        self.inner().as_str_type()
    }
}
