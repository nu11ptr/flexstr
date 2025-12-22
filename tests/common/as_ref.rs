#![allow(dead_code)]

use flexstry::{FlexStr, RefCounted};

/// Test AsRef<[u8]> for FlexStr<[u8], R>
#[cfg(feature = "bytes")]
pub fn test_as_ref_bytes_flex_str<R>(s: &'static [u8])
where
    R: RefCounted<[u8]>,
{
    let flex_str: FlexStr<'_, [u8], R> = FlexStr::from_borrowed(s);
    let bytes_ref: &[u8] = flex_str.as_ref();
    assert_eq!(bytes_ref, s);
}

/// Test AsRef<OsStr> for FlexStr<OsStr, R>
#[cfg(all(feature = "std", feature = "osstr"))]
pub fn test_as_ref_osstr_flex_str<R>(s: &'static std::ffi::OsStr)
where
    R: RefCounted<std::ffi::OsStr>,
{
    let flex_str: FlexStr<'_, std::ffi::OsStr, R> = FlexStr::from_borrowed(s);
    let os_str_ref: &std::ffi::OsStr = flex_str.as_ref();
    assert_eq!(os_str_ref, s);
}

/// Test AsRef<Path> for FlexStr<Path, R>
#[cfg(all(feature = "std", feature = "path"))]
pub fn test_as_ref_path_flex_str<R>(s: &'static std::path::Path)
where
    R: RefCounted<std::path::Path>,
{
    let flex_str: FlexStr<'_, std::path::Path, R> = FlexStr::from_borrowed(s);
    let path_ref: &std::path::Path = flex_str.as_ref();
    assert_eq!(path_ref, s);
}

/// Test AsRef<CStr> for FlexStr<CStr, R>
#[cfg(feature = "cstr")]
pub fn test_as_ref_cstr_flex_str<R>(s: &'static core::ffi::CStr)
where
    R: RefCounted<core::ffi::CStr>,
{
    let flex_str: FlexStr<'_, core::ffi::CStr, R> = FlexStr::from_borrowed(s);
    let cstr_ref: &core::ffi::CStr = flex_str.as_ref();
    assert_eq!(cstr_ref.to_bytes(), s.to_bytes());
}

/// Test AsRef<str> for FlexStr<str, R>
#[cfg(feature = "str")]
pub fn test_as_ref_str_flex_str<R>(s: &'static str)
where
    R: RefCounted<str>,
{
    let flex_str: FlexStr<'_, str, R> = FlexStr::from_borrowed(s);
    let str_ref: &str = flex_str.as_ref();
    assert_eq!(str_ref, s);
}
