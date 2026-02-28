#![allow(dead_code)]

use core::fmt;

use prost::Message;

/// Test that encoding and decoding round-trips correctly.
pub fn encode_decode_round_trip<F>(s: &'static str)
where
    F: Message + Default + TryFrom<&'static str> + PartialEq + fmt::Debug,
    <F as TryFrom<&'static str>>::Error: fmt::Debug,
{
    let original: F = s.try_into().unwrap();
    let encoded = original.encode_to_vec();
    let decoded = F::decode(&encoded[..]).unwrap();
    assert_eq!(original, decoded);
}

/// Test that length-delimited encoding and decoding round-trips correctly.
pub fn encode_length_delimited_round_trip<F>(s: &'static str)
where
    F: Message + Default + TryFrom<&'static str> + PartialEq + fmt::Debug,
    <F as TryFrom<&'static str>>::Error: fmt::Debug,
{
    let original: F = s.try_into().unwrap();
    let encoded = original.encode_length_delimited_to_vec();
    let decoded = F::decode_length_delimited(&encoded[..]).unwrap();
    assert_eq!(original, decoded);
}

/// Test that encoded bytes are just raw UTF-8 (no field tags or wrapping).
pub fn verify_wire_format<F>(s: &'static str)
where
    F: Message + Default + TryFrom<&'static str> + fmt::Debug,
    <F as TryFrom<&'static str>>::Error: fmt::Debug,
{
    let original: F = s.try_into().unwrap();
    let encoded = original.encode_to_vec();
    assert_eq!(encoded, s.as_bytes(), "encoded bytes should be raw UTF-8");
    assert_eq!(
        original.encoded_len(),
        s.len(),
        "encoded_len should match byte length"
    );
}

/// Test that decoding an empty buffer produces the default (empty string).
pub fn decode_empty<F>()
where
    F: Message + Default + AsRef<str> + fmt::Debug,
{
    let decoded = F::decode(&[][..]).unwrap();
    assert_eq!(
        decoded.as_ref(),
        "",
        "decoding empty buffer should give empty string"
    );
}

/// Test that clear resets to the default (empty string).
pub fn clear_test<F>(s: &'static str)
where
    F: Message + Default + TryFrom<&'static str> + AsRef<str> + fmt::Debug,
    <F as TryFrom<&'static str>>::Error: fmt::Debug,
{
    let mut value: F = s.try_into().unwrap();
    value.clear();
    assert_eq!(value.as_ref(), "", "clear should reset to empty string");
}
