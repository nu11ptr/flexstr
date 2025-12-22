#![allow(dead_code)]

use core::fmt;
use inline_flexstr::TooLongForInlining;

#[cfg(feature = "str")]
use inline_flexstr::TooLongOrUtf8Error;

#[cfg(feature = "cstr")]
use flexstr::InteriorNulError;
#[cfg(feature = "cstr")]
use inline_flexstr::TooLongOrNulError;

/// Test Display implementation for error types
pub fn test_error_display<E>(error: E)
where
    E: fmt::Display + fmt::Debug,
{
    let display_str = format!("{}", error);
    let debug_str = format!("{:?}", error);

    // Display should produce a non-empty string
    assert!(!display_str.is_empty());
    // Debug should also produce a non-empty string
    assert!(!debug_str.is_empty());
}

/// Test TooLongForInlining error
pub fn test_too_long_for_inlining() {
    let err = TooLongForInlining {
        length: 100,
        inline_capacity: 30,
    };

    // Test that the error message contains relevant information
    let msg = format!("{}", err);
    assert!(msg.contains("too long"));
    assert!(msg.contains("100"));
    assert!(msg.contains("30"));

    test_error_display(err);
}

/// Test TooLongOrUtf8Error::TooLong variant
#[cfg(feature = "str")]
pub fn test_too_long_or_utf8_error_too_long() {
    let err = TooLongOrUtf8Error::TooLong(TooLongForInlining {
        length: 100,
        inline_capacity: 30,
    });

    test_error_display(err);
}

/// Test TooLongOrUtf8Error::Utf8Error variant
#[cfg(feature = "str")]
pub fn test_too_long_or_utf8_error_utf8() {
    // Create an invalid UTF-8 sequence
    let invalid_utf8: &[u8] =
        unsafe { core::slice::from_raw_parts([0xFFu8, 0xFFu8, 0xFFu8].as_ptr(), 3) };
    let utf8_err = str::from_utf8(invalid_utf8).unwrap_err();

    let err = TooLongOrUtf8Error::Utf8Error(utf8_err);
    test_error_display(err);
}

/// Test InteriorNulError
#[cfg(feature = "cstr")]
pub fn test_interior_nul_error() {
    let err = InteriorNulError { position: 5 };

    let msg = format!("{}", err);
    assert!(msg.contains("Interior NUL"));
    assert!(msg.contains("5"));

    test_error_display(err);
}

/// Test TooLongOrNulError::TooLong variant
#[cfg(feature = "cstr")]
pub fn test_too_long_or_nul_error_too_long() {
    let err = TooLongOrNulError::TooLong(TooLongForInlining {
        length: 100,
        inline_capacity: 30,
    });

    test_error_display(err);
}

/// Test TooLongOrNulError::NulError variant
#[cfg(feature = "cstr")]
pub fn test_too_long_or_nul_error_nul() {
    let err = TooLongOrNulError::NulError(InteriorNulError { position: 3 });

    test_error_display(err);
}
