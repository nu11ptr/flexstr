#[cfg(feature = "cstr")]
use alloc::ffi::CString;
#[cfg(all(not(feature = "std"), feature = "bytes"))]
use alloc::vec::Vec;
#[cfg(not(feature = "std"))]
use alloc::{borrow::ToOwned, boxed::Box, string::String};
#[cfg(feature = "cstr")]
use core::ffi::CStr;
#[cfg(all(feature = "std", feature = "osstr"))]
use std::ffi::{OsStr, OsString};
#[cfg(all(feature = "std", feature = "path"))]
use std::path::{Path, PathBuf};

// *** StringToFromBytes ***

/// Trait for string types that can be converted to and from bytes
pub trait StringToFromBytes: ToOwned + 'static {
    /// Convert bytes to a string type
    fn bytes_as_self(bytes: &[u8]) -> &Self;

    /// Convert a string type to bytes (excludes nul for CStr)
    #[inline]
    fn self_as_bytes(&self) -> &[u8] {
        self.self_as_raw_bytes()
    }

    /// Convert a string type to raw bytes (inludes nul for CStr)
    fn self_as_raw_bytes(&self) -> &[u8];
}

// *** StringFromBytesMut ***

/// Trait for string types that can be converted from bytes to mutable string reference
pub trait StringFromBytesMut: StringToFromBytes {
    /// Convert bytes to a mutable string reference
    fn bytes_as_self_mut(bytes: &mut [u8]) -> &mut Self;
}

// *** StringLike ***

/// Trait for string types that provide various operations
pub trait StringLike<S: ?Sized + StringToFromBytes>
where
    Self: Sized,
{
    /// Borrow a string reference as `&S`
    fn as_ref_type(&self) -> &S;

    /// Borrow the string as bytes
    fn as_bytes(&self) -> &[u8];

    /// Consume a string and convert it to an owned string. `S::to_owned` is called on Borrowed/Inlined/RefCounted variants.
    /// Boxed variants are converted directly into `S::Owned` (most likely without copy or allocation).
    fn into_owned_type(self) -> S::Owned
    where
        S::Owned: From<Box<S>>;

    /// Convert a string reference to an owned string. `S::to_owned` is called on all variants.
    fn to_owned_type(&self) -> S::Owned;

    /// Returns true if this is an empty string
    fn is_empty(&self) -> bool {
        self.as_bytes().is_empty()
    }

    /// Returns the length of this string in bytes
    fn len(&self) -> usize {
        self.as_bytes().len()
    }

    /// Borrow the string as an `&str`
    fn as_str(&self) -> &str
    where
        S: AsRef<str>,
    {
        self.as_ref_type().as_ref()
    }

    #[cfg(all(feature = "std", feature = "osstr"))]
    /// Borrow the string as an `&OsStr`
    fn as_os_str(&self) -> &OsStr
    where
        S: AsRef<OsStr>,
    {
        self.as_ref_type().as_ref()
    }

    #[cfg(all(feature = "std", feature = "path"))]
    /// Borrow the string as a `&Path`
    fn as_path(&self) -> &Path
    where
        S: AsRef<Path>,
    {
        self.as_ref_type().as_ref()
    }

    #[cfg(feature = "cstr")]
    /// Borrow the string as a `&CStr`
    fn as_c_str(&self) -> &CStr
    where
        S: AsRef<CStr>,
    {
        self.as_ref_type().as_ref()
    }

    /// Consume a string and convert it to a [String]
    fn into_string(self) -> String
    where
        S::Owned: Into<String> + From<Box<S>>,
    {
        self.into_owned_type().into()
    }

    #[cfg(all(feature = "std", feature = "osstr"))]
    /// Consume a string and convert it to an [OsString]
    fn into_os_string(self) -> OsString
    where
        S::Owned: Into<OsString> + From<Box<S>>,
    {
        self.into_owned_type().into()
    }

    #[cfg(all(feature = "std", feature = "path"))]
    /// Consume a string and convert it to a [PathBuf]
    fn into_path_buf(self) -> PathBuf
    where
        S::Owned: Into<PathBuf> + From<Box<S>>,
    {
        self.into_owned_type().into()
    }

    #[cfg(feature = "cstr")]
    /// Consume a string and convert it to a [CString]
    fn into_c_string(self) -> CString
    where
        S::Owned: Into<CString> + From<Box<S>>,
    {
        self.into_owned_type().into()
    }

    #[cfg(feature = "bytes")]
    /// Consume a string and convert it to a [`Vec<u8>`]
    fn into_vec_bytes(self) -> Vec<u8>
    where
        S::Owned: Into<Vec<u8>> + From<Box<S>>,
    {
        self.into_owned_type().into()
    }

    /// Convert a string reference to a [String]
    fn to_string(&self) -> String
    where
        S::Owned: Into<String>,
    {
        self.to_owned_type().into()
    }

    #[cfg(all(feature = "std", feature = "osstr"))]
    /// Convert a string reference to an [OsString]
    fn to_os_string(&self) -> OsString
    where
        S::Owned: Into<OsString>,
    {
        self.to_owned_type().into()
    }

    #[cfg(all(feature = "std", feature = "path"))]
    /// Convert a string reference to a [PathBuf]
    fn to_path_buf(&self) -> PathBuf
    where
        S::Owned: Into<PathBuf>,
    {
        self.to_owned_type().into()
    }

    #[cfg(feature = "cstr")]
    /// Convert a string reference to a [CString]
    fn to_c_string(&self) -> CString
    where
        S::Owned: Into<CString>,
    {
        self.to_owned_type().into()
    }

    #[cfg(feature = "bytes")]
    /// Convert a string reference to a [`Vec<u8>`]
    fn to_vec_bytes(&self) -> Vec<u8>
    where
        S::Owned: Into<Vec<u8>>,
    {
        self.to_owned_type().into()
    }
}
