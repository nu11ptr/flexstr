use crate::traits::{StringFromBytesMut, StringToFromBytes};

// *** StringToFromBytes ***

impl StringToFromBytes for str {
    #[cfg(feature = "safe")]
    #[inline]
    fn bytes_as_self(bytes: &[u8]) -> &Self {
        // PANIC SAFETY: We know the bytes are valid UTF-8
        str::from_utf8(bytes).expect("Invalid UTF-8")
    }

    #[cfg(not(feature = "safe"))]
    #[inline]
    fn bytes_as_self(bytes: &[u8]) -> &Self {
        // SAFETY: We know the bytes are valid UTF-8
        unsafe { str::from_utf8_unchecked(bytes) }
    }

    #[inline]
    fn self_as_raw_bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}

// *** StringFromBytesMut ***

impl StringFromBytesMut for str {
    #[cfg(feature = "safe")]
    #[inline]
    fn bytes_as_self_mut(bytes: &mut [u8]) -> &mut Self {
        // PANIC SAFETY: We know the bytes are valid UTF-8
        str::from_utf8_mut(bytes).expect("Invalid UTF-8")
    }

    #[cfg(not(feature = "safe"))]
    #[inline]
    fn bytes_as_self_mut(bytes: &mut [u8]) -> &mut Self {
        // SAFETY: We know the bytes are valid UTF-8
        unsafe { str::from_utf8_unchecked_mut(bytes) }
    }
}
