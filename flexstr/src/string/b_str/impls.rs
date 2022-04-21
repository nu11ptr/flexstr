use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::sync::Arc;

use bstr::BStr;
use paste::paste;

use crate::inner::FlexStrInner;
use crate::traits::private;
use crate::traits::private::FlexStrCoreInner;
use crate::{define_flex_types, FlexStrCore, FlexStrCoreRef, Storage};

define_flex_types!("BStr", BStr, [u8]);

// *** FlexBStr ***

impl<const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexStrCore<'static, SIZE, BPAD, HPAD, HEAP, BStr> for FlexBStr<SIZE, BPAD, HPAD, HEAP>
where
    HEAP: Storage<BStr>,
{
    #[inline(always)]
    fn as_str_type(&self) -> &BStr {
        self.inner().as_str_type()
    }
}

// *** FlexBStrRef ***

impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexStrCore<'str, SIZE, BPAD, HPAD, HEAP, BStr> for FlexBStrRef<'str, SIZE, BPAD, HPAD, HEAP>
where
    HEAP: Storage<BStr>,
{
    #[inline(always)]
    fn as_str_type(&self) -> &BStr {
        self.inner().as_str_type()
    }
}
