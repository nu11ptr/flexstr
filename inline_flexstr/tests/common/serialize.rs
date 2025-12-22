#![allow(dead_code)]

use core::fmt;
use flexstr_support::StringToFromBytes;
use inline_flexstr::InlineFlexStr;
use serde::{Deserialize, Serialize};

pub fn serialize_deserialize_test<'s, F, S>(s: &'static S)
where
    F: TryFrom<&'s S> + PartialEq + Serialize + for<'de> Deserialize<'de> + fmt::Debug,
    <F as TryFrom<&'s S>>::Error: fmt::Debug,
    S: ?Sized + StringToFromBytes + Serialize + fmt::Debug + PartialEq,
    Box<S>: for<'de> Deserialize<'de>,
{
    let expected_str: F = s.try_into().unwrap();
    let serialized = serde_json::to_value(&expected_str).unwrap();
    let expected_json = serde_json::to_value(s).unwrap();
    assert_eq!(expected_json, serialized);

    let deserialized: F = serde_json::from_value(serialized).unwrap();
    assert_eq!(expected_str, deserialized);
}

/// Test deserialization error path when string is too long for inline storage
#[cfg(feature = "str")]
pub fn test_deserialize_error_str() {
    // Create a string that's definitely too long
    let long_string = "x".repeat(inline_flexstr::INLINE_CAPACITY + 1);
    let boxed: Box<str> = long_string.into_boxed_str();

    // Serialize it
    let serialized = serde_json::to_string(&boxed).unwrap();

    // Try to deserialize into InlineFlexStr - should fail
    let result: Result<InlineFlexStr<str>, _> = serde_json::from_str(&serialized);
    result.unwrap_err();
}
