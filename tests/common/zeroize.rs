#![allow(dead_code)]

use core::fmt;
use flexstr::{FlexStr, RefCountedMut, StringLike};
use flexstr_support::StringToFromBytes;
use inline_flexstr::InlineFlexStr;
use zeroize::{TryZeroize, Zeroize};

/// Test try_zeroize on an Inlined variant — should succeed
pub fn test_zeroize_inlined<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCountedMut<S>,
{
    let inline = InlineFlexStr::try_from_type(s).expect("test input should be small enough");
    let mut flex: FlexStr<'_, S, R> = FlexStr::from_inline(inline);
    assert!(flex.is_inlined());

    assert!(flex.try_zeroize());

    assert!(flex.is_inlined());
    assert!(StringLike::is_empty(&flex));
}

/// Test try_zeroize on a Borrowed variant — should fail (we don't own the data)
pub fn test_zeroize_borrowed<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCountedMut<S>,
{
    let mut flex: FlexStr<'_, S, R> = FlexStr::from_borrowed(s);
    assert!(flex.is_borrowed());

    assert!(!flex.try_zeroize());

    // Value should be unchanged
    assert!(flex.is_borrowed());
    assert_eq!(flex.as_ref_type(), s);
}

/// Test try_zeroize on a unique RefCounted variant (last reference) — should succeed
pub fn test_zeroize_ref_counted<S, R>(rc: R)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCountedMut<S>,
{
    let mut flex: FlexStr<'_, S, R> = FlexStr::from_ref_counted(rc);
    assert!(flex.is_ref_counted());

    assert!(flex.try_zeroize());

    assert!(flex.is_inlined());
    assert!(StringLike::is_empty(&flex));
}

/// Test try_zeroize on a Boxed variant — should succeed
pub fn test_zeroize_boxed<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCountedMut<S>,
    Box<S>: From<S::Owned>,
    S::Owned: AsRef<S>,
{
    let boxed = Box::from(s.to_owned());
    let mut flex: FlexStr<'_, S, R> = FlexStr::from_boxed(boxed);
    assert!(flex.is_boxed());

    assert!(flex.try_zeroize());

    assert!(flex.is_inlined());
    assert!(StringLike::is_empty(&flex));
}

/// Test try_zeroize on a shared RefCounted variant — should fail (other references exist)
pub fn test_zeroize_ref_counted_shared<S, R>(rc: R)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCountedMut<S> + Clone,
{
    let mut flex: FlexStr<'_, S, R> = FlexStr::from_ref_counted(rc);
    assert!(flex.is_ref_counted());

    // Create a second reference so try_zeroize cannot get exclusive access
    let flex2 = flex.clone();
    assert!(flex2.is_ref_counted());

    assert!(!flex.try_zeroize());

    // Value should be unchanged — still ref counted and pointing to the same data
    assert!(flex.is_ref_counted());
    assert_eq!(flex.as_ref_type(), flex2.as_ref_type());
}

/// Test that InlineFlexStr raw bytes are actually zeroed after zeroize
pub fn test_zeroize_inline_bytes_cleared<S>(s: &S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
{
    let mut inline = InlineFlexStr::try_from_type(s).expect("test input should be small enough");

    // Verify we have non-zero data
    assert!(
        !S::self_as_raw_bytes(s).is_empty(),
        "test input must be non-empty"
    );

    // Get a raw pointer to the struct's memory (struct is on the stack, still alive after zeroize)
    let ptr = &inline as *const InlineFlexStr<S> as *const u8;
    let size = core::mem::size_of::<InlineFlexStr<S>>();

    inline.zeroize();

    // Read the raw bytes of the struct — it's still alive on the stack
    let bytes = unsafe { core::slice::from_raw_parts(ptr, size) };
    assert!(
        bytes.iter().all(|&b| b == 0),
        "all bytes of InlineFlexStr should be zero after zeroize, got: {bytes:?}"
    );
}
