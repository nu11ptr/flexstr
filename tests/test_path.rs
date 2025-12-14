#![cfg(all(feature = "std", feature = "path"))]

#[cfg(feature = "serde")]
use flexstry::{InlineFlexStr, LocalPath, SharedPath};

use std::path::Path;

mod common;

#[cfg(feature = "serde")]
#[test]
fn serialize_deserialize_test_local_path() {
    common::serialize::serialize_deserialize_test::<LocalPath<'_>, Path>(Path::new("test"));
}

#[cfg(feature = "serde")]
#[test]
fn serialize_deserialize_test_shared_path() {
    common::serialize::serialize_deserialize_test::<SharedPath<'_>, Path>(Path::new("test"));
}

#[cfg(feature = "serde")]
#[test]
fn serialize_deserialize_test_inline_path() {
    common::serialize::serialize_deserialize_test::<InlineFlexStr<Path>, Path>(Path::new("test"));
}
