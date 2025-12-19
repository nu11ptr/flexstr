#![cfg(feature = "str")]

extern crate alloc;

use alloc::sync::Arc;

#[cfg(feature = "serde")]
use flexstry::{InlineStr, LocalStr, SharedStr};

mod common;

// *** Serialize/Deserialize Tests ***

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
    common::serialize::serialize_deserialize_test::<InlineStr, str>("test");
}

// *** Mutation Tests ***

#[test]
fn test_mutation_borrowed_str() {
    common::mutate::test_mutation_borrowed::<str, Arc<str>>("test");
}

#[test]
fn test_mutation_inlined_str() {
    common::mutate::test_mutation_inlined::<str, Arc<str>>("test");
}

#[test]
fn test_mutation_ref_counted_str() {
    common::mutate::test_mutation_ref_counted::<str, Arc<str>>("test".into());
}

#[test]
fn test_mutation_boxed_str() {
    common::mutate::test_mutation_boxed::<str, Arc<str>>("test".into());
}
