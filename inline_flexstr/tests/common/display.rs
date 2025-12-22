#![allow(dead_code)]

use core::fmt;
use flexstr_support::StringToFromBytes;
use inline_flexstr::InlineFlexStr;

/// Test Display implementation for InlineFlexStr
pub fn test_display<S>(s: &'static S)
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

