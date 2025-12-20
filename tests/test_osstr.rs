#![cfg(all(feature = "std", feature = "osstr"))]

extern crate alloc;

use alloc::{rc::Rc, sync::Arc};
use std::ffi::OsStr;

#[cfg(feature = "serde")]
use flexstry::{InlineOsStr, LocalOsStr, SharedOsStr};

mod common;

// *** Serialize/Deserialize Tests ***

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
    common::serialize::serialize_deserialize_test::<InlineOsStr, OsStr>(OsStr::new("test"));
}

// *** Basic Tests ***

#[test]
fn test_creation_from_borrowed_osstr() {
    common::basic::test_creation_from_borrowed::<OsStr, Arc<OsStr>>(OsStr::new("test"));
}

#[test]
fn test_creation_from_inline_osstr() {
    common::basic::test_creation_from_inline::<OsStr, Arc<OsStr>>(OsStr::new("test"));
}

#[test]
fn test_creation_from_ref_counted_osstr() {
    common::basic::test_creation_from_ref_counted::<OsStr, Arc<OsStr>>(OsStr::new("test").into());
}

#[test]
fn test_empty_osstr() {
    common::basic::test_empty::<OsStr, Arc<OsStr>>(OsStr::new(""));
}

#[test]
fn test_accessors_osstr() {
    common::basic::test_accessors::<OsStr, Arc<OsStr>>(OsStr::new("test"));
}

#[test]
fn test_clone_all_variants_osstr() {
    common::basic::test_clone_all_variants::<OsStr, Arc<OsStr>>(OsStr::new("test"));
}

#[test]
fn test_default_osstr() {
    common::basic::test_default::<OsStr, Arc<OsStr>>();
}

// *** Conversion Tests ***

#[test]
fn test_to_owned_osstr() {
    common::conversion::test_to_owned::<OsStr, Arc<OsStr>>(OsStr::new("test"));
}

#[test]
fn test_into_owned_osstr() {
    common::conversion::test_into_owned::<OsStr, Arc<OsStr>>(OsStr::new("test"));
}

#[test]
fn test_to_owned_type_osstr() {
    common::conversion::test_to_owned_type::<OsStr, Arc<OsStr>>(OsStr::new("test"));
}

#[test]
fn test_optimize_osstr() {
    common::conversion::test_optimize::<OsStr, Arc<OsStr>>(OsStr::new("test"));
}

#[test]
fn test_from_borrowed_ref_osstr() {
    common::conversion::test_from_borrowed_ref::<OsStr, Arc<OsStr>>(OsStr::new("test"));
}

#[test]
fn test_from_inline_flex_str_osstr() {
    common::conversion::test_from_inline_flex_str::<OsStr, Arc<OsStr>>(OsStr::new("test"));
}

// *** Comparison Tests ***

#[test]
fn test_partial_eq_osstr() {
    common::comparison::test_partial_eq::<OsStr, Arc<OsStr>>(
        OsStr::new("test"),
        OsStr::new("test"),
    );
    common::comparison::test_partial_eq::<OsStr, Arc<OsStr>>(
        OsStr::new("test"),
        OsStr::new("other"),
    );
}

#[test]
fn test_eq_osstr() {
    common::comparison::test_eq::<OsStr, Arc<OsStr>>(OsStr::new("test"));
}

#[test]
fn test_hash_osstr() {
    common::comparison::test_hash::<OsStr, Arc<OsStr>>(OsStr::new("test"));
}

#[test]
fn test_comparison_with_ref_osstr() {
    common::comparison::test_comparison_with_ref::<OsStr, Arc<OsStr>>(OsStr::new("test"));
}

// *** Storage Tests ***

#[test]
fn test_variant_queries_osstr() {
    common::storage::test_variant_queries::<OsStr, Arc<OsStr>>(OsStr::new("test"));
}

#[test]
fn test_transition_borrowed_to_inlined_osstr() {
    common::storage::test_transition_borrowed_to_inlined::<OsStr, Arc<OsStr>>(OsStr::new("test"));
}

#[test]
fn test_storage_optimization_osstr() {
    common::storage::test_storage_optimization::<OsStr, Arc<OsStr>>(OsStr::new("test"));
}

// *** Edge Case Tests ***

#[test]
fn test_empty_string_osstr() {
    common::edge_cases::test_empty_string::<OsStr, Arc<OsStr>>(OsStr::new(""));
}

#[test]
fn test_various_lengths_osstr() {
    common::edge_cases::test_various_lengths::<OsStr, Arc<OsStr>>(OsStr::new("test"));
    common::edge_cases::test_various_lengths::<OsStr, Arc<OsStr>>(OsStr::new(""));
}

#[test]
fn test_special_content_osstr() {
    common::edge_cases::test_special_content::<OsStr, Arc<OsStr>>(OsStr::new("test"));
}

#[test]
fn test_clone_variants_osstr() {
    common::edge_cases::test_clone_variants::<OsStr, Arc<OsStr>>(OsStr::new("test"));
}

// *** Mutation Tests ***

#[test]
fn test_mutation_shared_borrowed_osstr() {
    common::mutate_fallback::test_mutation_immutable_bytes_borrowed::<OsStr, Arc<OsStr>>(
        OsStr::new("test"),
    );
}

#[test]
fn test_mutation_local_borrowed_osstr() {
    common::mutate_fallback::test_mutation_immutable_bytes_borrowed::<OsStr, Rc<OsStr>>(
        OsStr::new("test"),
    );
}

#[test]
fn test_mutation_shared_inlined_osstr() {
    common::mutate_fallback::test_mutation_immutable_bytes_inlined::<OsStr, Arc<OsStr>>(
        OsStr::new("test"),
    );
}

#[test]
fn test_mutation_local_inlined_osstr() {
    common::mutate_fallback::test_mutation_immutable_bytes_inlined::<OsStr, Rc<OsStr>>(OsStr::new(
        "test",
    ));
}

#[test]
fn test_mutation_shared_ref_counted_osstr() {
    common::mutate_fallback::test_mutation_immutable_bytes_ref_counted::<OsStr, Arc<OsStr>>(
        OsStr::new("test").into(),
    );
}

#[test]
fn test_mutation_local_ref_counted_osstr() {
    common::mutate_fallback::test_mutation_immutable_bytes_ref_counted::<OsStr, Rc<OsStr>>(
        OsStr::new("test").into(),
    );
}

// NOTE: Boxed strings don't use Rc/Arc, so we don't need to test both
#[test]
fn test_mutation_boxed_osstr() {
    common::mutate_fallback::test_mutation_immutable_bytes_boxed::<OsStr, Arc<OsStr>>(
        OsStr::new("test").into(),
    );
}
