#![cfg(feature = "str")]

extern crate alloc;

#[cfg(feature = "serde")]
use inline_flexstr::InlineStr;

mod common;

// *** Serialize/Deserialize Tests ***

#[cfg(feature = "serde")]
#[test]
fn serialize_deserialize_test_inline_str() {
    common::serialize::serialize_deserialize_test::<InlineStr, str>("test");
}

// *** Basic Tests ***

#[test]
fn test_creation_from_inline_str() {
    common::basic::test_creation_from_inline::<str>("test");
}

#[test]
fn test_empty_str() {
    common::basic::test_empty::<str>("");
}

#[test]
fn test_accessors_str() {
    common::basic::test_accessors::<str>("test");
}

#[test]
fn test_clone_str() {
    common::basic::test_clone::<str>("test");
}

#[test]
fn test_default_str() {
    common::basic::test_default::<str>();
}

// *** Conversion Tests ***

#[test]
fn test_to_owned_type_str() {
    common::conversion::test_to_owned_type::<str>("test");
}

#[test]
fn test_into_owned_type_str() {
    common::conversion::test_into_owned_type::<str>("test");
}

// *** Comparison Tests ***

#[test]
fn test_partial_eq_str() {
    common::comparison::test_partial_eq::<str>("test", "test");
    common::comparison::test_partial_eq::<str>("test", "other");
}

#[test]
fn test_eq_str() {
    common::comparison::test_eq::<str>("test");
}

#[test]
fn test_partial_ord_str() {
    common::comparison::test_partial_ord::<str>("a", "b");
}

#[test]
fn test_ord_str() {
    common::comparison::test_ord::<str>("a", "b");
}

#[test]
fn test_hash_str() {
    common::comparison::test_hash::<str>("test");
}

#[test]
fn test_comparison_with_ref_str() {
    common::comparison::test_comparison_with_ref::<str>("test");
}

#[test]
fn test_comparison_with_owned_str() {
    common::comparison::test_comparison_with_owned::<str>("test");
}

#[test]
fn test_partial_eq_with_owned_types_str() {
    common::comparison::test_partial_eq_with_owned_types::<str>("test");
}

// *** Edge Case Tests ***

#[test]
fn test_empty_string_str() {
    common::edge_cases::test_empty_string::<str>("");
}

#[test]
fn test_capacity_boundary_exact_str() {
    // Create a string exactly at capacity
    let s = "a".repeat(inline_flexstr::INLINE_CAPACITY);
    let s_static: &'static str = Box::leak(s.into_boxed_str());
    common::edge_cases::test_capacity_boundary_exact::<str>(s_static);
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
    common::edge_cases::test_various_lengths::<str>("test");
    common::edge_cases::test_various_lengths::<str>("");
    common::edge_cases::test_various_lengths::<str>("a");
}

#[test]
fn test_special_content_str() {
    common::edge_cases::test_special_content::<str>("test");
    common::edge_cases::test_special_content::<str>("hello\nworld");
    common::edge_cases::test_special_content::<str>("🚀");
}

#[test]
fn test_clone_str_edge() {
    common::edge_cases::test_clone::<str>("test");
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
    common::stringlike::test_as_str::<str>("test");
}

#[test]
fn test_into_string() {
    common::stringlike::test_into_string::<str>("test");
}

#[test]
fn test_to_string() {
    common::stringlike::test_to_string::<str>("test");
}

// *** TryFrom Tests ***

#[cfg(feature = "std")]
#[test]
fn test_try_from_osstr_str() {
    common::try_from::test_try_from_osstr_str();
}

#[cfg(feature = "std")]
#[test]
fn test_try_from_path_str() {
    common::try_from::test_try_from_path_str();
}

#[test]
fn test_try_from_bytes_str() {
    common::try_from::test_try_from_bytes_str();
}

// *** FromStr Tests ***

#[test]
fn test_from_str_success() {
    common::from_str::test_from_str_success::<str>("test");
}

// *** Display Tests ***

#[test]
fn test_display_str() {
    common::display::test_display::<str>("test");
}

// *** Borrow Tests ***

#[test]
fn test_borrow_mut_str() {
    common::borrow::test_borrow_mut::<str>("test");
}

// *** Index Tests ***

#[test]
fn test_index_str() {
    common::index::test_index::<str>("test");
}

#[test]
fn test_index_mut_str() {
    common::index::test_index_mut::<str>("test");
}

// *** ToSocketAddrs Tests ***

#[cfg(feature = "std")]
#[test]
fn test_to_socket_addrs_str() {
    common::socket::test_to_socket_addrs::<str>("127.0.0.1:8080");
}

// *** AsRef Tests ***

#[test]
fn test_as_ref_str() {
    common::as_ref::test_as_ref_str("test");
}

// *** Serialize Tests ***

#[cfg(feature = "serde")]
#[test]
fn test_deserialize_error_str() {
    common::serialize::test_deserialize_error_str();
}

