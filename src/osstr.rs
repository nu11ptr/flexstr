#[cfg(not(feature = "std"))]
compile_error!("OsStr support is not available without the 'std' feature");

use alloc::{rc::Rc, sync::Arc};

use crate::{Flex, RefCounted, StringOps};
use std::ffi::OsStr;
#[cfg(feature = "path")]
use std::path::Path;

pub type LocalOsStr<'s> = Flex<'s, OsStr, Rc<OsStr>>;
pub type SharedOsStr<'s> = Flex<'s, OsStr, Arc<OsStr>>;

impl<R: RefCounted<OsStr>> Flex<'_, OsStr, R> {
    pub fn as_os_str(&self) -> &OsStr {
        self.as_borrowed_type()
    }

    #[cfg(feature = "path")]
    pub fn as_path(&self) -> &Path {
        self.as_os_str().as_ref()
    }
}

impl StringOps for OsStr {
    #[cfg(all(feature = "safe", target_family = "windows"))]
    #[inline(always)]
    fn bytes_as_self(bytes: &[u8]) -> &Self {
        // TODO: With a 3rd party crate, we could use: os_str_bytes::OsStrBytes::assert_from_raw_bytes()
        // But is this any better? They likely use unsafe internally anyway.
        compile_error!("OsStr support is not available with the 'safe' feature on Windows");
        unreachable!()
    }

    #[cfg(all(feature = "safe", target_family = "unix"))]
    #[inline(always)]
    fn bytes_as_self(bytes: &[u8]) -> &Self {
        use std::os::unix::prelude::OsStrExt;

        OsStrExt::from_bytes(bytes)
    }

    #[cfg(not(feature = "safe"))]
    #[inline(always)]
    fn bytes_as_self(bytes: &[u8]) -> &Self {
        // SAFETY: We know the bytes are a valid OsStr
        unsafe { OsStr::from_encoded_bytes_unchecked(bytes) }
    }

    fn self_as_bytes(&self) -> &[u8] {
        self.as_encoded_bytes()
    }
}

#[cfg(feature = "path")]
impl<R: RefCounted<OsStr>> AsRef<Path> for Flex<'_, OsStr, R> {
    fn as_ref(&self) -> &Path {
        self.as_path()
    }
}
