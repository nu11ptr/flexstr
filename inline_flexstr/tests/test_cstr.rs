#![cfg(feature = "cstr")]

extern crate alloc;

#[cfg(feature = "serde")]
use inline_flexstr::InlineCStr;

use core::ffi::CStr;

mod common;

// *** Serialize/Deserialize Tests ***

#[cfg(feature = "serde")]
#[test]
fn serialize_deserialize_test_inline_cstr() {
    common::serialize::serialize_deserialize_test::<InlineCStr, CStr>(c"test");
}

// *** Basic Tests ***

#[test]
fn test_creation_from_inline_cstr() {
    common::basic::test_creation_from_inline::<CStr>(c"test");
}

#[test]
fn test_empty_cstr() {
    common::basic::test_empty::<CStr>(c"");
}

#[test]
fn test_accessors_cstr() {
    common::basic::test_accessors::<CStr>(c"test");
}

#[test]
fn test_clone_cstr() {
    common::basic::test_clone::<CStr>(c"test");
}

#[test]
fn test_default_cstr() {
    common::basic::test_default::<CStr>();
}

// *** Conversion Tests ***

#[test]
fn test_to_owned_type_cstr() {
    common::conversion::test_to_owned_type::<CStr>(c"test");
}

#[test]
fn test_into_owned_type_cstr() {
    common::conversion::test_into_owned_type::<CStr>(c"test");
}

// *** Comparison Tests ***

#[test]
fn test_partial_eq_cstr() {
    common::comparison::test_partial_eq::<CStr>(c"test", c"test");
    common::comparison::test_partial_eq::<CStr>(c"test", c"other");
}

#[test]
fn test_eq_cstr() {
    common::comparison::test_eq::<CStr>(c"test");
}

#[test]
fn test_comparison_with_ref_cstr() {
    common::comparison::test_comparison_with_ref::<CStr>(c"test");
}

#[test]
fn test_comparison_with_owned_cstr() {
    common::comparison::test_comparison_with_owned::<CStr>(c"test");
}

#[test]
fn test_partial_eq_with_owned_types_cstr() {
    common::comparison::test_partial_eq_with_owned_types::<CStr>(c"test");
}

// *** Edge Case Tests ***

#[test]
fn test_empty_string_cstr() {
    common::edge_cases::test_empty_string::<CStr>(c"");
}

#[test]
fn test_various_lengths_cstr() {
    common::edge_cases::test_various_lengths::<CStr>(c"test");
    common::edge_cases::test_various_lengths::<CStr>(c"");
}

#[test]
fn test_special_content_cstr() {
    common::edge_cases::test_special_content::<CStr>(c"test");
}

#[test]
fn test_clone_cstr_edge() {
    common::edge_cases::test_clone::<CStr>(c"test");
}

// *** Error Tests ***

#[test]
fn test_too_long_for_inlining() {
    common::errors::test_too_long_for_inlining();
}

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
    common::stringlike::test_as_c_str::<CStr>(c"test");
}

#[test]
fn test_into_c_string() {
    common::stringlike::test_into_c_string::<CStr>(c"test");
}

#[test]
fn test_to_c_string() {
    common::stringlike::test_to_c_string::<CStr>(c"test");
}

// *** TryFrom Tests ***

#[test]
fn test_try_from_str_cstr() {
    common::try_from::test_try_from_str_cstr();
}

#[test]
fn test_try_from_bytes_cstr() {
    common::try_from::test_try_from_bytes_cstr();
}

// *** FromStr Tests ***

#[test]
fn test_from_str_cstr_success() {
    common::from_str::test_from_str_cstr_success();
}

#[test]
fn test_from_str_cstr_error() {
    common::from_str::test_from_str_cstr_error();
}

// *** AsRef Tests ***

#[test]
fn test_as_ref_cstr() {
    common::as_ref::test_as_ref_cstr(c"test");
}

// *** CStr Specific Tests ***

#[test]
fn test_try_from_bytes_with_nul() {
    common::cstr_specific::test_try_from_bytes_with_nul();
}

#[test]
fn test_try_from_bytes_without_nul() {
    common::cstr_specific::test_try_from_bytes_without_nul();
}

#[test]
fn test_try_from_bytes_interior_nul() {
    common::cstr_specific::test_try_from_bytes_interior_nul();
}

#[test]
fn test_try_from_bytes_too_long() {
    common::cstr_specific::test_try_from_bytes_too_long();
}

#[test]
fn test_as_bytes_with_nul() {
    common::cstr_specific::test_as_bytes_with_nul(c"test");
}

