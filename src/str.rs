use alloc::{rc::Rc, string::String, sync::Arc};
#[cfg(all(feature = "std", feature = "osstr"))]
use std::ffi::OsStr;
#[cfg(all(feature = "std", feature = "path"))]
use std::path::Path;

use crate::{FlexStr, InlineFlexStr, RefCounted, StringOps};

/// Local `str` type (NOTE: This can't be shared between threads)
pub type LocalStr<'s> = FlexStr<'s, str, Rc<str>>;

/// Shared `str` type
pub type SharedStr<'s> = FlexStr<'s, str, Arc<str>>;

/// Inline `str` type
pub type InlineStr = InlineFlexStr<str>;

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

    #[cfg(all(feature = "std", feature = "osstr"))]
    /// Borrow the str as an `&OsStr`
    pub fn as_os_str(&self) -> &OsStr {
        self.as_str().as_ref()
    }

    #[cfg(all(feature = "std", feature = "path"))]
    /// Borrow the str as a `&Path`
    pub fn as_path(&self) -> &Path {
        self.as_str().as_ref()
    }
}

impl InlineStr {
    /// Borrow the str as an `&str`
    #[inline]
    pub fn as_str(&self) -> &str {
        self.as_borrowed_type()
    }

    /// Borrow the str as an `&OsStr`
    #[cfg(all(feature = "std", feature = "osstr"))]
    #[inline]
    pub fn as_os_str(&self) -> &OsStr {
        self.as_str().as_ref()
    }

    /// Borrow the str as a `&Path`
    #[cfg(all(feature = "std", feature = "path"))]
    #[inline]
    pub fn as_path(&self) -> &Path {
        self.as_str().as_ref()
    }
}

impl StringOps for str {
    #[cfg(feature = "safe")]
    #[inline]
    fn bytes_as_self(bytes: &[u8]) -> &Self {
        // PANIC SAFETY: We know the bytes are valid UTF-8
        str::from_utf8(bytes).expect("Invalid UTF-8")
    }

    #[cfg(not(feature = "safe"))]
    #[inline]
    fn bytes_as_self(bytes: &[u8]) -> &Self {
        // SAFETY: We know the bytes are valid UTF-8
        unsafe { str::from_utf8_unchecked(bytes) }
    }

    #[inline]
    fn self_as_raw_bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}

// *** From<String> ***

// NOTE: Cannot be implemented generically because of impl<T> From<T> for T
impl<'s, R: RefCounted<str>> From<String> for FlexStr<'s, str, R> {
    fn from(s: String) -> Self {
        FlexStr::from_owned(s)
    }
}

// *** TryFrom<&str> for InlineFlexStr ***

// NOTE: Cannot be implemented generically because of impl<T> TryFrom<T> for T
impl<'s> TryFrom<&'s str> for InlineFlexStr<str> {
    type Error = &'s str;

    #[inline]
    fn try_from(s: &'s str) -> Result<Self, Self::Error> {
        InlineFlexStr::try_from_type(s)
    }
}

// *** AsRef<OsStr>, AsRef<Path>, and AsRef<[u8]> ***

// NOTE: Cannot be implemented generically because it conflicts with AsRef<S> for Bytes
impl<'s, R: RefCounted<str>> AsRef<[u8]> for FlexStr<'s, str, R> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

#[cfg(all(feature = "std", feature = "osstr"))]
impl<R: RefCounted<str>> AsRef<OsStr> for FlexStr<'_, str, R> {
    fn as_ref(&self) -> &OsStr {
        self.as_os_str()
    }
}

#[cfg(all(feature = "std", feature = "path"))]
impl<R: RefCounted<str>> AsRef<Path> for FlexStr<'_, str, R> {
    fn as_ref(&self) -> &Path {
        self.as_path()
    }
}

// *** AsRef<OsStr>, AsRef<Path>, and AsRef<[u8]> for InlineFlexStr ***

// NOTE: Cannot be implemented generically because it conflicts with AsRef<S> for Bytes
impl AsRef<[u8]> for InlineFlexStr<str> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

#[cfg(all(feature = "std", feature = "osstr"))]
impl AsRef<OsStr> for InlineFlexStr<str> {
    #[inline]
    fn as_ref(&self) -> &OsStr {
        self.as_os_str()
    }
}

#[cfg(all(feature = "std", feature = "path"))]
impl AsRef<Path> for InlineFlexStr<str> {
    #[inline]
    fn as_ref(&self) -> &Path {
        self.as_path()
    }
}
