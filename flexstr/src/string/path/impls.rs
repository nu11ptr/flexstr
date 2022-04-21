use alloc::rc::Rc;
use alloc::sync::Arc;
use std::path::Path;

use paste::paste;

use crate::inner::FlexStrInner;
use crate::traits::private;
use crate::traits::private::FlexStrCoreInner;
use crate::{define_flex_types, FlexStrCore, FlexStrCoreRef, Storage};

define_flex_types!("Path", Path, Path);

// *** FlexPath ***

impl<const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexStrCore<'static, SIZE, BPAD, HPAD, HEAP, Path> for FlexPath<SIZE, BPAD, HPAD, HEAP>
where
    HEAP: Storage<Path>,
{
    #[inline(always)]
    fn as_str_type(&self) -> &Path {
        self.inner().as_str_type()
    }
}

// *** FlexPathRef ***

impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexStrCore<'str, SIZE, BPAD, HPAD, HEAP, Path> for FlexPathRef<'str, SIZE, BPAD, HPAD, HEAP>
where
    HEAP: Storage<Path>,
{
    #[inline(always)]
    fn as_str_type(&self) -> &Path {
        self.inner().as_str_type()
    }
}
