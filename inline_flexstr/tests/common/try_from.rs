#![allow(dead_code)]

use core::fmt;
use flexstr_support::StringToFromBytes;
use inline_flexstr::InlineFlexStr;

/// Test TryFrom success path for InlineFlexStr
pub fn test_try_from_success<T, S>(value: T)
where
    T: core::convert::TryInto<InlineFlexStr<S>>,
    T::Error: fmt::Debug,
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
{
    let inline_str: InlineFlexStr<S> = value.try_into().unwrap();
    assert_eq!(inline_str.as_ref_type(), inline_str.as_ref_type()); // Basic sanity check
}

/// Test TryFrom error path for InlineFlexStr
pub fn test_try_from_error<T, S>(value: T)
where
    T: core::convert::TryInto<InlineFlexStr<S>>,
    T::Error: fmt::Debug + fmt::Display,
    S: ?Sized + StringToFromBytes + fmt::Debug,
{
    let result: Result<InlineFlexStr<S>, T::Error> = value.try_into();
    let err = result.unwrap_err();
    // Test that error can be displayed
    let _ = format!("{}", err);
}

/// Test TryFrom<&[u8]> for InlineFlexStr<[u8]> with too long string
#[cfg(feature = "bytes")]
pub fn test_try_from_bytes_too_long() {
    // Create a byte slice that's definitely too long
    let long_bytes = vec![0u8; inline_flexstr::INLINE_CAPACITY + 1];
    let result: Result<InlineFlexStr<[u8]>, inline_flexstr::TooLongForInlining> =
        long_bytes.as_slice().try_into();
    let err = result.unwrap_err();
    assert_eq!(err.length, inline_flexstr::INLINE_CAPACITY + 1);
    assert_eq!(err.inline_capacity, inline_flexstr::INLINE_CAPACITY);
}

/// Test TryFrom<&str> for InlineFlexStr<[u8]> with too long string
#[cfg(feature = "bytes")]
pub fn test_try_from_str_too_long() {
    // Create a string that's definitely too long
    let long_str = "x".repeat(inline_flexstr::INLINE_CAPACITY + 1);
    let result: Result<InlineFlexStr<[u8]>, inline_flexstr::TooLongForInlining> =
        long_str.as_str().try_into();
    let _err = result.unwrap_err();
}

/// Test TryFrom<&OsStr> for InlineFlexStr<OsStr> with too long string
#[cfg(all(feature = "std", feature = "osstr"))]
pub fn test_try_from_osstr_too_long() {
    use std::ffi::OsStr;

    // Create an OsStr that's definitely too long
    let long_str = "x".repeat(inline_flexstr::INLINE_CAPACITY + 1);
    let os_str = OsStr::new(&long_str);
    let result: Result<InlineFlexStr<OsStr>, inline_flexstr::TooLongForInlining> =
        os_str.try_into();
    let _err = result.unwrap_err();
}

/// Test TryFrom<&str> for InlineFlexStr<OsStr> with too long string
#[cfg(all(feature = "std", feature = "osstr"))]
pub fn test_try_from_str_osstr_too_long() {
    // Create a string that's definitely too long
    let long_str = "x".repeat(inline_flexstr::INLINE_CAPACITY + 1);
    let result: Result<InlineFlexStr<std::ffi::OsStr>, inline_flexstr::TooLongForInlining> =
        long_str.as_str().try_into();
    let _err = result.unwrap_err();
}

/// Test TryFrom<&Path> for InlineFlexStr<OsStr> with too long string
#[cfg(all(feature = "std", feature = "osstr", feature = "path"))]
pub fn test_try_from_path_osstr_too_long() {
    use std::path::Path;

    // Create a path that's definitely too long
    let long_str = "x".repeat(inline_flexstr::INLINE_CAPACITY + 1);
    let path = Path::new(&long_str);
    let result: Result<InlineFlexStr<std::ffi::OsStr>, inline_flexstr::TooLongForInlining> =
        path.try_into();
    let _err = result.unwrap_err();
}

/// Test TryFrom<&Path> for InlineFlexStr<Path> with too long string
#[cfg(all(feature = "std", feature = "path"))]
pub fn test_try_from_path_too_long() {
    use std::path::Path;

    // Create a path that's definitely too long
    let long_str = "x".repeat(inline_flexstr::INLINE_CAPACITY + 1);
    let path = Path::new(&long_str);
    let result: Result<InlineFlexStr<Path>, inline_flexstr::TooLongForInlining> = path.try_into();
    let _err = result.unwrap_err();
}

/// Test TryFrom<&str> for InlineFlexStr<Path> with too long string
#[cfg(all(feature = "std", feature = "path"))]
pub fn test_try_from_str_path_too_long() {
    // Create a string that's definitely too long
    let long_str = "x".repeat(inline_flexstr::INLINE_CAPACITY + 1);
    let result: Result<InlineFlexStr<std::path::Path>, inline_flexstr::TooLongForInlining> =
        long_str.as_str().try_into();
    let _err = result.unwrap_err();
}

