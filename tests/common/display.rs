#![allow(dead_code)]

use core::fmt;
use flexstry::{FlexStr, InlineFlexStr, RefCounted, StringToFromBytes};

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

/// Test Display implementation for InlineFlexStr
pub fn test_inline_display<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Display + fmt::Debug,
{
    // Input should be small enough to inline
    let inline_str =
        InlineFlexStr::try_from_type(s).expect("test input should be small enough to inline");

    // Format both the original and InlineFlexStr
    let original_fmt = format!("{}", s);
    let inline_fmt = format!("{}", inline_str);

    assert_eq!(original_fmt, inline_fmt);
}
