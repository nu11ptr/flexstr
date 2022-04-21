#![cfg(feature = "std")]

use alloc::rc::Rc;
use alloc::sync::Arc;
use std::ffi::CStr;

use paste::paste;

use crate::inner::FlexStrInner;
use crate::string::c_str::EMPTY;
use crate::traits::private;
use crate::traits::private::FlexStrCoreInner;
use crate::{define_flex_types, FlexStrCore, FlexStrCoreRef, Storage};

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
