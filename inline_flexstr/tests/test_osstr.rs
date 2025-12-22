#![cfg(all(feature = "std", feature = "osstr"))]

extern crate alloc;

#[cfg(feature = "serde")]
use inline_flexstr::InlineOsStr;

use std::ffi::OsStr;

mod common;

// *** Serialize/Deserialize Tests ***

#[cfg(feature = "serde")]
#[test]
fn serialize_deserialize_test_inline_osstr() {
    common::serialize::serialize_deserialize_test::<InlineOsStr, OsStr>(OsStr::new("test"));
}

// *** Basic Tests ***

#[test]
fn test_creation_from_inline_osstr() {
    common::basic::test_creation_from_inline::<OsStr>(OsStr::new("test"));
}

#[test]
fn test_empty_osstr() {
    common::basic::test_empty::<OsStr>(OsStr::new(""));
}

#[test]
fn test_accessors_osstr() {
    common::basic::test_accessors::<OsStr>(OsStr::new("test"));
}

#[test]
fn test_clone_osstr() {
    common::basic::test_clone::<OsStr>(OsStr::new("test"));
}

#[test]
fn test_default_osstr() {
    common::basic::test_default::<OsStr>();
}

// *** Conversion Tests ***

#[test]
fn test_to_owned_type_osstr() {
    common::conversion::test_to_owned_type::<OsStr>(OsStr::new("test"));
}

#[test]
fn test_into_owned_type_osstr() {
    common::conversion::test_into_owned_type::<OsStr>(OsStr::new("test"));
}

// *** Comparison Tests ***

#[test]
fn test_partial_eq_osstr() {
    common::comparison::test_partial_eq::<OsStr>(OsStr::new("test"), OsStr::new("test"));
    common::comparison::test_partial_eq::<OsStr>(OsStr::new("test"), OsStr::new("other"));
}

#[test]
fn test_eq_osstr() {
    common::comparison::test_eq::<OsStr>(OsStr::new("test"));
}

#[test]
fn test_comparison_with_ref_osstr() {
    common::comparison::test_comparison_with_ref::<OsStr>(OsStr::new("test"));
}

#[test]
fn test_comparison_with_owned_osstr() {
    common::comparison::test_comparison_with_owned::<OsStr>(OsStr::new("test"));
}

// *** Edge Case Tests ***

#[test]
fn test_empty_string_osstr() {
    common::edge_cases::test_empty_string::<OsStr>(OsStr::new(""));
}

#[test]
fn test_various_lengths_osstr() {
    common::edge_cases::test_various_lengths::<OsStr>(OsStr::new("test"));
    common::edge_cases::test_various_lengths::<OsStr>(OsStr::new(""));
}

#[test]
fn test_special_content_osstr() {
    common::edge_cases::test_special_content::<OsStr>(OsStr::new("test"));
}

#[test]
fn test_clone_osstr_edge() {
    common::edge_cases::test_clone::<OsStr>(OsStr::new("test"));
}

// *** Error Tests ***

#[test]
fn test_too_long_for_inlining() {
    common::errors::test_too_long_for_inlining();
}

// *** StringLike Tests ***

#[test]
fn test_as_os_str() {
    common::stringlike::test_as_os_str::<OsStr>(OsStr::new("test"));
}

#[test]
fn test_into_os_string() {
    common::stringlike::test_into_os_string::<OsStr>(OsStr::new("test"));
}

#[test]
fn test_to_os_string() {
    common::stringlike::test_to_os_string::<OsStr>(OsStr::new("test"));
}

// *** TryFrom Tests ***

#[test]
fn test_try_from_osstr_too_long() {
    common::try_from::test_try_from_osstr_too_long();
}

#[test]
fn test_try_from_str_osstr_too_long() {
    common::try_from::test_try_from_str_osstr_too_long();
}

// *** FromStr Tests ***

#[test]
fn test_from_str_osstr_success() {
    common::from_str::test_from_str_osstr_success();
}

#[test]
fn test_from_str_osstr_error() {
    common::from_str::test_from_str_osstr_error();
}

// *** AsRef Tests ***

#[test]
fn test_as_ref_osstr() {
    common::as_ref::test_as_ref_osstr(OsStr::new("test"));
}

