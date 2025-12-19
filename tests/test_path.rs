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
