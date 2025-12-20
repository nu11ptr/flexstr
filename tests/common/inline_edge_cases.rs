#![allow(dead_code)]

use core::fmt;
use flexstry::{FlexStr, InlineFlexStr, RefCounted, StringToFromBytes};

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
pub fn test_try_from_type_too_long<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug,
{
    let bytes = S::self_as_raw_bytes(s);
    
    if bytes.len() > flexstry::INLINE_CAPACITY {
        let err = InlineFlexStr::try_from_type(s).unwrap_err();
        assert_eq!(err.length, bytes.len());
        assert_eq!(err.inline_capacity, flexstry::INLINE_CAPACITY);
    }
}

/// Test as_mut_type() for mutable string types
#[cfg(feature = "str")]
pub fn test_as_mut_type_str() {
    let mut inline_str = InlineFlexStr::<str>::try_from_type("test").unwrap();
    let mut_ref = inline_str.as_mut_type();
    // Test that we can mutate
    unsafe {
        let bytes = mut_ref.as_bytes_mut();
        if !bytes.is_empty() {
            bytes[0] = b'T';
        }
    }
    assert_eq!(inline_str.as_ref_type(), "Test");
}

/// Test as_mut_type() for [u8]
#[cfg(feature = "bytes")]
pub fn test_as_mut_type_bytes() {
    let mut inline_str = InlineFlexStr::<[u8]>::try_from_type(b"test").unwrap();
    let mut_ref = inline_str.as_mut_type();
    // Test that we can mutate
    let bytes = mut_ref;
    if !bytes.is_empty() {
        bytes[0] = b'T';
    }
    assert_eq!(inline_str.as_ref_type(), b"Test");
}

/// Test optimize() path where RefCounted is converted to Inlined
pub fn test_optimize_ref_counted_to_inlined<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S>,
{
    let bytes = S::self_as_raw_bytes(s);
    
    // Only test if the string is small enough to inline
    if bytes.len() <= flexstry::INLINE_CAPACITY {
        let rc: R = s.into();
        let ref_counted: FlexStr<'_, S, R> = FlexStr::from_ref_counted(rc);
        let optimized = ref_counted.optimize();
        
        // Should be inlined after optimization
        assert!(optimized.is_inlined());
        assert_eq!(optimized.as_ref_type(), s);
    }
}

/// Test optimize() path where RefCounted stays RefCounted (too large)
pub fn test_optimize_ref_counted_stays_ref_counted<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S>,
{
    let bytes = S::self_as_raw_bytes(s);
    
    // Only test if the string is too large to inline
    if bytes.len() > flexstry::INLINE_CAPACITY {
        let rc: R = s.into();
        let ref_counted: FlexStr<'_, S, R> = FlexStr::from_ref_counted(rc.clone());
        let optimized = ref_counted.optimize();
        
        // Should stay as ref_counted
        assert!(optimized.is_ref_counted());
        assert_eq!(optimized.as_ref_type(), s);
    }
}

