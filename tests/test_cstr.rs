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
