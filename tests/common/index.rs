#![allow(dead_code)]

use core::fmt;
use flexstr::{FlexStr, RefCounted};
use flexstr_support::StringToFromBytes;

/// Test Index implementation for FlexStr
/// This tests that FlexStr can be dereferenced to &S, which is required for Index
pub fn test_index<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + PartialEq + fmt::Debug,
    R: RefCounted<S>,
{
    let flex_str: FlexStr<'_, S, R> = FlexStr::from_borrowed(s);

    // Test that we can deref to get &S (required for Index trait)
    // The dereferenced values should be equal, not necessarily the same pointer
    let original_ref: &S = s;
    let flex_ref: &S = &flex_str;

    // Verify the references are equal via PartialEq (not pointer equality)
    assert_eq!(original_ref, flex_ref);
}
