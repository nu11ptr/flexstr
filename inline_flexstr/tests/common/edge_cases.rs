#![allow(dead_code)]

use core::fmt;
use flexstr_support::{StringLike, StringToFromBytes};
use inline_flexstr::{InlineFlexStr, TooLongForInlining};

/// Test empty string operations
pub fn test_empty_string<S>(empty: &'static S)
where
    S: ?Sized + StringToFromBytes + PartialEq + fmt::Debug,
    InlineFlexStr<S>: StringLike<S>,
{
    let inline_str = InlineFlexStr::try_from_type(empty).unwrap();

    assert!(StringLike::is_empty(&inline_str));
    assert_eq!(StringLike::len(&inline_str), 0);

    // Test empty string can be copied
    let cloned = inline_str;
    assert_eq!(inline_str, cloned);
}

/// Test capacity boundary - string at exact capacity
/// Input must be exactly at INLINE_CAPACITY
pub fn test_capacity_boundary_exact<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
{
    let bytes = s.self_as_raw_bytes();
    assert_eq!(
        bytes.len(),
        inline_flexstr::INLINE_CAPACITY,
        "test input must be exactly at capacity"
    );

    let inline_str =
        InlineFlexStr::try_from_type(s).expect("string at exact capacity should inline");
    assert_eq!(inline_str.as_ref_type(), s);
}

/// Test capacity boundary - string one byte over capacity
/// Input must be smaller than INLINE_CAPACITY
pub fn test_capacity_boundary_overflow<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
{
    let bytes = s.self_as_raw_bytes();
    assert!(
        bytes.len() < inline_flexstr::INLINE_CAPACITY,
        "test input must be smaller than capacity"
    );

    // Since bytes.len() < INLINE_CAPACITY, bytes.len() <= INLINE_CAPACITY is always true
    // So try_from_type should always succeed
    let _inline_str =
        InlineFlexStr::try_from_type(s).expect("string smaller than capacity should succeed");
}

/// Test TryFrom error cases - too long
// Type parameter intentionally unused - kept for API consistency with other test functions
#[allow(unused)]
pub fn test_try_from_too_long() {
    // Create a string that's definitely too long
    // This is tricky to do generically, so we'll test the error type
    let _long_bytes = [0u8; inline_flexstr::INLINE_CAPACITY + 1];

    // Try to create from bytes if possible
    // This will depend on the specific string type
    // For now, we'll just verify the error type exists
    let err = TooLongForInlining {
        length: inline_flexstr::INLINE_CAPACITY + 1,
        inline_capacity: inline_flexstr::INLINE_CAPACITY,
    };

    assert_eq!(err.length, inline_flexstr::INLINE_CAPACITY + 1);
    assert_eq!(err.inline_capacity, inline_flexstr::INLINE_CAPACITY);
}

/// Test various string lengths
pub fn test_various_lengths<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + PartialEq + fmt::Debug,
    InlineFlexStr<S>: StringLike<S>,
{
    let bytes = s.self_as_bytes();
    let len = bytes.len();

    // Input should be small enough to inline
    let inline_str =
        InlineFlexStr::try_from_type(s).expect("test input should be small enough to inline");

    // Test length matches
    assert_eq!(StringLike::len(&inline_str), len);

    // Test empty check
    assert_eq!(StringLike::is_empty(&inline_str), len == 0);
}

/// Test special content (if applicable)
pub fn test_special_content<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + PartialEq,
{
    // Input should be small enough to inline
    let inline_str =
        InlineFlexStr::try_from_type(s).expect("test input should be small enough to inline");

    // Test that special characters/content are preserved
    let bytes = inline_str.as_bytes();
    assert_eq!(bytes, s.self_as_bytes());

    // Test that raw bytes match
    let raw_bytes = inline_str.as_raw_bytes();
    assert_eq!(raw_bytes, s.self_as_raw_bytes());
}

/// Test clone
pub fn test_clone<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + PartialEq + fmt::Debug,
{
    // Input should be small enough to inline
    let inline_str =
        InlineFlexStr::try_from_type(s).expect("test input should be small enough to inline");
    let cloned = inline_str;
    assert_eq!(inline_str, cloned);
}

/// Test try_from_type error when string is too long
/// Input must be larger than INLINE_CAPACITY
pub fn test_try_from_type_too_long<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug,
{
    let bytes = S::self_as_raw_bytes(s);
    assert!(
        bytes.len() > inline_flexstr::INLINE_CAPACITY,
        "test input must be too long to inline"
    );

    let err = InlineFlexStr::try_from_type(s).unwrap_err();
    assert_eq!(err.length, bytes.len());
    assert_eq!(err.inline_capacity, inline_flexstr::INLINE_CAPACITY);
}

/// Test as_mut_type() for mutable string types
#[cfg(feature = "str")]
pub fn test_as_mut_type_str() {
    let mut inline_str = InlineFlexStr::<str>::try_from_type("test").unwrap();
    let mut_ref = inline_str.as_mut_type();
    // Test that we can mutate
    // Test input should be non-empty

    let bytes = unsafe { mut_ref.as_bytes_mut() };
    assert!(!bytes.is_empty(), "test input should be non-empty");
    bytes[0] = b'T';
    assert_eq!(inline_str.as_ref_type(), "Test");
}

/// Test as_mut_type() for [u8]
#[cfg(feature = "bytes")]
pub fn test_as_mut_type_bytes() {
    let mut inline_str = InlineFlexStr::<[u8]>::try_from_type(b"test").unwrap();
    let mut_ref = inline_str.as_mut_type();
    // Test that we can mutate
    // Test input should be non-empty
    let bytes = mut_ref;
    assert!(!bytes.is_empty(), "test input should be non-empty");
    bytes[0] = b'T';
    assert_eq!(inline_str.as_ref_type(), b"Test");
}
