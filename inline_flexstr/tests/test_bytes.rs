#![cfg(feature = "bytes")]

extern crate alloc;

#[cfg(feature = "serde")]
use inline_flexstr::InlineBytes;

mod common;

// *** Serialize/Deserialize Tests ***

#[cfg(feature = "serde")]
#[test]
fn serialize_deserialize_test_inline_bytes() {
    common::serialize::serialize_deserialize_test::<InlineBytes, [u8]>(b"test");
}

// *** Basic Tests ***

#[test]
fn test_creation_from_inline_bytes() {
    common::basic::test_creation_from_inline::<[u8]>(b"test");
}

#[test]
fn test_empty_bytes() {
    common::basic::test_empty::<[u8]>(b"");
}

#[test]
fn test_accessors_bytes() {
    common::basic::test_accessors::<[u8]>(b"test");
}

#[test]
fn test_clone_bytes() {
    common::basic::test_clone::<[u8]>(b"test");
}

#[test]
fn test_default_bytes() {
    common::basic::test_default::<[u8]>();
}

// *** Conversion Tests ***

#[test]
fn test_to_owned_type_bytes() {
    common::conversion::test_to_owned_type::<[u8]>(b"test");
}

#[test]
fn test_into_owned_type_bytes() {
    common::conversion::test_into_owned_type::<[u8]>(b"test");
}

// *** Comparison Tests ***

#[test]
fn test_partial_eq_bytes() {
    common::comparison::test_partial_eq::<[u8]>(b"test", b"test");
    common::comparison::test_partial_eq::<[u8]>(b"test", b"other");
}

#[test]
fn test_eq_bytes() {
    common::comparison::test_eq::<[u8]>(b"test");
}

#[test]
fn test_partial_ord_bytes() {
    common::comparison::test_partial_ord::<[u8]>(b"a", b"b");
}

#[test]
fn test_ord_bytes() {
    common::comparison::test_ord::<[u8]>(b"a", b"b");
}

#[test]
fn test_comparison_with_ref_bytes() {
    common::comparison::test_comparison_with_ref::<[u8]>(b"test");
}

#[test]
fn test_comparison_with_owned_bytes() {
    common::comparison::test_comparison_with_owned::<[u8]>(b"test");
}

// *** Edge Case Tests ***

#[test]
fn test_empty_string_bytes() {
    common::edge_cases::test_empty_string::<[u8]>(b"");
}

#[test]
fn test_various_lengths_bytes() {
    common::edge_cases::test_various_lengths::<[u8]>(b"test");
    common::edge_cases::test_various_lengths::<[u8]>(b"");
    common::edge_cases::test_various_lengths::<[u8]>(b"a");
}

#[test]
fn test_special_content_bytes() {
    common::edge_cases::test_special_content::<[u8]>(b"test");
}

#[test]
fn test_clone_bytes_edge() {
    common::edge_cases::test_clone::<[u8]>(b"test");
}

// *** Error Tests ***

#[test]
fn test_too_long_for_inlining() {
    common::errors::test_too_long_for_inlining();
}

// *** StringLike Tests ***

#[test]
fn test_into_vec_bytes() {
    common::stringlike::test_into_vec_bytes::<[u8]>(b"test");
}

#[test]
fn test_to_vec_bytes() {
    common::stringlike::test_to_vec_bytes::<[u8]>(b"test");
}

// *** TryFrom Tests ***

#[test]
fn test_try_from_bytes_too_long() {
    common::try_from::test_try_from_bytes_too_long();
}

#[test]
fn test_try_from_str_too_long() {
    common::try_from::test_try_from_str_too_long();
}

// *** FromStr Tests ***

#[test]
fn test_from_str_bytes_success() {
    common::from_str::test_from_str_bytes_success();
}

#[test]
fn test_from_str_bytes_error() {
    common::from_str::test_from_str_bytes_error();
}

// *** AsRef Tests ***

#[test]
fn test_as_ref_bytes() {
    common::as_ref::test_as_ref_bytes(b"test");
}