/// Test TryFrom<&OsStr> for InlineFlexStr<Path> with too long string
#[cfg(all(feature = "std", feature = "path"))]
pub fn test_try_from_osstr_path_too_long() {
    use std::ffi::OsStr;

    // Create an OsStr that's definitely too long
    let long_str = "x".repeat(inline_flexstr::INLINE_CAPACITY + 1);
    let os_str = OsStr::new(&long_str);
    let result: Result<InlineFlexStr<std::path::Path>, inline_flexstr::TooLongForInlining> =
        os_str.try_into();
    let _err = result.unwrap_err();
}

/// Test TryFrom<&str> for InlineFlexStr<CStr>
#[cfg(feature = "cstr")]
pub fn test_try_from_str_cstr() {
    use inline_flexstr::TooLongOrNulError;

    // Valid CStr (no interior NUL, small enough)
    let s: &str = "test";
    let result: Result<InlineFlexStr<core::ffi::CStr>, TooLongOrNulError> = s.try_into();
    let inline_str = result.unwrap();
    assert_eq!(inline_str.as_ref_type().to_bytes(), b"test");

    // Invalid CStr (interior NUL)
    let s_with_nul: &str = "test\0middle";
    let result: Result<InlineFlexStr<core::ffi::CStr>, TooLongOrNulError> = s_with_nul.try_into();
    result.unwrap_err();

    // Too long
    let long_str = "x".repeat(inline_flexstr::INLINE_CAPACITY + 1);
    let result: Result<InlineFlexStr<core::ffi::CStr>, TooLongOrNulError> =
        long_str.as_str().try_into();
    result.unwrap_err();
}

/// Test TryFrom<&[u8]> for InlineFlexStr<CStr>
#[cfg(feature = "cstr")]
pub fn test_try_from_bytes_cstr() {
    use inline_flexstr::TooLongOrNulError;

    // Valid CStr (no interior NUL, small enough)
    let bytes: &[u8] = b"test";
    let result: Result<InlineFlexStr<core::ffi::CStr>, TooLongOrNulError> = bytes.try_into();
    let inline_str = result.unwrap();
    assert_eq!(inline_str.as_ref_type().to_bytes(), b"test");

    // Invalid CStr (interior NUL)
    let bytes_with_nul: &[u8] = b"test\0middle";
    let result: Result<InlineFlexStr<core::ffi::CStr>, TooLongOrNulError> =
        bytes_with_nul.try_into();
    result.unwrap_err();

    // Too long
    let long_bytes = vec![b'x'; inline_flexstr::INLINE_CAPACITY + 1];
    let result: Result<InlineFlexStr<core::ffi::CStr>, TooLongOrNulError> =
        long_bytes.as_slice().try_into();
    result.unwrap_err();
}

/// Test TryFrom<&[u8]> for InlineFlexStr<str>
#[cfg(feature = "str")]
pub fn test_try_from_bytes_str() {
    use inline_flexstr::TooLongOrUtf8Error;

    // Valid UTF-8 bytes, small enough
    let bytes: &[u8] = b"test";
    let result: Result<InlineFlexStr<str>, TooLongOrUtf8Error> = bytes.try_into();
    let inline_str = result.unwrap();
    assert_eq!(inline_str.as_ref_type(), "test");

    // Invalid UTF-8 bytes
    let invalid_bytes: &[u8] = &[0xFF, 0xFF, 0xFF];
    let result: Result<InlineFlexStr<str>, TooLongOrUtf8Error> = invalid_bytes.try_into();
    result.unwrap_err();

    // Too long
    let long_bytes = vec![b'x'; inline_flexstr::INLINE_CAPACITY + 1];
    let result: Result<InlineFlexStr<str>, TooLongOrUtf8Error> = long_bytes.as_slice().try_into();
    result.unwrap_err();
}

/// Test TryFrom<&OsStr> for InlineFlexStr<str>
#[cfg(all(feature = "str", feature = "std"))]
pub fn test_try_from_osstr_str() {
    use inline_flexstr::TooLongOrUtf8Error;
    use std::ffi::OsStr;

    // Valid UTF-8 OsStr, small enough
    let os_str = OsStr::new("test");
    let result: Result<InlineFlexStr<str>, TooLongOrUtf8Error> = os_str.try_into();
    let inline_str = result.unwrap();
    assert_eq!(inline_str.as_ref_type(), "test");

    // Too long
    let long_str = "x".repeat(inline_flexstr::INLINE_CAPACITY + 1);
    let os_str = OsStr::new(&long_str);
    let result: Result<InlineFlexStr<str>, TooLongOrUtf8Error> = os_str.try_into();
    result.unwrap_err();
}

/// Test TryFrom<&Path> for InlineFlexStr<str>
#[cfg(all(feature = "str", feature = "std"))]
pub fn test_try_from_path_str() {
    use inline_flexstr::TooLongOrUtf8Error;
    use std::path::Path;

    // Valid UTF-8 Path, small enough
    let path = Path::new("test");
    let result: Result<InlineFlexStr<str>, TooLongOrUtf8Error> = path.try_into();
    let inline_str = result.unwrap();
    assert_eq!(inline_str.as_ref_type(), "test");

    // Too long
    let long_str = "x".repeat(inline_flexstr::INLINE_CAPACITY + 1);
    let path = Path::new(&long_str);
    let result: Result<InlineFlexStr<str>, TooLongOrUtf8Error> = path.try_into();
    result.unwrap_err();
}
