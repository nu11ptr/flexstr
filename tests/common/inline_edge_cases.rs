#![allow(dead_code)]

use core::fmt;
use flexstry::{FlexStr, InlineFlexStr, RefCounted, StringFromBytesMut, StringToFromBytes};

/// Test Default implementation for InlineFlexStr
pub fn test_inline_default<S>()
where
    S: ?Sized + StringToFromBytes,
    for<'a> &'a S: Default,
{
    let default_str = InlineFlexStr::<S>::default();
    assert_eq!(default_str.as_bytes().len(), 0);
}

/// Test try_from_type error when string is too long
/// Input must be larger than INLINE_CAPACITY
pub fn test_try_from_type_too_long<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug,
{
    let bytes = S::self_as_raw_bytes(s);
    assert!(
        bytes.len() > flexstry::INLINE_CAPACITY,
        "test input must be too long to inline"
    );

    let err = InlineFlexStr::try_from_type(s).unwrap_err();
    assert_eq!(err.length, bytes.len());
    assert_eq!(err.inline_capacity, flexstry::INLINE_CAPACITY);
}

/// Test as_mut_type() for mutable string types
#[cfg(feature = "str")]
pub fn test_as_mut_type_str() {
    let mut inline_str = InlineFlexStr::<str>::try_from_type("test").unwrap();
    let mut_ref = inline_str.as_mut_type();
    // Test that we can mutate
    // Test input should be non-empty
    unsafe {
        let bytes = mut_ref.as_bytes_mut();
        assert!(!bytes.is_empty(), "test input should be non-empty");
        bytes[0] = b'T';
    }
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

/// Test optimize() path where RefCounted is converted to Inlined
/// Input must be small enough to inline (<= INLINE_CAPACITY)
pub fn test_optimize_ref_counted_to_inlined<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S>,
{
    let bytes = S::self_as_raw_bytes(s);
    assert!(
        bytes.len() <= flexstry::INLINE_CAPACITY,
        "test input must be small enough to inline"
    );

    let rc: R = s.into();
    let ref_counted: FlexStr<'_, S, R> = FlexStr::from_ref_counted(rc);
    let optimized = ref_counted.optimize();

    // Should be inlined after optimization
    assert!(optimized.is_inlined());
    assert_eq!(optimized.as_ref_type(), s);
}

/// Test optimize() path where RefCounted stays RefCounted (too large)
/// Input must be too large to inline (> INLINE_CAPACITY)
pub fn test_optimize_ref_counted_stays_ref_counted<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S>,
{
    let bytes = S::self_as_raw_bytes(s);
    assert!(
        bytes.len() > flexstry::INLINE_CAPACITY,
        "test input must be too large to inline"
    );

    let rc: R = s.into();
    let ref_counted: FlexStr<'_, S, R> = FlexStr::from_ref_counted(rc.clone());
    let optimized = ref_counted.optimize();

    // Should stay as ref_counted
    assert!(optimized.is_ref_counted());
    assert_eq!(optimized.as_ref_type(), s);
}

/// Test BorrowMut implementation for InlineFlexStr
pub fn test_inline_borrow_mut<S>(s: &'static S)
where
    S: ?Sized + StringFromBytesMut,
{
    use core::borrow::BorrowMut;

    // Input should be small enough to inline
    let mut inline_str =
        InlineFlexStr::try_from_type(s).expect("test input should be small enough to inline");

    // Test BorrowMut::borrow_mut() returns &mut S
    let borrowed_mut: &mut S = inline_str.borrow_mut();
    assert_eq!(borrowed_mut as *const S, s as *const S);
}
