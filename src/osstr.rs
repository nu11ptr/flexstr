#[cfg(not(feature = "std"))]
compile_error!("OsStr support is not available without the 'std' feature");

use alloc::{boxed::Box, rc::Rc, sync::Arc};
use core::ops::Deref;

use crate::Flex;
use std::ffi::{OsStr, OsString};
#[cfg(feature = "path")]
use std::path::Path;

pub type LocalOsStr<'s> = Flex<'s, OsStr, Rc<OsStr>>;
pub type SharedOsStr<'s> = Flex<'s, OsStr, Arc<OsStr>>;
pub type BoxOsStr<'s> = Flex<'s, OsStr, Box<OsStr>>;

pub type LocalOsString<'s> = Flex<'s, OsString, Rc<OsString>>;
pub type SharedOsString<'s> = Flex<'s, OsString, Arc<OsString>>;
pub type BoxOsString<'s> = Flex<'s, OsString, Box<OsString>>;

impl<S: Deref<Target = OsStr>> Flex<'_, OsStr, S> {
    pub fn as_os_str(&self) -> &OsStr {
        match self {
            Flex::Borrowed(s) => s,
            Flex::Inlined(inline) => bytes_as_os_str(inline.as_ref()),
            Flex::Stored(a) => &*a,
        }
    }

    #[cfg(feature = "path")]
    pub fn as_path(&self) -> &Path {
        Path::new(self.as_os_str())
    }
}

#[cfg(all(feature = "safe", target_family = "windows"))]
#[inline(always)]
pub(crate) fn bytes_as_os_str(bytes: &[u8]) -> &OsStr {
    // TODO: With a 3rd party crate, we could use: os_str_bytes::OsStrBytes::assert_from_raw_bytes()
    // But is this any better? They likely use unsafe internally anyway.
    compile_error!("OsStr support is not available with the 'safe' feature on Windows");
    unreachable!()
}

#[cfg(all(feature = "safe", target_family = "unix"))]
#[inline(always)]
pub(crate) fn bytes_as_os_str(bytes: &[u8]) -> &OsStr {
    use std::os::unix::prelude::OsStrExt;

    OsStrExt::from_bytes(bytes)
}

#[cfg(not(feature = "safe"))]
#[inline(always)]
pub(crate) fn bytes_as_os_str(bytes: &[u8]) -> &OsStr {
    // SAFETY: We know the bytes are a valid OsStr
    unsafe { OsStr::from_encoded_bytes_unchecked(bytes) }
}
