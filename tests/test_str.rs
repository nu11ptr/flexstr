#![cfg(feature = "str")]

#[cfg(feature = "serde")]
use flexstry::{InlineFlexStr, LocalStr, SharedStr};

mod common;

#[cfg(feature = "serde")]
#[test]
fn serialize_deserialize_test_local_str() {
    common::serialize::serialize_deserialize_test::<LocalStr<'_>, str>("test");
}

#[cfg(feature = "serde")]
#[test]
fn serialize_deserialize_test_shared_str() {
    common::serialize::serialize_deserialize_test::<SharedStr<'_>, str>("test");
}

#[cfg(feature = "serde")]
#[test]
fn serialize_deserialize_test_inline_str() {
    common::serialize::serialize_deserialize_test::<InlineFlexStr<str>, str>("test");
}
