#![cfg(feature = "str")]

extern crate alloc;

use alloc::sync::Arc;

#[cfg(feature = "serde")]
use flexstry::{LocalStr, SharedStr};
use inline_flexstr::INLINE_CAPACITY;

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

// *** Basic Tests ***

#[test]
fn test_creation_from_borrowed_str() {
    common::basic::test_creation_from_borrowed::<str, Arc<str>>("test");
}

#[test]
fn test_creation_from_owned_str() {
    common::basic::test_creation_from_owned::<str, Arc<str>>("test".to_string());
}

#[test]
fn test_creation_from_inline_str() {
    common::basic::test_creation_from_inline::<str, Arc<str>>("test");
}

#[test]
fn test_creation_from_ref_counted_str() {
    common::basic::test_creation_from_ref_counted::<str, Arc<str>>("test".into());
}

#[test]
fn test_creation_from_boxed_str() {
    common::basic::test_creation_from_boxed::<str, Arc<str>>("test");
}

#[test]
fn test_empty_str() {
    common::basic::test_empty::<str, Arc<str>>("");
}

#[test]
fn test_accessors_str() {
    common::basic::test_accessors::<str, Arc<str>>("test");
}

#[test]
fn test_clone_all_variants_str() {
    common::basic::test_clone_all_variants::<str, Arc<str>>("test");
}

#[test]
fn test_default_str() {
    common::basic::test_default::<str, Arc<str>>();
}

// *** Conversion Tests ***

#[test]
fn test_to_owned_str() {
    common::conversion::test_to_owned::<str, Arc<str>>("test");
}

#[test]
fn test_into_owned_str() {
    common::conversion::test_into_owned::<str, Arc<str>>("test");
}

#[test]
fn test_to_owned_type_str() {
    common::conversion::test_to_owned_type::<str, Arc<str>>("test");
}

#[test]
fn test_into_owned_type_str() {
    common::conversion::test_into_owned_type::<str, Arc<str>>("test");
}

#[test]
fn test_to_local_str() {
    common::conversion::test_to_local::<str>("test");
}

#[test]
fn test_into_local_str() {
    common::conversion::test_into_local::<str>("test");
}

#[test]
fn test_to_shared_str() {
    common::conversion::test_to_shared::<str>("test");
}

#[test]
fn test_into_shared_str() {
    common::conversion::test_into_shared::<str>("test");
}

#[test]
fn test_optimize_str() {
    common::conversion::test_optimize::<str, Arc<str>>("test");
}

#[test]
fn test_from_borrowed_ref_str() {
    common::conversion::test_from_borrowed_ref::<str, Arc<str>>("test");
}

#[test]
fn test_from_box_str() {
    common::conversion::test_from_box::<str, Arc<str>>("test");
}

#[test]
fn test_from_inline_flex_str_str() {
    common::conversion::test_from_inline_flex_str::<str, Arc<str>>("test");
}

#[test]
fn test_from_cow_str() {
    common::conversion::test_from_cow::<str, Arc<str>>("test");
}

// *** Comparison Tests ***

#[test]
fn test_partial_eq_str() {
    common::comparison::test_partial_eq::<str, Arc<str>>("test", "test");
    common::comparison::test_partial_eq::<str, Arc<str>>("test", "other");
}

#[test]
fn test_eq_str() {
    common::comparison::test_eq::<str, Arc<str>>("test");
}

#[test]
fn test_partial_ord_str() {
    common::comparison::test_partial_ord::<str, Arc<str>>("a", "b");
}

#[test]
fn test_ord_str() {
    common::comparison::test_ord::<str, Arc<str>>("a", "b");
}

#[test]
fn test_hash_str() {
    common::comparison::test_hash::<str, Arc<str>>("test");
}

#[test]
fn test_comparison_with_ref_str() {
    common::comparison::test_comparison_with_ref::<str, Arc<str>>("test");
}

#[test]
fn test_comparison_with_owned_str() {
    common::comparison::test_comparison_with_owned::<str, Arc<str>>("test");
}

#[test]
fn test_comparison_with_inline_str() {
    common::comparison::test_comparison_with_inline::<str, Arc<str>>("test");
}

#[test]
fn test_partial_eq_with_owned_types_str() {
    common::comparison::test_partial_eq_with_owned_types::<str, Arc<str>>("test");
}

// *** Storage Tests ***

#[test]
fn test_variant_queries_str() {
    common::storage::test_variant_queries::<str, Arc<str>>("test");
}

#[test]
fn test_transition_borrowed_to_inlined_str() {
    common::storage::test_transition_borrowed_to_inlined::<str, Arc<str>>("test");
}

#[test]
fn test_transition_borrowed_to_ref_counted_str() {
    common::storage::test_transition_borrowed_to_ref_counted::<str, Arc<str>>(
        "this is a very long string that definitely won't fit inline",
    );
}

#[test]
fn test_transition_inlined_to_ref_counted_str() {
    common::storage::test_transition_inlined_to_ref_counted::<str, Arc<str>>("test");
}

#[test]
fn test_transition_boxed_to_optimized_str() {
    common::storage::test_transition_boxed_to_optimized::<str, Arc<str>>("test");
}

#[test]
fn test_storage_optimization_str() {
    common::storage::test_storage_optimization::<str, Arc<str>>("test");
}

// *** Edge Case Tests ***

