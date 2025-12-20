#![allow(dead_code)]

use core::borrow::Borrow;
use flexstry::{FlexStr, InlineFlexStr, RefCounted, StringFromBytesMut, StringToFromBytes};

/// Test Borrow implementation for FlexStr
pub fn test_borrow<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes,
    R: RefCounted<S>,
{
    let flex_str: FlexStr<'_, S, R> = FlexStr::from_borrowed(s);

    // Test Borrow::borrow() returns &S
    let borrowed: &S = flex_str.borrow();
    assert_eq!(borrowed as *const S, s as *const S);
}

/// Test BorrowMut implementation for InlineFlexStr
pub fn test_inline_borrow_mut<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + StringFromBytesMut,
{
    use core::borrow::BorrowMut;

    // Input should be small enough to inline
    let mut inline_str =
        InlineFlexStr::try_from_type(s).expect("test input should be small enough to inline");

    // Test BorrowMut::borrow_mut() returns &mut S
    let borrowed_mut: &mut S = inline_str.borrow_mut();
    assert_eq!(borrowed_mut as *const S, s as *const S);
}
