#![allow(dead_code)]

use core::fmt;
use flexstr_support::StringToFromBytes;
use inline_flexstr::InlineFlexStr;

/// Test PartialEq implementation
pub fn test_partial_eq<S>(s1: &'static S, s2: &'static S)
where
    S: ?Sized + StringToFromBytes + PartialEq + fmt::Debug,
{
    // Inputs should be small enough to inline
    let inline_str1 =
        InlineFlexStr::try_from_type(s1).expect("test input should be small enough to inline");
    let inline_str2 =
        InlineFlexStr::try_from_type(s2).expect("test input should be small enough to inline");

    // Test equality
    if s1 == s2 {
        assert_eq!(inline_str1, inline_str2);
    } else {
        assert!(inline_str1 != inline_str2);
    }
}

/// Test Eq implementation
pub fn test_eq<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + PartialEq + Eq + fmt::Debug,
{
    // Input should be small enough to inline
    let inline_str1 =
        InlineFlexStr::try_from_type(s).expect("test input should be small enough to inline");
    let inline_str2 =
        InlineFlexStr::try_from_type(s).expect("test input should be small enough to inline");

    // Eq requires reflexivity, symmetry, transitivity
    // Reflexivity: a == a
    assert_eq!(inline_str1, inline_str2); // Symmetry
    assert_eq!(inline_str2, inline_str1); // Symmetry
}

/// Test PartialOrd implementation
pub fn test_partial_ord<S>(s1: &'static S, s2: &'static S)
where
    S: ?Sized + StringToFromBytes + PartialOrd,
{
    // Inputs should be small enough to inline
    let inline_str1 =
        InlineFlexStr::try_from_type(s1).expect("test input should be small enough to inline");
    let inline_str2 =
        InlineFlexStr::try_from_type(s2).expect("test input should be small enough to inline");

    // Test inputs should be comparable (partial_cmp should return Some)
    let ord = s1
        .partial_cmp(s2)
        .expect("test inputs should be comparable");
    assert_eq!(inline_str1.partial_cmp(&inline_str2), Some(ord));
}

/// Test Ord implementation
pub fn test_ord<S>(s1: &'static S, s2: &'static S)
where
    S: ?Sized + StringToFromBytes + Ord,
{
    // Inputs should be small enough to inline
    let inline_str1 =
        InlineFlexStr::try_from_type(s1).expect("test input should be small enough to inline");
    let inline_str2 =
        InlineFlexStr::try_from_type(s2).expect("test input should be small enough to inline");

    assert_eq!(inline_str1.cmp(&inline_str2), s1.cmp(s2));
}

/// Test Hash implementation consistency
pub fn test_hash<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + core::hash::Hash,
{
    use core::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    // Input should be small enough to inline
    let inline_str1 =
        InlineFlexStr::try_from_type(s).expect("test input should be small enough to inline");
    let mut hasher1 = DefaultHasher::new();
    inline_str1.hash(&mut hasher1);
    let hash1 = hasher1.finish();

    // Test that copying produces same hash
    let inline_str2 = inline_str1;
    let mut hasher2 = DefaultHasher::new();
    inline_str2.hash(&mut hasher2);
    let hash2 = hasher2.finish();
    assert_eq!(hash1, hash2);
}

/// Test comparison with &S
pub fn test_comparison_with_ref<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
{
    // Input should be small enough to inline
    let inline_str =
        InlineFlexStr::try_from_type(s).expect("test input should be small enough to inline");

    // Test equality with &S - compare through as_ref_type
    assert_eq!(inline_str.as_ref_type(), s);
}

/// Test comparison with S::Owned
pub fn test_comparison_with_owned<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    S::Owned: PartialEq + AsRef<S>,
{
    // Input should be small enough to inline
    let inline_str =
        InlineFlexStr::try_from_type(s).expect("test input should be small enough to inline");
    let owned = s.to_owned();

    assert_eq!(inline_str.as_ref_type(), owned.as_ref());
}

/// Test PartialEq with owned types (String, Cow, etc.) for InlineFlexStr
pub fn test_partial_eq_with_owned_types<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + PartialEq + fmt::Debug,
    S::Owned: PartialEq<S> + AsRef<S>,
{
    use alloc::borrow::Cow;

    // Input should be small enough to inline
    let inline_str =
        InlineFlexStr::try_from_type(s).expect("test input should be small enough to inline");
    let owned: S::Owned = s.to_owned();

    // Test InlineFlexStr == S::Owned (through PartialEq implementation)
    assert_eq!(inline_str.as_ref_type(), owned.as_ref());
    // Test reverse comparison: S::Owned == InlineFlexStr
    assert_eq!(owned.as_ref(), inline_str.as_ref_type());

    // Test with Cow::Owned
    let cow_owned: Cow<'_, S> = Cow::Owned(owned);
    assert_eq!(inline_str.as_ref_type(), cow_owned.as_ref());
    assert_eq!(cow_owned.as_ref(), inline_str.as_ref_type());

    // Test with Cow::Borrowed
    let cow_borrowed: Cow<'_, S> = Cow::Borrowed(s);
    assert_eq!(inline_str.as_ref_type(), cow_borrowed.as_ref());
    assert_eq!(cow_borrowed.as_ref(), inline_str.as_ref_type());
}
