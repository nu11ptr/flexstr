#![cfg(feature = "cstr")]

extern crate alloc;

use alloc::{rc::Rc, sync::Arc};

#[cfg(feature = "serde")]
use flexstry::{InlineCStr, LocalCStr, SharedCStr};

use core::ffi::CStr;

mod common;

// *** Serialize/Deserialize Tests ***

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
    common::serialize::serialize_deserialize_test::<InlineCStr, CStr>(c"test");
}

// *** Basic Tests ***

#[test]
fn test_creation_from_borrowed_cstr() {
    common::basic::test_creation_from_borrowed::<CStr, Arc<CStr>>(c"test");
}

#[test]
fn test_creation_from_inline_cstr() {
    common::basic::test_creation_from_inline::<CStr, Arc<CStr>>(c"test");
}

#[test]
fn test_creation_from_ref_counted_cstr() {
    common::basic::test_creation_from_ref_counted::<CStr, Arc<CStr>>(c"test".into());
}

#[test]
fn test_empty_cstr() {
    common::basic::test_empty::<CStr, Arc<CStr>>(c"");
}

#[test]
fn test_accessors_cstr() {
    common::basic::test_accessors::<CStr, Arc<CStr>>(c"test");
}

#[test]
fn test_clone_all_variants_cstr() {
    common::basic::test_clone_all_variants::<CStr, Arc<CStr>>(c"test");
}

#[test]
fn test_default_cstr() {
    common::basic::test_default::<CStr, Arc<CStr>>();
}

// *** Conversion Tests ***

#[test]
fn test_to_owned_cstr() {
    common::conversion::test_to_owned::<CStr, Arc<CStr>>(c"test");
}

#[test]
fn test_into_owned_cstr() {
    common::conversion::test_into_owned::<CStr, Arc<CStr>>(c"test");
}

#[test]
fn test_to_owned_type_cstr() {
    common::conversion::test_to_owned_type::<CStr, Arc<CStr>>(c"test");
}

#[test]
fn test_optimize_cstr() {
    common::conversion::test_optimize::<CStr, Arc<CStr>>(c"test");
}

#[test]
fn test_from_borrowed_ref_cstr() {
    common::conversion::test_from_borrowed_ref::<CStr, Arc<CStr>>(c"test");
}

#[test]
fn test_from_inline_flex_str_cstr() {
    common::conversion::test_from_inline_flex_str::<CStr, Arc<CStr>>(c"test");
}

// *** Comparison Tests ***

#[test]
fn test_partial_eq_cstr() {
    common::comparison::test_partial_eq::<CStr, Arc<CStr>>(c"test", c"test");
    common::comparison::test_partial_eq::<CStr, Arc<CStr>>(c"test", c"other");
}

#[test]
fn test_eq_cstr() {
    common::comparison::test_eq::<CStr, Arc<CStr>>(c"test");
}

#[test]
fn test_hash_cstr() {
    common::comparison::test_hash::<CStr, Arc<CStr>>(c"test");
}

#[test]
fn test_comparison_with_ref_cstr() {
    common::comparison::test_comparison_with_ref::<CStr, Arc<CStr>>(c"test");
}

// *** Storage Tests ***

#[test]
fn test_variant_queries_cstr() {
    common::storage::test_variant_queries::<CStr, Arc<CStr>>(c"test");
}

#[test]
fn test_transition_borrowed_to_inlined_cstr() {
    common::storage::test_transition_borrowed_to_inlined::<CStr, Arc<CStr>>(c"test");
}

#[test]
fn test_storage_optimization_cstr() {
    common::storage::test_storage_optimization::<CStr, Arc<CStr>>(c"test");
}

// *** Edge Case Tests ***

#[test]
fn test_empty_string_cstr() {
    common::edge_cases::test_empty_string::<CStr, Arc<CStr>>(c"");
}

#[test]
fn test_various_lengths_cstr() {
    common::edge_cases::test_various_lengths::<CStr, Arc<CStr>>(c"test");
    common::edge_cases::test_various_lengths::<CStr, Arc<CStr>>(c"");
}

#[test]
fn test_special_content_cstr() {
    common::edge_cases::test_special_content::<CStr, Arc<CStr>>(c"test");
}

#[test]
fn test_clone_variants_cstr() {
    common::edge_cases::test_clone_variants::<CStr, Arc<CStr>>(c"test");
}

