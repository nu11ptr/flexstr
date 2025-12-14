use alloc::{rc::Rc, string::String, sync::Arc};
#[cfg(feature = "osstr")]
use std::ffi::OsStr;
#[cfg(feature = "path")]
use std::path::Path;

use crate::{FlexStr, RefCounted, StringOps};

/// Local `str` type (NOTE: This can't be shared between threads)
pub type LocalStr<'s> = FlexStr<'s, str, Rc<str>>;

/// Shared `str` type
pub type SharedStr<'s> = FlexStr<'s, str, Arc<str>>;

const _: () = assert!(
    size_of::<Option<LocalStr>>() <= size_of::<String>(),
    "Option<LocalStr> must be less than or equal to the size of String"
);
const _: () = assert!(
    size_of::<Option<SharedStr>>() <= size_of::<String>(),
    "Option<SharedStr> must be less than or equal to the size of String"
);

impl<R: RefCounted<str>> FlexStr<'_, str, R> {
    /// Borrow the str as an `&str`
    pub fn as_str(&self) -> &str {
        self.as_borrowed_type()
    }

    #[cfg(feature = "osstr")]
    /// Borrow the str as an `&OsStr`
    pub fn as_os_str(&self) -> &OsStr {
        self.as_str().as_ref()
    }

    #[cfg(feature = "path")]
    /// Borrow the str as a `&Path`
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

    #[inline(always)]
    fn self_as_raw_bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}

// *** From<String> ***

// NOTE: Cannot be implemented generically because of impl<T> From<T> for T
impl<'s, R: RefCounted<str>> From<String> for FlexStr<'s, str, R> {
    #[inline(always)]
    fn from(s: String) -> Self {
        FlexStr::from_owned(s)
    }
}

// *** AsRef<OsStr>, AsRef<Path>, and AsRef<[u8]> ***

// NOTE: Cannot be implemented generically because it conflicts with AsRef<S> for Bytes
impl<'s, R: RefCounted<str>> AsRef<[u8]> for FlexStr<'s, str, R> {
    #[inline(always)]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

#[cfg(feature = "osstr")]
impl<R: RefCounted<str>> AsRef<OsStr> for FlexStr<'_, str, R> {
    fn as_ref(&self) -> &OsStr {
        self.as_os_str()
    }
}

#[cfg(feature = "path")]
impl<R: RefCounted<str>> AsRef<Path> for FlexStr<'_, str, R> {
    fn as_ref(&self) -> &Path {
        self.as_path()
    }
}