#[test]
fn test_empty_string_str() {
    common::edge_cases::test_empty_string::<str, Arc<str>>("");
}

#[test]
fn test_capacity_boundary_exact_str() {
    // Create a string exactly at capacity
    let s = "a".repeat(INLINE_CAPACITY);
    let s_static: &'static str = Box::leak(s.into_boxed_str());
    common::edge_cases::test_capacity_boundary_exact::<str, Arc<str>>(s_static);
}

#[test]
fn test_capacity_boundary_overflow_str() {
    common::edge_cases::test_capacity_boundary_overflow::<str>("test");
}

#[test]
fn test_try_from_too_long_str() {
    common::edge_cases::test_try_from_too_long();
}

#[test]
fn test_various_lengths_str() {
    common::edge_cases::test_various_lengths::<str, Arc<str>>("test");
    common::edge_cases::test_various_lengths::<str, Arc<str>>("");
    common::edge_cases::test_various_lengths::<str, Arc<str>>("a");
}

#[test]
fn test_special_content_str() {
    common::edge_cases::test_special_content::<str, Arc<str>>("test");
    common::edge_cases::test_special_content::<str, Arc<str>>("hello\nworld");
    common::edge_cases::test_special_content::<str, Arc<str>>("🚀");
}

#[test]
fn test_clone_variants_str() {
    common::edge_cases::test_clone_variants::<str, Arc<str>>("test");
}

// *** Error Tests ***

#[test]
fn test_too_long_for_inlining() {
    common::errors::test_too_long_for_inlining();
}

#[test]
fn test_too_long_or_utf8_error_too_long() {
    common::errors::test_too_long_or_utf8_error_too_long();
}

#[test]
fn test_too_long_or_utf8_error_utf8() {
    common::errors::test_too_long_or_utf8_error_utf8();
}

// *** StringLike Tests ***

#[test]
fn test_as_str() {
    common::stringlike::test_as_str::<str, Arc<str>>("test");
}

#[test]
fn test_into_string() {
    common::stringlike::test_into_string::<str, Arc<str>>("test");
}

#[test]
fn test_to_string() {
    common::stringlike::test_to_string::<str, Arc<str>>("test");
}

// *** TryFrom Tests ***

#[test]
fn test_try_from_bytes_invalid_utf8() {
    common::try_from::test_try_from_bytes_invalid_utf8::<Arc<str>>();
}

#[test]
fn test_try_from_vec_bytes_invalid_utf8() {
    common::try_from::test_try_from_vec_bytes_invalid_utf8::<Arc<str>>();
}

// *** From Tests ***

#[test]
fn test_from_string() {
    common::from::test_from_string_str::<Arc<str>>();
}

// *** FromStr Tests ***

#[test]
fn test_from_str_flex_str_success() {
    common::from_str::test_from_str_flex_str_success::<str, Arc<str>>("test");
}

// *** FlexStr Edge Cases ***

#[test]
fn test_optimize_ref_counted_to_inlined() {
    common::inline_edge_cases::test_optimize_ref_counted_to_inlined::<str, Arc<str>>("test");
}

#[test]
fn test_optimize_ref_counted_stays_ref_counted() {
    let long_str: &'static str = Box::leak(Box::new("x".repeat(INLINE_CAPACITY + 1)));
    common::inline_edge_cases::test_optimize_ref_counted_stays_ref_counted::<str, Arc<str>>(
        long_str,
    );
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

// *** Display Tests ***

#[test]
fn test_display_str() {
    common::display::test_display::<str, Arc<str>>("test");
}

// *** Borrow Tests ***

#[test]
fn test_borrow_str() {
    common::borrow::test_borrow::<str, Arc<str>>("test");
}

// *** Index Tests ***

#[test]
fn test_index_str() {
    common::index::test_index::<str, Arc<str>>("test");
}

// *** ToSocketAddrs Tests ***

#[cfg(feature = "std")]
#[test]
fn test_to_socket_addrs_str() {
    common::socket::test_to_socket_addrs::<str, Arc<str>>("127.0.0.1:8080");
}

// *** TryFrom Tests ***

#[cfg(feature = "std")]
#[test]
fn test_try_from_osstr_str() {
    common::try_from::test_try_from_osstr_str::<Arc<str>>();
}

#[cfg(feature = "std")]
#[test]
fn test_try_from_path_str() {
    common::try_from::test_try_from_path_str::<Arc<str>>();
}

#[test]
fn test_try_from_vec_u8_str() {
    common::try_from::test_try_from_vec_u8_str::<Arc<str>>();
}

#[cfg(feature = "cstr")]
#[test]
fn test_try_from_cstring_str() {
    common::try_from::test_try_from_cstring_str::<Arc<str>>();
}

// *** FromStr Tests ***

#[cfg(feature = "cstr")]
#[test]
fn test_from_str_cstr_success() {
    common::from_str::test_from_str_cstr_success::<Arc<core::ffi::CStr>>();
}

#[cfg(feature = "cstr")]
#[test]
fn test_from_str_cstr_error() {
    common::from_str::test_from_str_cstr_error::<Arc<core::ffi::CStr>>();
}

// *** AsRef Tests ***

#[test]
fn test_as_ref_str_flex_str() {
    common::as_ref::test_as_ref_str_flex_str::<Arc<str>>("test");
}

// *** Serialize Tests ***
