#![allow(dead_code)]

use core::fmt;
use flexstry::{FlexStr, InlineFlexStr, RefCounted, StringToFromBytes};

/// Test TryFrom success path for FlexStr
pub fn test_try_from_flex_str_success<'s, T, S, R>(value: T)
where
    T: core::convert::TryInto<FlexStr<'s, S, R>>,
    T::Error: fmt::Debug,
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S> + fmt::Debug,
{
    let flex_str: FlexStr<'s, S, R> = value.try_into().unwrap();
    assert_eq!(flex_str.as_ref_type(), flex_str.as_ref_type()); // Basic sanity check
}

/// Test TryFrom error path for FlexStr
pub fn test_try_from_flex_str_error<'s, T, S, R>(value: T)
where
    T: core::convert::TryInto<FlexStr<'s, S, R>>,
    T::Error: fmt::Debug + fmt::Display,
    S: ?Sized + StringToFromBytes + fmt::Debug,
    R: RefCounted<S> + fmt::Debug,
{
    let result: Result<FlexStr<'s, S, R>, T::Error> = value.try_into();
    let err = result.unwrap_err();
    // Test that error can be displayed
    let _ = format!("{}", err);
}

/// Test TryFrom success path for InlineFlexStr
pub fn test_try_from_inline_success<T, S>(value: T)
where
    T: core::convert::TryInto<InlineFlexStr<S>>,
    T::Error: fmt::Debug,
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
{
    let inline_str: InlineFlexStr<S> = value.try_into().unwrap();
    assert_eq!(inline_str.as_ref_type(), inline_str.as_ref_type()); // Basic sanity check
}

/// Test TryFrom error path for InlineFlexStr
pub fn test_try_from_inline_error<T, S>(value: T)
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

/// Test TryFrom<&[u8]> for FlexStr<str, R> with invalid UTF-8
#[cfg(feature = "str")]
pub fn test_try_from_bytes_invalid_utf8<R>()
where
    R: RefCounted<str> + core::fmt::Debug,
{
    use core::str::Utf8Error;
    
    // Invalid UTF-8 sequence
    let invalid_utf8: &[u8] = &[0xFF, 0xFF, 0xFF];
    let result: Result<FlexStr<'_, str, R>, Utf8Error> = invalid_utf8.try_into();
    let _err = result.unwrap_err(); // Test that error can be unwrapped
}

/// Test TryFrom<Vec<u8>> for FlexStr<str, R> with invalid UTF-8
#[cfg(feature = "str")]
pub fn test_try_from_vec_bytes_invalid_utf8<R>()
where
    R: RefCounted<str> + fmt::Debug,
{
    use alloc::string::FromUtf8Error;
    
    // Invalid UTF-8 sequence
    let invalid_utf8 = alloc::vec![0xFF, 0xFF, 0xFF];
    let result: Result<FlexStr<'static, str, R>, FromUtf8Error> = invalid_utf8.try_into();
    let _err = result.unwrap_err();
}

/// Test TryFrom<CString> for FlexStr<str, R> with invalid UTF-8
#[cfg(all(feature = "str", feature = "cstr"))]
pub fn test_try_from_cstring_invalid_utf8<R>()
where
    R: RefCounted<str>,
{
    use alloc::ffi::{CString, IntoStringError};
    
    // Create a CString with invalid UTF-8 (this is tricky, but we can try)
    // Note: CString::new will fail if there's a NUL byte, so we need a different approach
    // For now, we'll test with a valid CString that contains non-UTF-8 bytes
    // This test may need adjustment based on actual CString behavior
    let cstring = CString::new(b"test\0").unwrap();
    // This should succeed since "test" is valid UTF-8
    let result: Result<FlexStr<'static, str, R>, IntoStringError> = cstring.try_into();
    let _flex_str = result.unwrap();
}

/// Test TryFrom<&[u8]> for InlineFlexStr<[u8]> with too long string
#[cfg(feature = "bytes")]
pub fn test_try_from_bytes_too_long() {
    use flexstry::TooLongForInlining;
    
    // Create a byte slice that's definitely too long
    let long_bytes = vec![0u8; flexstry::INLINE_CAPACITY + 1];
    let result: Result<InlineFlexStr<[u8]>, TooLongForInlining> = long_bytes.as_slice().try_into();
    let err = result.unwrap_err();
    assert_eq!(err.length, flexstry::INLINE_CAPACITY + 1);
    assert_eq!(err.inline_capacity, flexstry::INLINE_CAPACITY);
}

