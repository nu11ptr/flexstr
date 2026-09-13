#![cfg(feature = "serde")]

use std::ops::Deref;

use flexstr::{FlexStr, RefCounted};
use flexstr_support::StringToFromBytes;
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

#[derive(Clone)]
struct MarkerRef(Marker);

impl Deref for MarkerRef {
    type Target = Marker;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&Marker> for MarkerRef {
    fn from(value: &Marker) -> Self {
        Self(*value)
    }
}

// Keep the original generic bounds usable by downstream generic functions.
fn deserialize<'de, S, R, D>(deserializer: D) -> Result<FlexStr<'static, S, R>, D::Error>
where
    S: ?Sized + StringToFromBytes,
    R: RefCounted<S>,
    D: Deserializer<'de>,
    Box<S>: Deserialize<'de>,
{
    FlexStr::deserialize(deserializer)
}

#[test]
fn deserialize_custom_backing_type() {
    let mut deserializer = serde_json::Deserializer::from_str("null");
    let value = deserialize::<Marker, MarkerRef, _>(&mut deserializer).unwrap();

    assert!(value.is_inlined());
    assert_eq!(&*value, &Marker);
}
