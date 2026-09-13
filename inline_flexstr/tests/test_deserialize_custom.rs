#![cfg(feature = "serde")]

use flexstr_support::StringToFromBytes;
use inline_flexstr::InlineFlexStr;
use serde::{Deserialize, Deserializer};

// A custom backing type whose representation is unrelated to built-in strings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Marker;

impl StringToFromBytes for Marker {
    fn bytes_as_self(bytes: &[u8]) -> &Self {
        assert!(bytes.is_empty());
        &Marker
    }

    fn self_as_raw_bytes(&self) -> &[u8] {
        &[]
    }
}

impl<'de> Deserialize<'de> for Marker {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <()>::deserialize(deserializer).map(|()| Marker)
    }
}

// Keep the original generic bounds usable by downstream generic functions.
fn deserialize<'de, S, D>(deserializer: D) -> Result<InlineFlexStr<S>, D::Error>
where
    S: ?Sized + StringToFromBytes,
    D: Deserializer<'de>,
    Box<S>: Deserialize<'de>,
{
    InlineFlexStr::deserialize(deserializer)
}

#[test]
fn deserialize_custom_backing_type() {
    let mut deserializer = serde_json::Deserializer::from_str("null");
    let value = deserialize::<Marker, _>(&mut deserializer).unwrap();

    assert_eq!(&*value, &Marker);
}
