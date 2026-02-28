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

/// Verify that `message::encode` (for flex types) and `string::encode` (for String)
/// produce identical wire format for the same string content. This is the foundation
/// of cross-type compatibility: if the bytes on the wire are identical, the two types
/// are interchangeable from a protocol perspective.
pub fn cross_type_wire_compat<F>(s: &'static str)
where
    F: Message + Default + TryFrom<&'static str> + fmt::Debug,
    <F as TryFrom<&'static str>>::Error: fmt::Debug,
{
    let flex_val: F = s.try_into().unwrap();
    let string_val = s.to_string();
    let tag = 1u32;

    // Encode the flex type as a message field
    let mut flex_buf = Vec::new();
    prost::encoding::message::encode(tag, &flex_val, &mut flex_buf);

    // Encode the String as a string field
    let mut string_buf = Vec::new();
    prost::encoding::string::encode(tag, &string_val, &mut string_buf);

    assert_eq!(
        flex_buf, string_buf,
        "message::encode(flex) and string::encode(String) should produce identical bytes"
    );

    // Also verify encoded_len matches
    let flex_len = prost::encoding::message::encoded_len(tag, &flex_val);
    let string_len = prost::encoding::string::encoded_len(tag, &string_val);
    assert_eq!(flex_len, string_len, "encoded_len should match");
}

/// Encode a flex type using `message::encode`, then decode as a String field using
/// `string::merge`. Simulates: SharedStr/InlineStr server → String client.
pub fn encode_flex_decode_string<F>(s: &'static str)
where
    F: Message + Default + TryFrom<&'static str> + fmt::Debug,
    <F as TryFrom<&'static str>>::Error: fmt::Debug,
{
    let flex_val: F = s.try_into().unwrap();
    let tag = 1u32;

    // Encode flex type as a message field
    let mut buf = Vec::new();
    prost::encoding::message::encode(tag, &flex_val, &mut buf);

    // Decode the field key, then decode as a String
    let mut cursor = &buf[..];
    let (decoded_tag, wire_type) = prost::encoding::decode_key(&mut cursor).unwrap();
    assert_eq!(decoded_tag, tag);

    let mut decoded = String::new();
    prost::encoding::string::merge(
        wire_type,
        &mut decoded,
        &mut cursor,
        prost::encoding::DecodeContext::default(),
    )
    .unwrap();

    assert_eq!(decoded, s, "String decoded from flex-encoded field should match");
}

/// Encode a String using `string::encode`, then decode as a flex type using
/// `merge_length_delimited`. Simulates: String client → SharedStr/InlineStr server.
pub fn encode_string_decode_flex<F>(s: &'static str)
where
    F: Message + Default + AsRef<str> + fmt::Debug,
{
    let string_val = s.to_string();
    let tag = 1u32;

    // Encode String as a string field
    let mut buf = Vec::new();
    prost::encoding::string::encode(tag, &string_val, &mut buf);

    // Read past the field key, then decode using our merge_length_delimited
    let mut cursor = &buf[..];
    let (decoded_tag, wire_type) = prost::encoding::decode_key(&mut cursor).unwrap();
    assert_eq!(decoded_tag, tag);
    assert_eq!(wire_type, prost::encoding::WireType::LengthDelimited);

    let mut decoded = F::default();
    decoded.merge_length_delimited(cursor).unwrap();

    assert_eq!(
        decoded.as_ref(),
        s,
        "flex type decoded from String-encoded field should match"
    );
}
