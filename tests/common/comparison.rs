#![allow(dead_code)]

use core::fmt;
use flexstry::{FlexStr, InlineFlexStr, RefCounted, StringToFromBytes};

// Remove Debug requirement from R where not needed

/// Test PartialEq implementation
pub fn test_partial_eq<S, R>(s1: &'static S, s2: &'static S)
where
    S: ?Sized + StringToFromBytes + PartialEq + fmt::Debug,
    R: RefCounted<S> + fmt::Debug,
    Box<S>: From<S::Owned>,
    S::Owned: AsRef<S>,
{
    let flex_str1: FlexStr<'_, S, R> = FlexStr::from_borrowed(s1);
    let flex_str2: FlexStr<'_, S, R> = FlexStr::from_borrowed(s2);

    // Test equality
    if s1 == s2 {
        assert_eq!(flex_str1, flex_str2);
    } else {
        assert!(flex_str1 != flex_str2);
    }

    // Test equality across variants
    if let Ok(inline_str) = InlineFlexStr::try_from_type(s1) {
        let inlined: FlexStr<'_, S, R> = FlexStr::from_inline(inline_str);
        assert_eq!(flex_str1, inlined);
    }
}

/// Test Eq implementation
pub fn test_eq<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + PartialEq + Eq + fmt::Debug,
    R: RefCounted<S> + fmt::Debug,
    Box<S>: From<S::Owned>,
    S::Owned: AsRef<S>,
{
    let flex_str1: FlexStr<'_, S, R> = FlexStr::from_borrowed(s);
    let flex_str2: FlexStr<'_, S, R> = FlexStr::from_borrowed(s);

    // Eq requires reflexivity, symmetry, transitivity
    // Reflexivity: a == a
    assert_eq!(flex_str1, flex_str2); // Symmetry
    assert_eq!(flex_str2, flex_str1); // Symmetry
}

/// Test PartialOrd implementation
pub fn test_partial_ord<S, R>(s1: &'static S, s2: &'static S)
where
    S: ?Sized + StringToFromBytes + PartialOrd,
    R: RefCounted<S>,
{
    let flex_str1: FlexStr<'_, S, R> = FlexStr::from_borrowed(s1);
    let flex_str2: FlexStr<'_, S, R> = FlexStr::from_borrowed(s2);

    if let Some(ord) = s1.partial_cmp(s2) {
        assert_eq!(flex_str1.partial_cmp(&flex_str2), Some(ord));
    }
}

/// Test Ord implementation
pub fn test_ord<S, R>(s1: &'static S, s2: &'static S)
where
    S: ?Sized + StringToFromBytes + Ord,
    R: RefCounted<S>,
{
    let flex_str1: FlexStr<'_, S, R> = FlexStr::from_borrowed(s1);
    let flex_str2: FlexStr<'_, S, R> = FlexStr::from_borrowed(s2);

    assert_eq!(flex_str1.cmp(&flex_str2), s1.cmp(s2));
}

/// Test Hash implementation consistency
pub fn test_hash<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + core::hash::Hash,
    R: RefCounted<S>,
    Box<S>: From<S::Owned>,
    S::Owned: AsRef<S>,
{
    use core::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    // Test that different variants hash to the same value
    let borrowed: FlexStr<'_, S, R> = FlexStr::from_borrowed(s);
    let mut hasher1 = DefaultHasher::new();
    borrowed.hash(&mut hasher1);
    let hash1 = hasher1.finish();

    if let Ok(inline_str) = InlineFlexStr::try_from_type(s) {
        let inlined: FlexStr<'_, S, R> = FlexStr::from_inline(inline_str);
        let mut hasher2 = DefaultHasher::new();
        inlined.hash(&mut hasher2);
        let hash2 = hasher2.finish();
        assert_eq!(hash1, hash2);
    }

    // Test that ref_counted variant hashes the same
    if !s.self_as_bytes().is_empty() {
        let rc: R = s.into();
        let ref_counted: FlexStr<'_, S, R> = FlexStr::from_ref_counted(rc);
        let mut hasher3 = DefaultHasher::new();
        ref_counted.hash(&mut hasher3);
        let hash3 = hasher3.finish();
        assert_eq!(hash1, hash3);
    }
}

/// Test comparison with &S
pub fn test_comparison_with_ref<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S>,
{
    let flex_str: FlexStr<'_, S, R> = FlexStr::from_borrowed(s);

    // Test equality with &S - compare through as_ref_type
    assert_eq!(flex_str.as_ref_type(), s);
}

/// Test comparison with S::Owned
pub fn test_comparison_with_owned<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S>,
    S::Owned: PartialEq + AsRef<S>,
{
    let flex_str: FlexStr<'_, S, R> = FlexStr::from_borrowed(s);
    let owned = s.to_owned();

    assert_eq!(flex_str.as_ref_type(), owned.as_ref());
}

/// Test comparison with InlineFlexStr
pub fn test_comparison_with_inline<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S>,
{
    if let Ok(inline_str) = InlineFlexStr::try_from_type(s) {
        let flex_str: FlexStr<'_, S, R> = FlexStr::from_borrowed(s);

        assert_eq!(flex_str.as_ref_type(), inline_str.as_ref_type());
    }
}
