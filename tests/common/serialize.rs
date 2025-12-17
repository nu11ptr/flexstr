use core::fmt;
use flexstry::StringToFromBytes;
use serde::{Deserialize, Serialize};

pub fn serialize_deserialize_test<'s, F, S>(s: &'static S)
where
    F: TryFrom<&'s S> + PartialEq + Serialize + for<'de> Deserialize<'de> + fmt::Debug,
    <F as TryFrom<&'s S>>::Error: fmt::Debug,
    S: ?Sized + StringToFromBytes + Serialize + fmt::Debug + PartialEq,
    Box<S>: for<'de> Deserialize<'de>,
{
    let expected_str: F = s.try_into().unwrap();
    let serialized = serde_json::to_value(&expected_str).unwrap();
    let expected_json = serde_json::to_value(s).unwrap();
    assert_eq!(expected_json, serialized);

    let deserialized: F = serde_json::from_value(serialized).unwrap();
    assert_eq!(expected_str, deserialized);
}
