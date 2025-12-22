#![allow(dead_code)]

use core::ffi::CStr;
use flexstry::{FlexStr, RefCounted};

/// Test try_from_bytes_with_or_without_nul with valid CStr (with NUL)
pub fn test_try_from_bytes_with_nul<R>()
where
    R: RefCounted<CStr>,
{
    let bytes = b"test\0";
    let flex_str = FlexStr::<'_, CStr, R>::try_from_bytes_with_or_without_nul(bytes).unwrap();
    assert_eq!(flex_str.as_ref_type(), c"test");
}

/// Test try_from_bytes_with_or_without_nul with bytes without NUL
pub fn test_try_from_bytes_without_nul<R>()
where
    R: RefCounted<CStr>,
{
    let bytes = b"test";
    let flex_str = FlexStr::<'_, CStr, R>::try_from_bytes_with_or_without_nul(bytes).unwrap();
    // Should have NUL appended
    assert_eq!(flex_str.as_bytes_with_nul(), b"test\0");
}

/// Test try_from_bytes_with_or_without_nul with interior NUL
pub fn test_try_from_bytes_interior_nul<R>()
where
    R: RefCounted<CStr> + core::fmt::Debug,
{
    let bytes = b"te\0st";
    let err = FlexStr::<'_, CStr, R>::try_from_bytes_with_or_without_nul(bytes).unwrap_err();
    assert_eq!(err.position, 2);
}

/// Test as_bytes_with_nul
pub fn test_as_bytes_with_nul<R>(cstr: &'static CStr)
where
    R: RefCounted<CStr>,
{
    let flex_str: FlexStr<'_, CStr, R> = FlexStr::from_borrowed(cstr);
    let bytes = flex_str.as_bytes_with_nul();
    assert_eq!(bytes, cstr.to_bytes_with_nul());
}
