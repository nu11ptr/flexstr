use alloc::fmt;
use core::ffi::CStr;

use crate::traits::StringToFromBytes;

// *** InteriorNulError ***

/// Error type returned when a C String has an interior NUL byte.
#[derive(Debug)]
pub struct InteriorNulError {
    /// The position of the interior NUL byte
    pub position: usize,
}

impl fmt::Display for InteriorNulError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Interior NUL byte found at position {}", self.position)
    }
}

impl core::error::Error for InteriorNulError {}

// *** StringToFromBytes ***

impl StringToFromBytes for CStr {
    #[cfg(feature = "safe")]
    #[inline]
    fn bytes_as_self(bytes: &[u8]) -> &Self {
        // PANIC SAFETY: We know the bytes are a valid CStr
        CStr::from_bytes_with_nul(bytes).expect("Missing NUL byte")
    }

    #[cfg(not(feature = "safe"))]
    #[inline]
    fn bytes_as_self(bytes: &[u8]) -> &Self {
        // SAFETY: We know the bytes are a valid CStr
        unsafe { CStr::from_bytes_with_nul_unchecked(bytes) }
    }

    #[inline]
    fn self_as_bytes(&self) -> &[u8] {
        self.to_bytes()
    }

    #[inline]
    fn self_as_raw_bytes(&self) -> &[u8] {
        self.to_bytes_with_nul()
    }
}
