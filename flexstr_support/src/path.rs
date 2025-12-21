use std::{ffi::OsStr, path::Path};

use crate::traits::StringToFromBytes;

// *** StringToFromBytes ***

impl StringToFromBytes for Path {
    #[inline]
    fn bytes_as_self(bytes: &[u8]) -> &Self {
        Path::new(OsStr::bytes_as_self(bytes))
    }

    #[inline]
    fn self_as_raw_bytes(&self) -> &[u8] {
        OsStr::self_as_bytes(self.as_os_str())
    }
}
