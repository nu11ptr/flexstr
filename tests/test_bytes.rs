#![cfg(feature = "bytes")]

#[cfg(feature = "serde")]
use flexstry::{InlineStr, LocalBytes, SharedBytes};

mod common;

#[cfg(feature = "serde")]
#[test]
fn serialize_deserialize_test_local_bytes() {
    common::serialize::serialize_deserialize_test::<LocalBytes<'_>, [u8]>(b"test");
}

#[cfg(feature = "serde")]
#[test]
fn serialize_deserialize_test_shared_bytes() {
    common::serialize::serialize_deserialize_test::<SharedBytes<'_>, [u8]>(b"test");
}

#[cfg(feature = "serde")]
#[test]
fn serialize_deserialize_test_inline_bytes() {
    common::serialize::serialize_deserialize_test::<InlineStr<[u8]>, [u8]>(b"test");
}
