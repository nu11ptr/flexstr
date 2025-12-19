#![cfg(feature = "bytes")]

extern crate alloc;

use alloc::sync::Arc;

#[cfg(feature = "serde")]
use flexstry::{InlineBytes, LocalBytes, SharedBytes};

mod common;

// *** Serialize/Deserialize Tests ***

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
    common::serialize::serialize_deserialize_test::<InlineBytes, [u8]>(b"test");
}

// *** Mutation Tests ***

#[test]
fn test_mutation_borrowed_bytes() {
    common::mutate::test_mutation_borrowed::<[u8], Arc<[u8]>>(b"test");
}

#[test]
fn test_mutation_inlined_bytes() {
    common::mutate::test_mutation_inlined::<[u8], Arc<[u8]>>(b"test");
}

#[test]
fn test_mutation_ref_counted_bytes() {
    common::mutate::test_mutation_ref_counted::<[u8], Arc<[u8]>>((*b"test").into());
}

#[test]
fn test_mutation_boxed_bytes() {
    common::mutate::test_mutation_boxed::<[u8], Arc<[u8]>>(b"test".into());
}
