use alloc::rc::Rc;
use alloc::sync::Arc;
use std::ffi::OsStr;

use paste::paste;

use crate::inner::FlexStrInner;
use crate::traits::private;
use crate::traits::private::FlexStrCoreInner;
use crate::{define_flex_types, FlexStrCore, FlexStrCoreRef, Storage};

define_flex_types!("OsStr", OsStr, OsStr);

// *** FlexOsStr ***

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
    FlexStrCore<'str, SIZE, BPAD, HPAD, HEAP, OsStr> for FlexOsStrRef<'str, SIZE, BPAD, HPAD, HEAP>
where
    HEAP: Storage<OsStr>,
{
    #[inline(always)]
    fn as_str_type(&self) -> &OsStr {
        self.inner().as_str_type()
    }
}
