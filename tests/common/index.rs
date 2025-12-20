#![allow(dead_code)]

use core::fmt;
use flexstry::{FlexStr, InlineFlexStr, RefCounted, StringToFromBytes};

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

/// Test Index implementation for InlineFlexStr
/// This tests that InlineFlexStr can be dereferenced to &S, which is required for Index
pub fn test_inline_index<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + PartialEq + fmt::Debug,
{
    // Input should be small enough to inline
    let inline_str =
        InlineFlexStr::try_from_type(s).expect("test input should be small enough to inline");

    // Test that we can deref to get &S (required for Index trait)
    // For inlined strings, the pointer will be different (data is copied), but values should be equal
    let original_ref: &S = s;
    let inline_ref: &S = &inline_str;

    // Verify the references are equal via PartialEq (not pointer equality)
    assert_eq!(original_ref, inline_ref);
}

/// Test IndexMut implementation for InlineFlexStr
/// This tests that InlineFlexStr can be dereferenced mutably to &mut S, which is required for IndexMut
pub fn test_inline_index_mut<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + flexstry::StringFromBytesMut + PartialEq + fmt::Debug,
{
    // Input should be small enough to inline
    let mut inline_str =
        InlineFlexStr::try_from_type(s).expect("test input should be small enough to inline");

    // Test that we can deref mutably to get &mut S (required for IndexMut trait)
    // The mutable reference should initially equal the original
    let original_ref: &S = s;
    let inline_ref: &mut S = &mut inline_str;

    // Verify the mutable reference equals the original via PartialEq
    assert_eq!(original_ref, inline_ref);
}
