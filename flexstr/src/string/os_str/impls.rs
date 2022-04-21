#![cfg(feature = "std")]

use alloc::rc::Rc;
use alloc::sync::Arc;
use std::ffi::OsStr;

use paste::paste;

use crate::inner::FlexStrInner;
use crate::string::Str;
use crate::traits::private;
use crate::traits::private::FlexStrCoreInner;
use crate::{define_flex_types, FlexStrCore, FlexStrCoreRef, Storage};

define_flex_types!("OsStr", OsStr, OsStr);

macro_rules! impl_body {
    () => {
        /// Creates a wrapped static string literal from a raw byte slice.
        #[cfg(unix)]
        #[cfg_attr(docsrs, doc(cfg(unix)))]
        #[inline]
        pub fn from_static_raw(s: &'static [u8]) -> Self {
            // I see no mention of const fn for these functions on unix - use trait
            Self(FlexStrInner::from_static(OsStr::from_inline_data(s)))
        }
    };
}

// *** FlexOsStr ***

impl<const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexOsStr<SIZE, BPAD, HPAD, HEAP>
{
    impl_body!();
}

impl<const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexStrCore<'static, SIZE, BPAD, HPAD, HEAP, OsStr> for FlexOsStr<SIZE, BPAD, HPAD, HEAP>
where
    HEAP: Storage<OsStr>,
{
    #[inline(always)]
    fn as_str_type(&self) -> &OsStr {
        self.inner().as_str_type()
    }
}

// *** FlexOsStrRef ***

impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexOsStrRef<'str, SIZE, BPAD, HPAD, HEAP>
{
    impl_body!();
}

impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexStrCore<'str, SIZE, BPAD, HPAD, HEAP, OsStr> for FlexOsStrRef<'str, SIZE, BPAD, HPAD, HEAP>
where
    HEAP: Storage<OsStr>,
{
    #[inline(always)]
    fn as_str_type(&self) -> &OsStr {
        self.inner().as_str_type()
    }
}
