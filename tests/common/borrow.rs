#![allow(dead_code)]

use core::borrow::Borrow;
use flexstr::{FlexStr, RefCounted};
use flexstr_support::StringToFromBytes;

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
