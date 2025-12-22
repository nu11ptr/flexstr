#![allow(dead_code)]

use core::fmt;
use flexstr_support::{StringLike, StringToFromBytes};
use inline_flexstr::InlineFlexStr;

/// Test to_owned_type conversion
pub fn test_to_owned_type<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    S::Owned: PartialEq + AsRef<S>,
    InlineFlexStr<S>: StringLike<S>,
{
    // Input should be small enough to inline
    let inline_str =
        InlineFlexStr::try_from_type(s).expect("test input should be small enough to inline");
    let owned = StringLike::to_owned_type(&inline_str);
    assert_eq!(owned.as_ref(), s);
}

/// Test into_owned_type conversion
pub fn test_into_owned_type<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + PartialEq + fmt::Debug,
    S::Owned: PartialEq + AsRef<S> + From<Box<S>>,
    InlineFlexStr<S>: StringLike<S>,
{
    // Input should be small enough to inline
    let inline_str =
        InlineFlexStr::try_from_type(s).expect("test input should be small enough to inline");

    let owned = StringLike::into_owned_type(inline_str);
    assert_eq!(owned.as_ref(), s);
}

