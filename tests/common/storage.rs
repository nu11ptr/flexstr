#![allow(dead_code)]

use core::fmt;
use flexstry::{FlexStr, InlineFlexStr, RefCounted, StringToFromBytes};

/// Test variant query methods
pub fn test_variant_queries<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S>,
    Box<S>: From<S::Owned>,
    S::Owned: AsRef<S>,
{
    // Test borrowed variant
    let borrowed: FlexStr<'_, S, R> = FlexStr::from_borrowed(s);
    assert!(borrowed.is_borrowed());
    assert!(!borrowed.is_inlined());
    assert!(!borrowed.is_ref_counted());
    assert!(!borrowed.is_boxed());
    assert!(borrowed.is_off_heap());
    assert!(!borrowed.is_on_heap());

    // Test inlined variant
    if let Ok(inline_str) = InlineFlexStr::try_from_type(s) {
        let inlined: FlexStr<'_, S, R> = FlexStr::from_inline(inline_str);
        assert!(!inlined.is_borrowed());
        assert!(inlined.is_inlined());
        assert!(!inlined.is_ref_counted());
        assert!(!inlined.is_boxed());
        assert!(inlined.is_off_heap());
        assert!(!inlined.is_on_heap());
    }

    // Test ref_counted variant
    let rc: R = s.into();
    let ref_counted: FlexStr<'_, S, R> = FlexStr::from_ref_counted(rc);
    assert!(!ref_counted.is_borrowed());
    assert!(!ref_counted.is_inlined());
    assert!(ref_counted.is_ref_counted());
    assert!(!ref_counted.is_boxed());
    assert!(!ref_counted.is_off_heap());
    assert!(ref_counted.is_on_heap());

    // Test boxed variant
    let boxed: FlexStr<'_, S, R> = FlexStr::from_boxed(Box::from(s.to_owned()));
    assert!(!boxed.is_borrowed());
    assert!(!boxed.is_inlined());
    assert!(!boxed.is_ref_counted());
    assert!(boxed.is_boxed());
    assert!(!boxed.is_off_heap());
    assert!(boxed.is_on_heap());
}

/// Test variant transitions: borrowed -> inlined
pub fn test_transition_borrowed_to_inlined<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S>,
{
    let borrowed: FlexStr<'_, S, R> = FlexStr::from_borrowed(s);
    let owned = borrowed.to_owned();

    // If small enough, should be inlined
    if s.self_as_raw_bytes().len() <= flexstry::INLINE_CAPACITY {
        assert!(owned.is_inlined() || owned.is_borrowed());
    }
    assert_eq!(owned.as_ref_type(), s);
}

/// Test variant transitions: borrowed -> ref_counted
pub fn test_transition_borrowed_to_ref_counted<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S>,
{
    // For strings that are already long enough, test ref_counted transition
    if s.self_as_raw_bytes().len() > flexstry::INLINE_CAPACITY {
        let borrowed: FlexStr<'_, S, R> = FlexStr::from_borrowed(s);
        let owned = borrowed.to_owned();
        assert!(owned.is_ref_counted());
        assert_eq!(owned.as_ref_type(), s);
    }
}

/// Test variant transitions: inlined -> ref_counted
pub fn test_transition_inlined_to_ref_counted<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S>,
{
    if let Ok(inline_str) = InlineFlexStr::try_from_type(s) {
        let inlined: FlexStr<'_, S, R> = FlexStr::from_inline(inline_str);
        let cloned = inlined.clone();

        // Cloning inlined should still be inlined
        assert!(cloned.is_inlined());
        assert_eq!(cloned.as_ref_type(), s);
    }
}

/// Test variant transitions: boxed -> inlined/ref_counted
pub fn test_transition_boxed_to_optimized<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S>,
    Box<S>: From<S::Owned>,
    S::Owned: AsRef<S>,
{
    let boxed: FlexStr<'_, S, R> = FlexStr::from_boxed(Box::from(s.to_owned()));
    let optimized = boxed.optimize();

    // Should be inlined or ref_counted, not boxed
    assert!(matches!(
        optimized,
        FlexStr::Inlined(_) | FlexStr::RefCounted(_)
    ));
    assert_eq!(optimized.as_ref_type(), s);
}

/// Test storage optimization
pub fn test_storage_optimization<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S>,
    Box<S>: From<S::Owned>,
    S::Owned: AsRef<S>,
{
    // Test optimize on boxed (should convert to inlined or ref_counted)
    let boxed: FlexStr<'_, S, R> = FlexStr::from_boxed(Box::from(s.to_owned()));
    let optimized = boxed.optimize();
    assert!(matches!(
        optimized,
        FlexStr::Inlined(_) | FlexStr::RefCounted(_)
    ));
    assert_eq!(optimized.as_ref_type(), s);

    // Test optimize on borrowed (should stay borrowed)
    let borrowed: FlexStr<'_, S, R> = FlexStr::from_borrowed(s);
    let optimized = borrowed.optimize();
    assert!(optimized.is_borrowed());
    assert_eq!(optimized.as_ref_type(), s);

    // Test optimize on inlined (should stay inlined)
    if let Ok(inline_str) = InlineFlexStr::try_from_type(s) {
        let inlined: FlexStr<'_, S, R> = FlexStr::from_inline(inline_str);
        let optimized = inlined.optimize();
        assert!(optimized.is_inlined());
        assert_eq!(optimized.as_ref_type(), s);
    }
}
