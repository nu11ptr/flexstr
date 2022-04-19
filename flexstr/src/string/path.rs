#![cfg(feature = "std")]

use alloc::rc::Rc;
use alloc::sync::Arc;
use core::convert::Infallible;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use paste::paste;

use crate::inner::FlexStrInner;
use crate::string::Str;
use crate::traits::private;
use crate::traits::private::FlexStrCoreInner;
use crate::{define_flex_types, FlexStrCore, FlexStrCoreRef, Storage};

#[cfg(unix)]
const RAW_EMPTY: &[u8] = b"";

impl Str for Path {
    type StringType = PathBuf;
    type HeapType = Path;
    type ConvertError = Infallible;

    #[cfg(unix)]
    #[inline]
    fn from_inline_data(bytes: &[u8]) -> &Self {
        use std::os::unix::ffi::OsStrExt;
        Path::new(OsStr::from_bytes(bytes))
    }

    #[cfg(not(unix))]
    #[inline]
    fn from_inline_data(_bytes: &[u8]) -> &Self {
        // TODO: Does os_str_bytes have a feature to help with this? Didn't see one
        unreachable!("Raw byte slice conversion not supported on this platform");
    }

    #[inline]
    fn from_heap_data(bytes: &Self::HeapType) -> &Self {
        bytes
    }

    #[cfg(unix)]
    #[inline]
    fn try_from_raw_data(bytes: &[u8]) -> Result<&Self, Self::ConvertError> {
        Ok(Self::from_inline_data(bytes))
    }

    #[cfg(not(unix))]
    #[inline]
    fn try_from_raw_data(bytes: &[u8]) -> Result<&Self, Self::ConvertError> {
        // TODO: Use os_str_bytes for platforms other than unix
        unreachable!("Raw byte slice conversion not supported on this platform")
    }

    #[cfg(unix)]
    #[inline(always)]
    fn empty(&self) -> Option<&'static Self> {
        if self.length() == 0 {
            Some(Self::from_inline_data(RAW_EMPTY))
        } else {
            None
        }
    }

    #[cfg(not(unix))]
    #[inline(always)]
    fn empty(&self) -> Option<&'static Self> {
        None
    }

    #[inline(always)]
    fn length(&self) -> usize {
        self.as_os_str().len()
    }

    #[inline]
    fn as_heap_type(&self) -> &Self::HeapType {
        self
    }

    #[cfg(unix)]
    #[inline(always)]
    fn as_inline_ptr(&self) -> *const u8 {
        use std::os::unix::ffi::OsStrExt;
        self.as_os_str().as_bytes() as *const [u8] as *const u8
    }

    #[cfg(not(unix))]
    #[inline]
    fn as_inline_ptr(&self) -> *const u8 {
        // TODO: Does os_str_bytes have a feature to help with this? Didn't see one
        unreachable!("Conversion back to raw pointer not supported on this platform");
    }
}

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
