#![allow(dead_code)]

use core::borrow::BorrowMut;
use core::fmt;
use flexstr_support::{StringFromBytesMut, StringToFromBytes};
use inline_flexstr::InlineFlexStr;

/// Test BorrowMut implementation for InlineFlexStr
pub fn test_borrow_mut<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + StringFromBytesMut + PartialEq + fmt::Debug,
{
    // Input should be small enough to inline
    let mut inline_str =
        InlineFlexStr::try_from_type(s).expect("test input should be small enough to inline");

    // Test BorrowMut::borrow_mut() returns &mut S
    // For inlined strings, the pointer will be different (data is copied), but values should be equal
    let borrowed_mut: &mut S = inline_str.borrow_mut();
    assert_eq!(borrowed_mut, s);
}

