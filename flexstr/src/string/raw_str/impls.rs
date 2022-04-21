use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::sync::Arc;

use paste::paste;

use crate::inner::FlexStrInner;
use crate::string::raw_str::EMPTY;
use crate::traits::private;
use crate::traits::private::FlexStrCoreInner;
use crate::{define_flex_types, FlexStrCore, FlexStrCoreRef, Storage};

define_flex_types!("RawStr", [u8], [u8]);

macro_rules! impl_body {
    () => {
        /// An empty ("") static constant string
        pub const EMPTY: Self = Self::from_static(EMPTY);

        /// Creates a wrapped static string literal. This function is equivalent to using the macro and
        /// is `const fn` so it can be used to initialize a constant at compile time with zero runtime cost.
        #[inline(always)]
        pub const fn from_static(s: &'static [u8]) -> Self {
            Self(FlexStrInner::from_static(s))
        }
    };
}

// *** FlexRawStr ***

impl<const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexRawStr<SIZE, BPAD, HPAD, HEAP>
{
    impl_body!();
}

impl<const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexStrCore<'static, SIZE, BPAD, HPAD, HEAP, [u8]> for FlexRawStr<SIZE, BPAD, HPAD, HEAP>
where
    HEAP: Storage<[u8]>,
{
    #[inline(always)]
    fn as_str_type(&self) -> &[u8] {
        self.inner().as_str_type()
    }
}

// *** FlexRawStrRef ***

impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexRawStrRef<'str, SIZE, BPAD, HPAD, HEAP>
{
    impl_body!();
}

impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexStrCore<'str, SIZE, BPAD, HPAD, HEAP, [u8]> for FlexRawStrRef<'str, SIZE, BPAD, HPAD, HEAP>
where
    HEAP: Storage<[u8]>,
{
    #[inline(always)]
    fn as_str_type(&self) -> &[u8] {
        self.inner().as_str_type()
    }
}
