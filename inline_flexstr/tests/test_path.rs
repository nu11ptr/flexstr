#![cfg(all(feature = "std", feature = "path"))]

extern crate alloc;

#[cfg(feature = "serde")]
use inline_flexstr::InlinePath;

use std::path::Path;

mod common;

// *** Serialize/Deserialize Tests ***

#[cfg(feature = "serde")]
#[test]
fn serialize_deserialize_test_inline_path() {
    common::serialize::serialize_deserialize_test::<InlinePath, Path>(Path::new("test"));
}

// *** Basic Tests ***

#[test]
fn test_creation_from_inline_path() {
    common::basic::test_creation_from_inline::<Path>(Path::new("test"));
}

#[test]
fn test_empty_path() {
    common::basic::test_empty::<Path>(Path::new(""));
}

#[test]
fn test_accessors_path() {
    common::basic::test_accessors::<Path>(Path::new("test"));
}

#[test]
fn test_clone_path() {
    common::basic::test_clone::<Path>(Path::new("test"));
}

// Path doesn't implement Default, so skip this test
// #[test]
// fn test_default_path() {
//     common::basic::test_default::<Path>();
// }

// *** Conversion Tests ***

#[test]
fn test_to_owned_type_path() {
    common::conversion::test_to_owned_type::<Path>(Path::new("test"));
}

#[test]
fn test_into_owned_type_path() {
    common::conversion::test_into_owned_type::<Path>(Path::new("test"));
}

// *** Comparison Tests ***

#[test]
fn test_partial_eq_path() {
    common::comparison::test_partial_eq::<Path>(Path::new("test"), Path::new("test"));
    common::comparison::test_partial_eq::<Path>(Path::new("test"), Path::new("other"));
}

#[test]
fn test_eq_path() {
    common::comparison::test_eq::<Path>(Path::new("test"));
}

#[test]
fn test_comparison_with_ref_path() {
    common::comparison::test_comparison_with_ref::<Path>(Path::new("test"));
}

#[test]
fn test_comparison_with_owned_path() {
    common::comparison::test_comparison_with_owned::<Path>(Path::new("test"));
}

// *** Edge Case Tests ***

#[test]
fn test_empty_string_path() {
    common::edge_cases::test_empty_string::<Path>(Path::new(""));
}

#[test]
fn test_various_lengths_path() {
    common::edge_cases::test_various_lengths::<Path>(Path::new("test"));
    common::edge_cases::test_various_lengths::<Path>(Path::new(""));
}

#[test]
fn test_special_content_path() {
    common::edge_cases::test_special_content::<Path>(Path::new("test"));
}

#[test]
fn test_clone_path_edge() {
    common::edge_cases::test_clone::<Path>(Path::new("test"));
}

// *** Error Tests ***

#[test]
fn test_too_long_for_inlining() {
    common::errors::test_too_long_for_inlining();
}

// *** StringLike Tests ***

#[test]
fn test_as_path() {
    common::stringlike::test_as_path::<Path>(Path::new("test"));
}

#[test]
fn test_into_path_buf() {
    common::stringlike::test_into_path_buf::<Path>(Path::new("test"));
}

#[test]
fn test_to_path_buf() {
    common::stringlike::test_to_path_buf::<Path>(Path::new("test"));
}

// *** TryFrom Tests ***

#[test]
fn test_try_from_path_too_long() {
    common::try_from::test_try_from_path_too_long();
}

#[test]
fn test_try_from_str_path_too_long() {
    common::try_from::test_try_from_str_path_too_long();
}

#[test]
fn test_try_from_osstr_path_too_long() {
    common::try_from::test_try_from_osstr_path_too_long();
}

// *** FromStr Tests ***

#[test]
fn test_from_str_path_success() {
    common::from_str::test_from_str_path_success();
}

#[test]
fn test_from_str_path_error() {
    common::from_str::test_from_str_path_error();
}

// *** AsRef Tests ***

#[test]
fn test_as_ref_path() {
    common::as_ref::test_as_ref_path(Path::new("test"));
}

