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

// *** Basic Tests ***

#[test]
fn test_creation_from_borrowed_bytes() {
    common::basic::test_creation_from_borrowed::<[u8], Arc<[u8]>>(b"test");
}

#[test]
fn test_creation_from_inline_bytes() {
    common::basic::test_creation_from_inline::<[u8], Arc<[u8]>>(b"test");
}

#[test]
fn test_creation_from_ref_counted_bytes() {
    common::basic::test_creation_from_ref_counted::<[u8], Arc<[u8]>>((*b"test").into());
}

#[test]
fn test_empty_bytes() {
    common::basic::test_empty::<[u8], Arc<[u8]>>(b"");
}

#[test]
fn test_accessors_bytes() {
    common::basic::test_accessors::<[u8], Arc<[u8]>>(b"test");
}

#[test]
fn test_clone_all_variants_bytes() {
    common::basic::test_clone_all_variants::<[u8], Arc<[u8]>>(b"test");
}

#[test]
fn test_default_bytes() {
    common::basic::test_default::<[u8], Arc<[u8]>>();
}

// *** Conversion Tests ***

#[test]
fn test_to_owned_bytes() {
    common::conversion::test_to_owned::<[u8], Arc<[u8]>>(b"test");
}

#[test]
fn test_into_owned_bytes() {
    common::conversion::test_into_owned::<[u8], Arc<[u8]>>(b"test");
}

#[test]
fn test_to_owned_type_bytes() {
    common::conversion::test_to_owned_type::<[u8], Arc<[u8]>>(b"test");
}

#[test]
fn test_optimize_bytes() {
    common::conversion::test_optimize::<[u8], Arc<[u8]>>(b"test");
}

#[test]
fn test_from_borrowed_ref_bytes() {
    common::conversion::test_from_borrowed_ref::<[u8], Arc<[u8]>>(b"test");
}

#[test]
fn test_from_inline_flex_str_bytes() {
    common::conversion::test_from_inline_flex_str::<[u8], Arc<[u8]>>(b"test");
}

// *** Comparison Tests ***

#[test]
fn test_partial_eq_bytes() {
    common::comparison::test_partial_eq::<[u8], Arc<[u8]>>(b"test", b"test");
    common::comparison::test_partial_eq::<[u8], Arc<[u8]>>(b"test", b"other");
}

#[test]
fn test_eq_bytes() {
    common::comparison::test_eq::<[u8], Arc<[u8]>>(b"test");
}

#[test]
fn test_partial_ord_bytes() {
    common::comparison::test_partial_ord::<[u8], Arc<[u8]>>(b"a", b"b");
}

#[test]
fn test_ord_bytes() {
    common::comparison::test_ord::<[u8], Arc<[u8]>>(b"a", b"b");
}

#[test]
fn test_hash_bytes() {
    common::comparison::test_hash::<[u8], Arc<[u8]>>(b"test");
}

#[test]
fn test_comparison_with_ref_bytes() {
    common::comparison::test_comparison_with_ref::<[u8], Arc<[u8]>>(b"test");
}

// *** Storage Tests ***

#[test]
fn test_variant_queries_bytes() {
    common::storage::test_variant_queries::<[u8], Arc<[u8]>>(b"test");
}

#[test]
fn test_transition_borrowed_to_inlined_bytes() {
    common::storage::test_transition_borrowed_to_inlined::<[u8], Arc<[u8]>>(b"test");
}

#[test]
fn test_storage_optimization_bytes() {
    common::storage::test_storage_optimization::<[u8], Arc<[u8]>>(b"test");
}

// *** Edge Case Tests ***

#[test]
fn test_empty_string_bytes() {
    common::edge_cases::test_empty_string::<[u8], Arc<[u8]>>(b"");
}

#[test]
fn test_various_lengths_bytes() {
    common::edge_cases::test_various_lengths::<[u8], Arc<[u8]>>(b"test");
    common::edge_cases::test_various_lengths::<[u8], Arc<[u8]>>(b"");
    common::edge_cases::test_various_lengths::<[u8], Arc<[u8]>>(b"a");
}

#[test]
fn test_special_content_bytes() {
    common::edge_cases::test_special_content::<[u8], Arc<[u8]>>(b"test");
}

#[test]
fn test_clone_variants_bytes() {
    common::edge_cases::test_clone_variants::<[u8], Arc<[u8]>>(b"test");
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
