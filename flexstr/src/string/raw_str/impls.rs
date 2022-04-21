use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::sync::Arc;

use paste::paste;

use crate::inner::FlexStrInner;
use crate::traits::private;
use crate::traits::private::FlexStrCoreInner;
use crate::{define_flex_types, FlexStrCore, FlexStrCoreRef, Storage};

define_flex_types!("RawStr", [u8], [u8]);

// *** FlexRawStr ***

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
    FlexStrCore<'str, SIZE, BPAD, HPAD, HEAP, [u8]> for FlexRawStrRef<'str, SIZE, BPAD, HPAD, HEAP>
where
    HEAP: Storage<[u8]>,
{
    #[inline(always)]
    fn as_str_type(&self) -> &[u8] {
        self.inner().as_str_type()
    }
}
