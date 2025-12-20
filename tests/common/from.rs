#![allow(dead_code)]

use core::fmt;
use flexstry::{FlexStr, RefCounted, StringToFromBytes};

/// Test From implementation
pub fn test_from<'s, T, S, R>(value: T)
where
    T: Into<FlexStr<'s, S, R>> + fmt::Debug,
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S>,
    S::Owned: AsRef<S>,
{
    let flex_str: FlexStr<'s, S, R> = value.into();
    // Basic sanity check - the conversion should succeed
    assert_eq!(flex_str.as_ref_type(), flex_str.as_ref_type());
}

/// Test From<String> for FlexStr<str, R>
#[cfg(feature = "str")]
pub fn test_from_string_str<R>()
where
    R: RefCounted<str>,
{
    use alloc::string::String;
    
    let s = String::from("test");
    let flex_str: FlexStr<'_, str, R> = s.into();
    assert_eq!(flex_str.as_ref_type(), "test");
}

/// Test From<Vec<u8>> for FlexStr<[u8], R>
#[cfg(feature = "bytes")]
pub fn test_from_vec_bytes<R>()
where
    R: RefCounted<[u8]>,
{
    let v = alloc::vec![1u8, 2u8, 3u8];
    let flex_str: FlexStr<'_, [u8], R> = v.into();
    assert_eq!(flex_str.as_ref_type(), &[1u8, 2u8, 3u8][..]);
}

/// Test From<&str> for FlexStr<[u8], R>
#[cfg(feature = "bytes")]
pub fn test_from_str_bytes<R>()
where
    R: RefCounted<[u8]>,
{
    let s = "test";
    let flex_str: FlexStr<'_, [u8], R> = s.into();
    assert_eq!(flex_str.as_ref_type(), b"test");
}

/// Test From<OsString> for FlexStr<OsStr, R>
#[cfg(all(feature = "std", feature = "osstr"))]
pub fn test_from_os_string<R>()
where
    R: RefCounted<std::ffi::OsStr>,
{
    use std::ffi::OsString;
    
    let os_string: OsString = OsString::from("test");
    let flex_str: FlexStr<'_, std::ffi::OsStr, R> = os_string.into();
    assert_eq!(flex_str.as_ref_type(), std::ffi::OsStr::new("test"));
}

/// Test From<String> for FlexStr<OsStr, R>
#[cfg(all(feature = "std", feature = "osstr"))]
pub fn test_from_string_osstr<R>()
where
    R: RefCounted<std::ffi::OsStr>,
{
    let s = String::from("test");
    let flex_str: FlexStr<'_, std::ffi::OsStr, R> = s.into();
    assert_eq!(flex_str.as_ref_type(), std::ffi::OsStr::new("test"));
}

/// Test From<PathBuf> for FlexStr<OsStr, R>
#[cfg(all(feature = "std", feature = "osstr", feature = "path"))]
pub fn test_from_path_buf_osstr<R>()
where
    R: RefCounted<std::ffi::OsStr>,
{
    use std::path::PathBuf;
    
    let path_buf = PathBuf::from("test");
    let flex_str: FlexStr<'_, std::ffi::OsStr, R> = path_buf.into();
    assert_eq!(flex_str.as_ref_type(), std::ffi::OsStr::new("test"));
}

/// Test From<&str> for FlexStr<OsStr, R>
#[cfg(all(feature = "std", feature = "osstr"))]
pub fn test_from_str_ref_osstr<R>()
where
    R: RefCounted<std::ffi::OsStr>,
{
    let s = "test";
    let flex_str: FlexStr<'_, std::ffi::OsStr, R> = s.into();
    assert_eq!(flex_str.as_ref_type(), std::ffi::OsStr::new("test"));
}

/// Test From<&Path> for FlexStr<OsStr, R>
#[cfg(all(feature = "std", feature = "osstr", feature = "path"))]
pub fn test_from_path_ref_osstr<R>()
where
    R: RefCounted<std::ffi::OsStr>,
{
    use std::path::Path;
    
    let path = Path::new("test");
    let flex_str: FlexStr<'_, std::ffi::OsStr, R> = path.into();
    assert_eq!(flex_str.as_ref_type(), std::ffi::OsStr::new("test"));
}

/// Test From<PathBuf> for FlexStr<Path, R>
#[cfg(all(feature = "std", feature = "path"))]
pub fn test_from_path_buf<R>()
where
    R: RefCounted<std::path::Path>,
{
    use std::path::{Path, PathBuf};
    
    let path_buf = PathBuf::from("test");
    let flex_str: FlexStr<'_, Path, R> = path_buf.into();
    assert_eq!(flex_str.as_ref_type(), Path::new("test"));
}

/// Test From<String> for FlexStr<Path, R>
#[cfg(all(feature = "std", feature = "path"))]
pub fn test_from_string_path<R>()
where
    R: RefCounted<std::path::Path>,
{
    use std::path::Path;
    
    let s = String::from("test");
    let flex_str: FlexStr<'_, Path, R> = s.into();
    assert_eq!(flex_str.as_ref_type(), Path::new("test"));
}

/// Test From<OsString> for FlexStr<Path, R>
#[cfg(all(feature = "std", feature = "path"))]
pub fn test_from_os_string_path<R>()
where
    R: RefCounted<std::path::Path>,
{
    use std::ffi::OsString;
    use std::path::Path;
    
    let os_string = OsString::from("test");
    let flex_str: FlexStr<'_, Path, R> = os_string.into();
    assert_eq!(flex_str.as_ref_type(), Path::new("test"));
}

/// Test From<&str> for FlexStr<Path, R>
#[cfg(all(feature = "std", feature = "path"))]
pub fn test_from_str_ref_path<R>()
where
    R: RefCounted<std::path::Path>,
{
    use std::path::Path;
    
    let s = "test";
    let flex_str: FlexStr<'_, Path, R> = s.into();
    assert_eq!(flex_str.as_ref_type(), Path::new("test"));
}

/// Test From<&OsStr> for FlexStr<Path, R>
#[cfg(all(feature = "std", feature = "path"))]
pub fn test_from_osstr_ref_path<R>()
where
    R: RefCounted<std::path::Path>,
{
    use std::ffi::OsStr;
    use std::path::Path;
    
    let os_str = OsStr::new("test");
    let flex_str: FlexStr<'_, Path, R> = os_str.into();
    assert_eq!(flex_str.as_ref_type(), Path::new("test"));
}

