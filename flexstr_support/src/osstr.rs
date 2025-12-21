use std::ffi::OsStr;

use crate::traits::StringToFromBytes;

// *** StringToFromBytes ***

impl StringToFromBytes for OsStr {
    #[cfg(all(
        feature = "safe",
        not(feature = "win_min_unsafe"),
        target_family = "windows"
    ))]
    #[inline]
    fn bytes_as_self(bytes: &[u8]) -> &Self {
        // TODO: With a 3rd party crate, we could use: os_str_bytes::OsStrBytes::assert_from_raw_bytes()
        // But is this any better? They likely use unsafe internally anyway (as of course the std library does as well).
        compile_error!(
            "OsStr/Path support is not available with the 'safe' feature on Windows. Using the 'win_min_unsafe' feature in combination with 'safe' will allow it to compile, but will use a single unsafe call."
        );
        unreachable!()
    }

    #[cfg(all(feature = "safe", target_family = "unix"))]
    #[inline]
    fn bytes_as_self(bytes: &[u8]) -> &Self {
        use std::os::unix::prelude::OsStrExt;

        OsStrExt::from_bytes(bytes)
    }

    #[cfg(any(
        not(feature = "safe"),
        all(feature = "win_min_unsafe", target_family = "windows")
    ))]
    #[inline]
    fn bytes_as_self(bytes: &[u8]) -> &Self {
        // SAFETY: We know the bytes are a valid OsStr
        unsafe { OsStr::from_encoded_bytes_unchecked(bytes) }
    }

    #[inline]
    fn self_as_raw_bytes(&self) -> &[u8] {
        self.as_encoded_bytes()
    }
}
