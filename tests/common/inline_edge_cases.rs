#![allow(dead_code)]

use core::fmt;
use flexstr::{FlexStr, RefCounted};
use flexstr_support::StringToFromBytes;
use inline_flexstr::INLINE_CAPACITY;

/// Test optimize() path where RefCounted is converted to Inlined
/// Input must be small enough to inline (<= INLINE_CAPACITY)
pub fn test_optimize_ref_counted_to_inlined<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S>,
{
    let bytes = S::self_as_raw_bytes(s);
    assert!(
        bytes.len() <= INLINE_CAPACITY,
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
        bytes.len() > INLINE_CAPACITY,
        "test input must be too large to inline"
    );

    let rc: R = s.into();
    let ref_counted: FlexStr<'_, S, R> = FlexStr::from_ref_counted(rc.clone());
    let optimized = ref_counted.optimize();

    // Should stay as ref_counted
    assert!(optimized.is_ref_counted());
    assert_eq!(optimized.as_ref_type(), s);
}
