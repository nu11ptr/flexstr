use crate::traits::{StringFromBytesMut, StringToFromBytes};

// *** StringToFromBytes ***

impl StringToFromBytes for [u8] {
    #[inline]
    fn bytes_as_self(bytes: &[u8]) -> &Self {
        bytes
    }

    #[inline]
    fn self_as_raw_bytes(&self) -> &[u8] {
        self
    }
}

// *** StringFromBytesMut ***

impl StringFromBytesMut for [u8] {
    #[inline]
    fn bytes_as_self_mut(bytes: &mut [u8]) -> &mut Self {
        bytes
    }
}
