#![cfg(all(
    feature = "serde",
    any(feature = "str", feature = "bytes", feature = "cstr", feature = "path")
))]

#[path = "common/deserialize.rs"]
mod deserialize;

use flexstr::{FlexStr, RefCounted};
use flexstr_support::StringToFromBytes;

fn storage<S: ?Sized + StringToFromBytes, R: RefCounted<S>>(
    value: &FlexStr<'_, S, R>,
    inline: bool,
) {
    assert_eq!(value.is_inlined(), inline);
    assert_eq!(value.is_ref_counted(), !inline);
}

#[cfg(feature = "str")]
#[test]
fn strings_deserialize_without_intermediate_allocations() {
    deserialize::text::<flexstr::LocalStr, str>(|s| s, Some(storage));
    deserialize::text::<flexstr::SharedStr, str>(|s| s, Some(storage));
}

#[cfg(feature = "bytes")]
#[test]
fn bytes_deserialize_without_intermediate_allocations() {
    deserialize::bytes::<flexstr::LocalBytes>(Some(storage));
    deserialize::bytes::<flexstr::SharedBytes>(Some(storage));
}

#[cfg(feature = "cstr")]
#[test]
fn cstr_deserializes_without_intermediate_allocations() {
    deserialize::cstr::<flexstr::LocalCStr>(Some(storage));
    deserialize::cstr::<flexstr::SharedCStr>(Some(storage));
}

#[cfg(feature = "path")]
#[test]
fn paths_deserialize_without_intermediate_allocations() {
    deserialize::text::<flexstr::LocalPath, std::path::Path>(std::path::Path::new, Some(storage));
    deserialize::text::<flexstr::SharedPath, std::path::Path>(std::path::Path::new, Some(storage));
}
