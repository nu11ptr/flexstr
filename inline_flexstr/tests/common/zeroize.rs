#![allow(dead_code)]

use core::fmt;
use flexstr_support::StringToFromBytes;
use inline_flexstr::InlineFlexStr;
use zeroize::Zeroize;

/// Test that InlineFlexStr has zero-length raw bytes after zeroize
pub fn test_zeroize_empty_after<S>(s: &S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
{
    let mut inline = InlineFlexStr::try_from_type(s).expect("test input should be small enough");
    assert!(
        !inline.as_raw_bytes().is_empty(),
        "test input must be non-empty"
    );

    inline.zeroize();

    // Check raw bytes directly — InlineFlexStr::zeroize() zeros everything including len,
    // which produces a maximally secure but potentially invalid-for-interpretation state
    // (e.g., CStr requires a NUL terminator). FlexStr::zeroize() handles this by replacing
    // with InlineFlexStr::zeroed() which produces a valid empty value.
    assert!(inline.as_raw_bytes().is_empty());
}

/// Test that all raw bytes of InlineFlexStr are zeroed after zeroize
pub fn test_zeroize_bytes_cleared<S>(s: &S)
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
