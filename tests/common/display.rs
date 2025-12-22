#![allow(dead_code)]

use core::fmt;
use flexstr::{FlexStr, RefCounted};
use flexstr_support::StringToFromBytes;

/// Test Display implementation for FlexStr
pub fn test_display<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Display + fmt::Debug,
    R: RefCounted<S>,
{
    let flex_str: FlexStr<'_, S, R> = FlexStr::from_borrowed(s);

    // Format both the original and FlexStr
    let original_fmt = format!("{}", s);
    let flex_fmt = format!("{}", flex_str);

    assert_eq!(original_fmt, flex_fmt);
}