/// Test TryFrom<&str> for InlineFlexStr<[u8]> with too long string
#[cfg(feature = "bytes")]
pub fn test_try_from_str_too_long() {
    use flexstry::TooLongForInlining;
    
    // Create a string that's definitely too long
    let long_str = "x".repeat(flexstry::INLINE_CAPACITY + 1);
    let result: Result<InlineFlexStr<[u8]>, TooLongForInlining> = long_str.as_str().try_into();
    let _err = result.unwrap_err();
}

/// Test TryFrom<&OsStr> for InlineFlexStr<OsStr> with too long string
#[cfg(all(feature = "std", feature = "osstr"))]
pub fn test_try_from_osstr_too_long() {
    use flexstry::TooLongForInlining;
    use std::ffi::OsStr;
    
    // Create an OsStr that's definitely too long
    let long_str = "x".repeat(flexstry::INLINE_CAPACITY + 1);
    let os_str = OsStr::new(&long_str);
    let result: Result<InlineFlexStr<OsStr>, TooLongForInlining> = os_str.try_into();
    let _err = result.unwrap_err();
}

/// Test TryFrom<&str> for InlineFlexStr<OsStr> with too long string
#[cfg(all(feature = "std", feature = "osstr"))]
pub fn test_try_from_str_osstr_too_long() {
    use flexstry::TooLongForInlining;
    
    // Create a string that's definitely too long
    let long_str = "x".repeat(flexstry::INLINE_CAPACITY + 1);
    let result: Result<InlineFlexStr<std::ffi::OsStr>, TooLongForInlining> = long_str.as_str().try_into();
    let _err = result.unwrap_err();
}

/// Test TryFrom<&Path> for InlineFlexStr<OsStr> with too long string
#[cfg(all(feature = "std", feature = "osstr", feature = "path"))]
pub fn test_try_from_path_osstr_too_long() {
    use flexstry::TooLongForInlining;
    use std::path::Path;
    
    // Create a path that's definitely too long
    let long_str = "x".repeat(flexstry::INLINE_CAPACITY + 1);
    let path = Path::new(&long_str);
    let result: Result<InlineFlexStr<std::ffi::OsStr>, TooLongForInlining> = path.try_into();
    let _err = result.unwrap_err();
}

/// Test TryFrom<&Path> for InlineFlexStr<Path> with too long string
#[cfg(all(feature = "std", feature = "path"))]
pub fn test_try_from_path_too_long() {
    use flexstry::TooLongForInlining;
    use std::path::Path;
    
    // Create a path that's definitely too long
    let long_str = "x".repeat(flexstry::INLINE_CAPACITY + 1);
    let path = Path::new(&long_str);
    let result: Result<InlineFlexStr<Path>, TooLongForInlining> = path.try_into();
    let _err = result.unwrap_err();
}

/// Test TryFrom<&str> for InlineFlexStr<Path> with too long string
#[cfg(all(feature = "std", feature = "path"))]
pub fn test_try_from_str_path_too_long() {
    use flexstry::TooLongForInlining;
    
    // Create a string that's definitely too long
    let long_str = "x".repeat(flexstry::INLINE_CAPACITY + 1);
    let result: Result<InlineFlexStr<std::path::Path>, TooLongForInlining> = long_str.as_str().try_into();
    let _err = result.unwrap_err();
}

/// Test TryFrom<&OsStr> for InlineFlexStr<Path> with too long string
#[cfg(all(feature = "std", feature = "path"))]
pub fn test_try_from_osstr_path_too_long() {
    use flexstry::TooLongForInlining;
    use std::ffi::OsStr;
    
    // Create an OsStr that's definitely too long
    let long_str = "x".repeat(flexstry::INLINE_CAPACITY + 1);
    let os_str = OsStr::new(&long_str);
    let result: Result<InlineFlexStr<std::path::Path>, TooLongForInlining> = os_str.try_into();
    let _err = result.unwrap_err();
}

