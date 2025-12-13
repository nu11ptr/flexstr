use alloc::{rc::Rc, string::String, sync::Arc};
#[cfg(feature = "osstr")]
use std::ffi::OsStr;
#[cfg(feature = "path")]
use std::path::Path;

use crate::{Flex, RefCounted, StringOps};

pub type LocalStr<'s> = Flex<'s, str, Rc<str>>;
pub type SharedStr<'s> = Flex<'s, str, Arc<str>>;

const _: () = assert!(
    size_of::<Option<LocalStr>>() <= size_of::<String>(),
    "Option<LocalStr> must be exactly the same size as String"
);
const _: () = assert!(
    size_of::<Option<SharedStr>>() <= size_of::<String>(),
    "Option<SharedStr> must be exactly the same size as String"
);

impl<R: RefCounted<str>> Flex<'_, str, R> {
    pub fn as_str(&self) -> &str {
        self.as_borrowed_type()
    }

    #[cfg(feature = "osstr")]
    pub fn as_os_str(&self) -> &OsStr {
        self.as_str().as_ref()
    }

    #[cfg(feature = "path")]
    pub fn as_path(&self) -> &Path {
        self.as_str().as_ref()
    }
}

impl StringOps for str {
    #[cfg(feature = "safe")]
    #[inline(always)]
    fn bytes_as_self(bytes: &[u8]) -> &Self {
        // PANIC SAFETY: We know the bytes are valid UTF-8
        str::from_utf8(bytes).expect("Invalid UTF-8")
    }

    #[cfg(not(feature = "safe"))]
    #[inline(always)]
    fn bytes_as_self(bytes: &[u8]) -> &Self {
        // SAFETY: We know the bytes are valid UTF-8
        unsafe { str::from_utf8_unchecked(bytes) }
    }

    fn self_as_bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}

// *** From<String> ***

// NOTE: Cannot be implemented generically because of impl<T> From<T> for T
impl<'s, R: RefCounted<str>> From<String> for Flex<'s, str, R> {
    #[inline(always)]
    fn from(s: String) -> Self {
        Flex::from_owned(s)
    }
}

// *** AsRef<OsStr> and AsRef<Path> ***

#[cfg(feature = "osstr")]
impl<R: RefCounted<str>> AsRef<OsStr> for Flex<'_, str, R> {
    fn as_ref(&self) -> &OsStr {
        self.as_os_str()
    }
}

#[cfg(feature = "path")]
impl<R: RefCounted<str>> AsRef<Path> for Flex<'_, str, R> {
    fn as_ref(&self) -> &Path {
        self.as_path()
    }
}
