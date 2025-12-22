#![allow(dead_code)]

use core::fmt;
use flexstr_support::{StringLike, StringToFromBytes};
use inline_flexstr::InlineFlexStr;

/// Test creation from inline string
pub fn test_creation_from_inline<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
{
    let inline_str = InlineFlexStr::try_from_type(s).unwrap();
    assert_eq!(inline_str.as_ref_type(), s);
}

/// Test empty string creation
pub fn test_empty<S>(empty: &'static S)
where
    S: ?Sized + StringToFromBytes,
    InlineFlexStr<S>: StringLike<S>,
{
    let inline_str = InlineFlexStr::try_from_type(empty).unwrap();
    assert!(StringLike::is_empty(&inline_str));
    assert_eq!(StringLike::len(&inline_str), 0);
}

/// Test accessor methods
pub fn test_accessors<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    InlineFlexStr<S>: StringLike<S>,
{
    let inline_str = InlineFlexStr::try_from_type(s).unwrap();

    // Test as_ref_type
    assert_eq!(inline_str.as_ref_type(), s);

    // Test as_bytes
    let bytes = inline_str.as_bytes();
    assert_eq!(bytes, S::self_as_bytes(s));

    // Test as_raw_bytes
    let raw_bytes = inline_str.as_raw_bytes();
    assert_eq!(raw_bytes, S::self_as_raw_bytes(s));

    // Test len
    assert_eq!(StringLike::len(&inline_str), s.self_as_bytes().len());

    // Test is_empty
    assert_eq!(
        StringLike::is_empty(&inline_str),
        s.self_as_bytes().is_empty()
    );
}

/// Test cloning
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

/// Test Default implementation
/// Note: This test is only applicable for types where `&S: Default`
pub fn test_default<S>()
where
    S: ?Sized + StringToFromBytes,
    for<'a> &'a S: Default,
    InlineFlexStr<S>: StringLike<S>,
{
    let inline_str = InlineFlexStr::default();
    assert!(StringLike::is_empty(&inline_str));
}
