#![cfg(all(feature = "std", feature = "path"))]

extern crate alloc;

use alloc::{rc::Rc, sync::Arc};

#[cfg(feature = "serde")]
use flexstry::{InlinePath, LocalPath, SharedPath};

use std::path::Path;

mod common;

// *** Serialize/Deserialize Tests ***

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
    common::serialize::serialize_deserialize_test::<InlinePath, Path>(Path::new("test"));
}

// *** Basic Tests ***

#[test]
fn test_creation_from_borrowed_path() {
    common::basic::test_creation_from_borrowed::<Path, Arc<Path>>(Path::new("test"));
}

#[test]
fn test_creation_from_inline_path() {
    common::basic::test_creation_from_inline::<Path, Arc<Path>>(Path::new("test"));
}

#[test]
fn test_creation_from_ref_counted_path() {
    common::basic::test_creation_from_ref_counted::<Path, Arc<Path>>(Path::new("test").into());
}

#[test]
fn test_empty_path() {
    common::basic::test_empty::<Path, Arc<Path>>(Path::new(""));
}

#[test]
fn test_accessors_path() {
    common::basic::test_accessors::<Path, Arc<Path>>(Path::new("test"));
}

#[test]
fn test_clone_all_variants_path() {
    common::basic::test_clone_all_variants::<Path, Arc<Path>>(Path::new("test"));
}

// Path doesn't implement Default, so skip this test
// #[test]
// fn test_default_path() {
//     common::basic::test_default::<Path, Arc<Path>>();
// }

// *** Conversion Tests ***

#[test]
fn test_to_owned_path() {
    common::conversion::test_to_owned::<Path, Arc<Path>>(Path::new("test"));
}

#[test]
fn test_into_owned_path() {
    common::conversion::test_into_owned::<Path, Arc<Path>>(Path::new("test"));
}

#[test]
fn test_to_owned_type_path() {
    common::conversion::test_to_owned_type::<Path, Arc<Path>>(Path::new("test"));
}

#[test]
fn test_optimize_path() {
    common::conversion::test_optimize::<Path, Arc<Path>>(Path::new("test"));
}

#[test]
fn test_from_borrowed_ref_path() {
    common::conversion::test_from_borrowed_ref::<Path, Arc<Path>>(Path::new("test"));
}

#[test]
fn test_from_inline_flex_str_path() {
    common::conversion::test_from_inline_flex_str::<Path, Arc<Path>>(Path::new("test"));
}

// *** Comparison Tests ***

#[test]
fn test_partial_eq_path() {
    common::comparison::test_partial_eq::<Path, Arc<Path>>(Path::new("test"), Path::new("test"));
    common::comparison::test_partial_eq::<Path, Arc<Path>>(Path::new("test"), Path::new("other"));
}

#[test]
fn test_eq_path() {
    common::comparison::test_eq::<Path, Arc<Path>>(Path::new("test"));
}

#[test]
fn test_hash_path() {
    common::comparison::test_hash::<Path, Arc<Path>>(Path::new("test"));
}

#[test]
fn test_comparison_with_ref_path() {
    common::comparison::test_comparison_with_ref::<Path, Arc<Path>>(Path::new("test"));
}

// *** Storage Tests ***

#[test]
fn test_variant_queries_path() {
    common::storage::test_variant_queries::<Path, Arc<Path>>(Path::new("test"));
}

#[test]
fn test_transition_borrowed_to_inlined_path() {
    common::storage::test_transition_borrowed_to_inlined::<Path, Arc<Path>>(Path::new("test"));
}

#[test]
fn test_storage_optimization_path() {
    common::storage::test_storage_optimization::<Path, Arc<Path>>(Path::new("test"));
}

// *** Edge Case Tests ***

#[test]
fn test_empty_string_path() {
    common::edge_cases::test_empty_string::<Path, Arc<Path>>(Path::new(""));
}

#[test]
fn test_various_lengths_path() {
    common::edge_cases::test_various_lengths::<Path, Arc<Path>>(Path::new("test"));
    common::edge_cases::test_various_lengths::<Path, Arc<Path>>(Path::new(""));
}

#[test]
fn test_special_content_path() {
    common::edge_cases::test_special_content::<Path, Arc<Path>>(Path::new("test"));
}

#[test]
fn test_clone_variants_path() {
    common::edge_cases::test_clone_variants::<Path, Arc<Path>>(Path::new("test"));
}

// *** Mutation Tests ***

#[test]
fn test_mutation_borrowed_shared_path() {
    common::mutate_fallback::test_mutation_immutable_bytes_borrowed::<Path, Arc<Path>>(Path::new(
        "test",
    ));
}

#[test]
fn test_mutation_borrowed_local_path() {
    common::mutate_fallback::test_mutation_immutable_bytes_borrowed::<Path, Rc<Path>>(Path::new(
        "test",
    ));
}

#[test]
fn test_mutation_inlined_shared_path() {
    common::mutate_fallback::test_mutation_immutable_bytes_inlined::<Path, Arc<Path>>(Path::new(
        "test",
    ));
}

#[test]
fn test_mutation_inlined_local_path() {
    common::mutate_fallback::test_mutation_immutable_bytes_inlined::<Path, Rc<Path>>(Path::new(
        "test",
    ));
}

#[test]
fn test_mutation_shared_ref_counted_path() {
    common::mutate_fallback::test_mutation_immutable_bytes_ref_counted::<Path, Arc<Path>>(
        Path::new("test").into(),
    );
}

#[test]
fn test_mutation_local_ref_counted_path() {
    common::mutate_fallback::test_mutation_immutable_bytes_ref_counted::<Path, Rc<Path>>(
        Path::new("test").into(),
    );
}

// NOTE: Boxed strings don't use Rc/Arc, so we don't need to test both
#[test]
fn test_mutation_boxed_path() {
    common::mutate_fallback::test_mutation_immutable_bytes_boxed::<Path, Arc<Path>>(
        Path::new("test").into(),
    );
}
