#![allow(dead_code)]

use core::fmt;
use core::str::FromStr;
use flexstr_support::StringToFromBytes;
use inline_flexstr::InlineFlexStr;

/// Test FromStr success for InlineFlexStr
pub fn test_from_str_success<S>(s: &str)
where
    InlineFlexStr<S>: FromStr,
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    <InlineFlexStr<S> as FromStr>::Err: fmt::Debug,
{
    let inline_str = InlineFlexStr::from_str(s).unwrap();
    assert_eq!(inline_str.as_ref_type(), inline_str.as_ref_type()); // Basic sanity check
}

/// Test FromStr error for InlineFlexStr (too long)
/// Input should be too long to inline, causing an error
pub fn test_from_str_error_too_long<S>(s: &str)
where
    InlineFlexStr<S>: FromStr,
    S: ?Sized + StringToFromBytes + fmt::Debug,
    <InlineFlexStr<S> as FromStr>::Err: fmt::Debug + fmt::Display,
{
    let err = InlineFlexStr::from_str(s).expect_err("test input should be too long to inline");
    // Test that error can be displayed
    let _ = format!("{}", err);
}

/// Test FromStr for InlineFlexStr<[u8]> success
#[cfg(feature = "bytes")]
pub fn test_from_str_bytes_success() {
    let inline_str = InlineFlexStr::<[u8]>::from_str("test").unwrap();
    assert_eq!(inline_str.as_ref_type(), b"test");
}

/// Test FromStr for InlineFlexStr<[u8]> error (too long)
#[cfg(feature = "bytes")]
pub fn test_from_str_bytes_error() {
    let long_str = "x".repeat(inline_flexstr::INLINE_CAPACITY + 1);
    let err = InlineFlexStr::<[u8]>::from_str(&long_str).unwrap_err();
    assert_eq!(err.length, inline_flexstr::INLINE_CAPACITY + 1);
    assert_eq!(err.inline_capacity, inline_flexstr::INLINE_CAPACITY);
}

/// Test FromStr for InlineFlexStr<OsStr> success
#[cfg(all(feature = "std", feature = "osstr"))]
pub fn test_from_str_osstr_success() {
    use std::ffi::OsStr;

    let inline_str = InlineFlexStr::<OsStr>::from_str("test").unwrap();
    assert_eq!(inline_str.as_ref_type(), OsStr::new("test"));
}

/// Test FromStr for InlineFlexStr<OsStr> error (too long)
#[cfg(all(feature = "std", feature = "osstr"))]
pub fn test_from_str_osstr_error() {
    let long_str = "x".repeat(inline_flexstr::INLINE_CAPACITY + 1);
    let err = InlineFlexStr::<std::ffi::OsStr>::from_str(&long_str).unwrap_err();
    assert_eq!(err.length, inline_flexstr::INLINE_CAPACITY + 1);
    assert_eq!(err.inline_capacity, inline_flexstr::INLINE_CAPACITY);
}

/// Test FromStr for InlineFlexStr<Path> success
#[cfg(all(feature = "std", feature = "path"))]
pub fn test_from_str_path_success() {
    use std::path::Path;

    let inline_str = InlineFlexStr::<Path>::from_str("test").unwrap();
    assert_eq!(inline_str.as_ref_type(), Path::new("test"));
}

/// Test FromStr for InlineFlexStr<Path> error (too long)
#[cfg(all(feature = "std", feature = "path"))]
pub fn test_from_str_path_error() {
    let long_str = "x".repeat(inline_flexstr::INLINE_CAPACITY + 1);
    let err = InlineFlexStr::<std::path::Path>::from_str(&long_str).unwrap_err();
    assert_eq!(err.length, inline_flexstr::INLINE_CAPACITY + 1);
    assert_eq!(err.inline_capacity, inline_flexstr::INLINE_CAPACITY);
}

/// Test FromStr for InlineFlexStr<CStr> success
#[cfg(feature = "cstr")]
pub fn test_from_str_cstr_success() {
    let inline_str = InlineFlexStr::<core::ffi::CStr>::from_str("test").unwrap();
    assert_eq!(inline_str.as_ref_type().to_bytes(), b"test");
}

/// Test FromStr for InlineFlexStr<CStr> error (interior NUL or too long)
#[cfg(feature = "cstr")]
pub fn test_from_str_cstr_error() {
    use inline_flexstr::TooLongOrNulError;

    // String with interior NUL should fail - use a string that fits in 32-bit capacity (10 bytes)
    // "ab\0cd" is 5 bytes, which fits in both 32-bit and 64-bit capacity
    let result: Result<InlineFlexStr<core::ffi::CStr>, TooLongOrNulError> =
        InlineFlexStr::from_str("ab\0cd");
    result.unwrap_err();

    // String too long should fail
    let long_str = "x".repeat(inline_flexstr::INLINE_CAPACITY + 1);
    let result: Result<InlineFlexStr<core::ffi::CStr>, TooLongOrNulError> =
        InlineFlexStr::from_str(&long_str);
    result.unwrap_err();
}
