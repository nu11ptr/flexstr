#![cfg(all(
    feature = "serde",
    any(feature = "str", feature = "bytes", feature = "cstr", feature = "path")
))]

#[path = "../../tests/common/deserialize.rs"]
mod deserialize;

#[cfg(feature = "str")]
#[test]
fn strings_deserialize_without_allocating() {
    deserialize::text::<inline_flexstr::InlineStr, str>(|s| s, None);
}

#[cfg(feature = "bytes")]
#[test]
fn bytes_deserialize_without_allocating() {
    deserialize::bytes::<inline_flexstr::InlineBytes>(None);
}

#[cfg(feature = "cstr")]
#[test]
fn cstr_deserializes_without_allocating() {
    deserialize::cstr::<inline_flexstr::InlineCStr>(None);
}

#[cfg(feature = "path")]
#[test]
fn paths_deserialize_without_allocating() {
    deserialize::text::<inline_flexstr::InlinePath, std::path::Path>(std::path::Path::new, None);
}
