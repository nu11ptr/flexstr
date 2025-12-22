#![allow(dead_code)]

use core::ffi::CStr;
use inline_flexstr::{InlineFlexStr, TooLongOrNulError};

/// Test InlineFlexStr::try_from_bytes_with_or_without_nul with valid CStr (with NUL)
pub fn test_try_from_bytes_with_nul() {
    let bytes = b"test\0";
    let inline_str = InlineFlexStr::<CStr>::try_from_bytes_with_or_without_nul(bytes).unwrap();
    assert_eq!(inline_str.as_ref_type(), c"test");
}

/// Test InlineFlexStr::try_from_bytes_with_or_without_nul with bytes without NUL
pub fn test_try_from_bytes_without_nul() {
    let bytes = b"test";
    let inline_str = InlineFlexStr::<CStr>::try_from_bytes_with_or_without_nul(bytes).unwrap();
    // Should have NUL appended
    assert_eq!(inline_str.as_bytes_with_nul(), b"test\0");
}

/// Test InlineFlexStr::try_from_bytes_with_or_without_nul with interior NUL
pub fn test_try_from_bytes_interior_nul() {
    let bytes = b"te\0st";
    let err = InlineFlexStr::<CStr>::try_from_bytes_with_or_without_nul(bytes).unwrap_err();
    match err {
        TooLongOrNulError::NulError(e) => assert_eq!(e.position, 2),
        _ => panic!("Expected NulError"),
    }
}

/// Test InlineFlexStr::try_from_bytes_with_or_without_nul with too long string
pub fn test_try_from_bytes_too_long() {
    let long_bytes = vec![b'x'; inline_flexstr::INLINE_CAPACITY];
    let err = InlineFlexStr::<CStr>::try_from_bytes_with_or_without_nul(&long_bytes).unwrap_err();
    match err {
        TooLongOrNulError::TooLong(e) => {
            assert_eq!(e.length, inline_flexstr::INLINE_CAPACITY);
            assert_eq!(e.inline_capacity, inline_flexstr::INLINE_CAPACITY);
        }
        _ => panic!("Expected TooLong error"),
    }
}

/// Test as_bytes_with_nul
pub fn test_as_bytes_with_nul(cstr: &'static CStr) {
    // Input should be small enough to inline
    let inline_str =
        InlineFlexStr::try_from_type(cstr).expect("test input should be small enough to inline");
    let bytes = inline_str.as_bytes_with_nul();
    assert_eq!(bytes, cstr.to_bytes_with_nul());
}
