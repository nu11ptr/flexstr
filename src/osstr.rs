#[cfg(feature = "safe")]
compile_error!("OsStr support is not available with the 'safe' feature");

#[cfg(not(feature = "std"))]
compile_error!("OsStr support is not available without the 'std' feature");

use alloc::{boxed::Box, rc::Rc, sync::Arc};
use core::ops::Deref;

use std::ffi::{OsStr, OsString};

use crate::Flex;

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
}

#[cfg(not(feature = "safe"))]
#[inline(always)]
fn bytes_as_os_str(bytes: &[u8]) -> &OsStr {
    // SAFETY: We know the bytes are a valid OsStr
    unsafe { OsStr::from_encoded_bytes_unchecked(bytes) }
}
