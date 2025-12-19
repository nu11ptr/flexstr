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
