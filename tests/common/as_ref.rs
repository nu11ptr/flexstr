#![allow(dead_code)]

#[cfg(any(
    feature = "bytes",
    all(feature = "std", feature = "osstr"),
    all(feature = "std", feature = "path")
))]
use flexstry::{FlexStr, InlineFlexStr, RefCounted};

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

/// Test AsRef<[u8]> for InlineFlexStr<[u8]>
/// Input should be small enough to inline
#[cfg(feature = "bytes")]
pub fn test_as_ref_bytes_inline(s: &'static [u8]) {
    let inline_str = InlineFlexStr::<[u8]>::try_from_type(s)
        .expect("test input should be small enough to inline");
    let bytes_ref: &[u8] = inline_str.as_ref();
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

/// Test AsRef<OsStr> for InlineFlexStr<OsStr>
/// Input should be small enough to inline
#[cfg(all(feature = "std", feature = "osstr"))]
pub fn test_as_ref_osstr_inline(s: &'static std::ffi::OsStr) {
    let inline_str = InlineFlexStr::<std::ffi::OsStr>::try_from_type(s)
        .expect("test input should be small enough to inline");
    let os_str_ref: &std::ffi::OsStr = inline_str.as_ref();
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

/// Test AsRef<Path> for InlineFlexStr<Path>
/// Input should be small enough to inline
#[cfg(all(feature = "std", feature = "path"))]
pub fn test_as_ref_path_inline(s: &'static std::path::Path) {
    let inline_str = InlineFlexStr::<std::path::Path>::try_from_type(s)
        .expect("test input should be small enough to inline");
    let path_ref: &std::path::Path = inline_str.as_ref();
    assert_eq!(path_ref, s);
}
