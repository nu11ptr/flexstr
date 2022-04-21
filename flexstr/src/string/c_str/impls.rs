use alloc::rc::Rc;
use alloc::sync::Arc;
use std::ffi::CStr;

use paste::paste;

use crate::inner::FlexStrInner;
use crate::traits::private;
use crate::traits::private::FlexStrCoreInner;
use crate::{define_flex_types, FlexStrCore, FlexStrCoreRef, Storage};

define_flex_types!("CStr", CStr, [u8]);

// *** FlexCStr ***

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
    FlexStrCore<'str, SIZE, BPAD, HPAD, HEAP, CStr> for FlexCStrRef<'str, SIZE, BPAD, HPAD, HEAP>
where
    HEAP: Storage<CStr>,
{
    #[inline(always)]
    fn as_str_type(&self) -> &CStr {
        self.inner().as_str_type()
    }
}
