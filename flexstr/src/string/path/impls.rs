#![cfg(feature = "std")]

use alloc::rc::Rc;
use alloc::sync::Arc;
use std::path::Path;

use paste::paste;

use crate::inner::FlexStrInner;
use crate::string::Str;
use crate::traits::private;
use crate::traits::private::FlexStrCoreInner;
use crate::{define_flex_types, FlexStrCore, FlexStrCoreRef, Storage};

#[cfg(unix)]
pub(crate) const RAW_EMPTY: &[u8] = b"";

define_flex_types!("Path", Path, Path);

macro_rules! impl_body {
    () => {
        /// Creates a wrapped static string literal from a raw byte slice.
        #[cfg(unix)]
        #[cfg_attr(docsrs, doc(cfg(unix)))]
        #[inline]
        pub fn from_static_raw(s: &'static [u8]) -> Self {
            // I see no mention of const fn for these functions on unix - use trait
            Self(FlexStrInner::from_static(Path::from_inline_data(s)))
        }
    };
}

// *** FlexPath ***

impl<const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexPath<SIZE, BPAD, HPAD, HEAP>
{
    impl_body!();
}

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
    FlexPathRef<'str, SIZE, BPAD, HPAD, HEAP>
{
    impl_body!();
}

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
