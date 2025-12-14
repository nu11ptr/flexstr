#![cfg(feature = "cstr")]

#[cfg(feature = "serde")]
use flexstry::{InlineStr, LocalCStr, SharedCStr};

use core::ffi::CStr;

mod common;

#[cfg(feature = "serde")]
#[test]
fn serialize_deserialize_test_local_cstr() {
    common::serialize::serialize_deserialize_test::<LocalCStr<'_>, CStr>(c"test");
}

#[cfg(feature = "serde")]
#[test]
fn serialize_deserialize_test_shared_cstr() {
    common::serialize::serialize_deserialize_test::<SharedCStr<'_>, CStr>(c"test");
}

#[cfg(feature = "serde")]
#[test]
fn serialize_deserialize_test_inline_cstr() {
    common::serialize::serialize_deserialize_test::<InlineStr<CStr>, CStr>(c"test");
}
