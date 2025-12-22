#![allow(dead_code)]

use inline_flexstr::InlineFlexStr;

/// Test AsRef<[u8]> for InlineFlexStr<[u8]>
/// Input should be small enough to inline
#[cfg(feature = "bytes")]
pub fn test_as_ref_bytes(s: &'static [u8]) {
    let inline_str = InlineFlexStr::<[u8]>::try_from_type(s)
        .expect("test input should be small enough to inline");
    let bytes_ref: &[u8] = inline_str.as_ref();
    assert_eq!(bytes_ref, s);
}

/// Test AsRef<OsStr> for InlineFlexStr<OsStr>
/// Input should be small enough to inline
#[cfg(all(feature = "std", feature = "osstr"))]
pub fn test_as_ref_osstr(s: &'static std::ffi::OsStr) {
    let inline_str = InlineFlexStr::<std::ffi::OsStr>::try_from_type(s)
        .expect("test input should be small enough to inline");
    let os_str_ref: &std::ffi::OsStr = inline_str.as_ref();
    assert_eq!(os_str_ref, s);
}

/// Test AsRef<Path> for InlineFlexStr<Path>
/// Input should be small enough to inline
#[cfg(all(feature = "std", feature = "path"))]
pub fn test_as_ref_path(s: &'static std::path::Path) {
    let inline_str = InlineFlexStr::<std::path::Path>::try_from_type(s)
        .expect("test input should be small enough to inline");
    let path_ref: &std::path::Path = inline_str.as_ref();
    assert_eq!(path_ref, s);
}

/// Test AsRef<CStr> for InlineFlexStr<CStr>
/// Input should be small enough to inline
#[cfg(feature = "cstr")]
pub fn test_as_ref_cstr(s: &'static core::ffi::CStr) {
    let inline_str = InlineFlexStr::<core::ffi::CStr>::try_from_type(s)
        .expect("test input should be small enough to inline");
    let cstr_ref: &core::ffi::CStr = inline_str.as_ref();
    assert_eq!(cstr_ref.to_bytes(), s.to_bytes());
}

/// Test AsRef<str> for InlineFlexStr<str>
/// Input should be small enough to inline
#[cfg(feature = "str")]
pub fn test_as_ref_str(s: &'static str) {
    let inline_str = InlineFlexStr::<str>::try_from_type(s)
        .expect("test input should be small enough to inline");
    let str_ref: &str = inline_str.as_ref();
    assert_eq!(str_ref, s);
}

