#![cfg(all(feature = "std", feature = "osstr"))]

use std::ffi::OsStr;

#[cfg(feature = "serde")]
use flexstry::{InlineStr, LocalOsStr, SharedOsStr};

mod common;

#[cfg(feature = "serde")]
#[test]
fn serialize_deserialize_test_local_osstr() {
    common::serialize::serialize_deserialize_test::<LocalOsStr<'_>, OsStr>(OsStr::new("test"));
}

#[cfg(feature = "serde")]
#[test]
fn serialize_deserialize_test_shared_osstr() {
    common::serialize::serialize_deserialize_test::<SharedOsStr<'_>, OsStr>(OsStr::new("test"));
}

#[cfg(feature = "serde")]
#[test]
fn serialize_deserialize_test_inline_osstr() {
    common::serialize::serialize_deserialize_test::<InlineStr<OsStr>, OsStr>(OsStr::new("test"));
}
