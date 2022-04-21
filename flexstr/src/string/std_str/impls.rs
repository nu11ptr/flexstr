use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::sync::Arc;
use core::str;

use paste::paste;

use crate::inner::FlexStrInner;
use crate::traits::private;
use crate::traits::private::FlexStrCoreInner;
use crate::{define_flex_types, FlexStrCore, FlexStrCoreRef, Storage};

define_flex_types!("Str", str, [u8]);

// *** FlexStr ***

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
    FlexStrCore<'str, SIZE, BPAD, HPAD, HEAP, str> for FlexStrRef<'str, SIZE, BPAD, HPAD, HEAP>
where
    HEAP: Storage<str>,
{
    #[inline(always)]
    fn as_str_type(&self) -> &str {
        self.inner().as_str_type()
    }
}