// *** Error Tests ***

#[test]
fn test_interior_nul_error() {
    common::errors::test_interior_nul_error();
}

#[test]
fn test_too_long_or_nul_error_too_long() {
    common::errors::test_too_long_or_nul_error_too_long();
}

#[test]
fn test_too_long_or_nul_error_nul() {
    common::errors::test_too_long_or_nul_error_nul();
}

// *** StringLike Tests ***

#[test]
fn test_as_c_str() {
    common::stringlike::test_as_c_str::<CStr, Arc<CStr>>(c"test");
}

#[test]
fn test_into_c_string() {
    common::stringlike::test_into_c_string::<CStr, Arc<CStr>>(c"test");
}

#[test]
fn test_to_c_string() {
    common::stringlike::test_to_c_string::<CStr, Arc<CStr>>(c"test");
}

// *** CStr-Specific Tests ***

#[test]
fn test_try_from_bytes_with_nul() {
    common::cstr_specific::test_try_from_bytes_with_nul::<Arc<CStr>>();
}

#[test]
fn test_try_from_bytes_without_nul() {
    common::cstr_specific::test_try_from_bytes_without_nul::<Arc<CStr>>();
}

#[test]
fn test_try_from_bytes_interior_nul() {
    common::cstr_specific::test_try_from_bytes_interior_nul::<Arc<CStr>>();
}

#[test]
fn test_as_bytes_with_nul() {
    common::cstr_specific::test_as_bytes_with_nul::<Arc<CStr>>(c"test");
}

#[test]
fn test_inline_try_from_bytes_with_nul() {
    common::cstr_specific::test_inline_try_from_bytes_with_nul();
}

#[test]
fn test_inline_try_from_bytes_without_nul() {
    common::cstr_specific::test_inline_try_from_bytes_without_nul();
}

#[test]
fn test_inline_try_from_bytes_interior_nul() {
    common::cstr_specific::test_inline_try_from_bytes_interior_nul();
}

#[test]
fn test_inline_try_from_bytes_too_long() {
    common::cstr_specific::test_inline_try_from_bytes_too_long();
}

// *** InlineFlexStr Edge Cases ***

#[test]
fn test_inline_default_cstr() {
    common::inline_edge_cases::test_inline_default::<CStr>();
}

#[test]
fn test_try_from_type_too_long_cstr() {
    let mut long_bytes = vec![b'x'; flexstry::INLINE_CAPACITY];
    long_bytes.push(0);
    let long_bytes_static: &'static [u8] = Box::leak(long_bytes.into_boxed_slice());
    let long_cstr = CStr::from_bytes_with_nul(long_bytes_static).unwrap();
    common::inline_edge_cases::test_try_from_type_too_long::<CStr>(long_cstr);
}

// *** Mutation Tests ***

#[test]
fn test_mutation_borrowed_shared_cstr() {
    common::mutate_fallback::test_mutation_immutable_bytes_borrowed::<CStr, Arc<CStr>>(c"test");
}

#[test]
fn test_mutation_borrowed_local_cstr() {
    common::mutate_fallback::test_mutation_immutable_bytes_borrowed::<CStr, Rc<CStr>>(c"test");
}

#[test]
fn test_mutation_inlined_shared_cstr() {
    common::mutate_fallback::test_mutation_immutable_bytes_inlined::<CStr, Arc<CStr>>(c"test");
}

#[test]
fn test_mutation_inlined_local_cstr() {
    common::mutate_fallback::test_mutation_immutable_bytes_inlined::<CStr, Rc<CStr>>(c"test");
}

#[test]
fn test_mutation_shared_ref_counted_cstr() {
    common::mutate_fallback::test_mutation_immutable_bytes_ref_counted::<CStr, Arc<CStr>>(
        c"test".into(),
    );
}

#[test]
fn test_mutation_local_ref_counted_cstr() {
    common::mutate_fallback::test_mutation_immutable_bytes_ref_counted::<CStr, Rc<CStr>>(
        c"test".into(),
    );
}

// NOTE: Boxed strings don't use Rc/Arc, so we don't need to test both
#[test]
fn test_mutation_boxed_cstr() {
    common::mutate_fallback::test_mutation_immutable_bytes_boxed::<CStr, Arc<CStr>>(c"test".into());
}
