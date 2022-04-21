use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::sync::Arc;
use core::str;

use paste::paste;

use crate::inner::FlexStrInner;
use crate::string::std_str::EMPTY;
use crate::traits::private;
use crate::traits::private::FlexStrCoreInner;
use crate::{define_flex_types, FlexStrCore, FlexStrCoreRef, Storage};

define_flex_types!("Str", str, [u8]);

macro_rules! impl_body {
    () => {
        /// An empty ("") static constant string
        pub const EMPTY: Self = Self::from_static(EMPTY);

        /// Creates a wrapped static string literal. This function is equivalent to using the macro and
        /// is `const fn` so it can be used to initialize a constant at compile time with zero runtime cost.
        /// ```
        /// use flexstr::{FlexStrCore, LocalStr};
        ///
        /// const S: LocalStr = LocalStr::from_static("test");
        /// assert!(S.is_static());
        /// ```
        #[inline(always)]
        pub const fn from_static(s: &'static str) -> Self {
            Self(FlexStrInner::from_static(s))
        }
    };
}

// *** FlexStr ***

impl<const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexStr<SIZE, BPAD, HPAD, HEAP>
{
    impl_body!();
}

impl<const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexStrCore<'static, SIZE, BPAD, HPAD, HEAP, str> for FlexStr<SIZE, BPAD, HPAD, HEAP>
where
    HEAP: Storage<str>,
{
    /// blah
    #[inline(always)]
    fn as_str_type(&self) -> &str {
        self.inner().as_str_type()
    }
}

// *** FlexStrRef ***

impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexStrRef<'str, SIZE, BPAD, HPAD, HEAP>
{
    impl_body!();
}

impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexStrCore<'str, SIZE, BPAD, HPAD, HEAP, str> for FlexStrRef<'str, SIZE, BPAD, HPAD, HEAP>
where
    HEAP: Storage<str>,
{
    #[inline(always)]
    fn as_str_type(&self) -> &str {
        self.inner().as_str_type()
    }
}
