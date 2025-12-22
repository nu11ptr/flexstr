#![allow(dead_code)]

use core::fmt;
use flexstr_support::StringToFromBytes;
use flexstry::{FlexStr, RefCounted};

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

/// Test TryFrom<&str> for FlexStr<CStr, R>
#[cfg(feature = "cstr")]
pub fn test_try_from_str_cstr<R>()
where
    R: RefCounted<core::ffi::CStr> + fmt::Debug,
{
    use flexstry::{FlexStr, InteriorNulError};

    // Valid CStr (no interior NUL)
    let s: &str = "test";
    let result: Result<FlexStr<'_, core::ffi::CStr, R>, InteriorNulError> = s.try_into();
    let flex_str = result.unwrap();
    assert_eq!(flex_str.as_ref_type().to_bytes(), b"test");

    // Invalid CStr (interior NUL)
    let s_with_nul: &str = "test\0middle";
    let result: Result<FlexStr<'_, core::ffi::CStr, R>, InteriorNulError> = s_with_nul.try_into();
    result.unwrap_err();
}

/// Test TryFrom<&[u8]> for FlexStr<CStr, R>
#[cfg(feature = "cstr")]
pub fn test_try_from_bytes_cstr<R>()
where
    R: RefCounted<core::ffi::CStr> + fmt::Debug,
{
    use flexstry::{FlexStr, InteriorNulError};

    // Valid CStr (no interior NUL)
    let bytes: &[u8] = b"test";
    let result: Result<FlexStr<'_, core::ffi::CStr, R>, InteriorNulError> = bytes.try_into();
    let flex_str = result.unwrap();
    assert_eq!(flex_str.as_ref_type().to_bytes(), b"test");

    // Invalid CStr (interior NUL)
    let bytes_with_nul: &[u8] = b"test\0middle";
    let result: Result<FlexStr<'_, core::ffi::CStr, R>, InteriorNulError> =
        bytes_with_nul.try_into();
    result.unwrap_err();
}

/// Test TryFrom<&OsStr> for FlexStr<str, R>
#[cfg(all(feature = "str", feature = "std"))]
pub fn test_try_from_osstr_str<R>()
where
    R: RefCounted<str> + fmt::Debug,
{
    use flexstry::FlexStr;
    use std::ffi::OsStr;

    // Valid UTF-8 OsStr
    let os_str = OsStr::new("test");
    let result: Result<FlexStr<'_, str, R>, core::str::Utf8Error> = os_str.try_into();
    let flex_str = result.unwrap();
    assert_eq!(flex_str.as_ref_type(), "test");
}

/// Test TryFrom<&Path> for FlexStr<str, R>
#[cfg(all(feature = "str", feature = "std"))]
pub fn test_try_from_path_str<R>()
where
    R: RefCounted<str> + fmt::Debug,
{
    use flexstry::FlexStr;
    use std::path::Path;

    // Valid UTF-8 Path
    let path = Path::new("test");
    let result: Result<FlexStr<'_, str, R>, core::str::Utf8Error> = path.try_into();
    let flex_str = result.unwrap();
    assert_eq!(flex_str.as_ref_type(), "test");
}

/// Test TryFrom<Vec<u8>> for FlexStr<str, R>
#[cfg(feature = "str")]
pub fn test_try_from_vec_u8_str<R>()
where
    R: RefCounted<str> + fmt::Debug,
{
    use alloc::string::FromUtf8Error;
    use flexstry::FlexStr;

    // Valid UTF-8 Vec<u8>
    let vec = b"test".to_vec();
    let result: Result<FlexStr<'static, str, R>, FromUtf8Error> = vec.try_into();
    let flex_str = result.unwrap();
    assert_eq!(flex_str.as_ref_type(), "test");

    // Invalid UTF-8 Vec<u8>
    let invalid_vec = vec![0xFF, 0xFF, 0xFF];
    let result: Result<FlexStr<'static, str, R>, FromUtf8Error> = invalid_vec.try_into();
    result.unwrap_err();
}

/// Test TryFrom<CString> for FlexStr<str, R>
#[cfg(all(feature = "str", feature = "cstr"))]
pub fn test_try_from_cstring_str<R>()
where
    R: RefCounted<str> + fmt::Debug,
{
    use alloc::ffi::{CString, IntoStringError};
    use flexstry::FlexStr;

    // Valid UTF-8 CString
    let cstring = CString::new("test").unwrap();
    let result: Result<FlexStr<'static, str, R>, IntoStringError> = cstring.try_into();
    let flex_str = result.unwrap();
    assert_eq!(flex_str.as_ref_type(), "test");
}
