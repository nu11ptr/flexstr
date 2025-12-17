use alloc::{rc::Rc, sync::Arc};
use std::ffi::{OsStr, OsString};

use crate::{FlexStr, InlineFlexStr, RefCounted, StringOps};

/// Local `OsStr` type (NOTE: This can't be shared between threads)
pub type LocalOsStr<'s> = FlexStr<'s, OsStr, Rc<OsStr>>;

/// Shared `OsStr` type
pub type SharedOsStr<'s> = FlexStr<'s, OsStr, Arc<OsStr>>;

/// Inline `OsStr` type
pub type InlineOsStr = InlineFlexStr<OsStr>;

const _: () = assert!(
    size_of::<Option<LocalOsStr>>() <= size_of::<OsString>(),
    "Option<LocalOsStr> must be less than or equal to the size of OsString"
);
const _: () = assert!(
    size_of::<Option<SharedOsStr>>() <= size_of::<OsString>(),
    "Option<SharedOsStr> must be less than or equal to the size of OsString"
);

impl StringOps for OsStr {
    #[cfg(all(feature = "safe", target_family = "windows"))]
    #[inline]
    fn bytes_as_self(bytes: &[u8]) -> &Self {
        // TODO: With a 3rd party crate, we could use: os_str_bytes::OsStrBytes::assert_from_raw_bytes()
        // But is this any better? They likely use unsafe internally anyway.
        compile_error!("OsStr support is not available with the 'safe' feature on Windows");
        unreachable!()
    }

    #[cfg(all(feature = "safe", target_family = "unix"))]
    #[inline]
    fn bytes_as_self(bytes: &[u8]) -> &Self {
        use std::os::unix::prelude::OsStrExt;

        OsStrExt::from_bytes(bytes)
    }

    #[cfg(not(feature = "safe"))]
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

// *** From<OsString> ***

// NOTE: Cannot be implemented generically because of impl<T> From<T> for T
impl<'s, R: RefCounted<OsStr>> From<OsString> for FlexStr<'s, OsStr, R> {
    fn from(s: OsString) -> Self {
        FlexStr::from_owned(s)
    }
}

// *** TryFrom<&OsStr> for InlineFlexStr ***

// NOTE: Cannot be implemented generically because of impl<T, U> TryFrom<U> for T where U: Into<T>
impl<'s> TryFrom<&'s OsStr> for InlineFlexStr<OsStr> {
    type Error = &'s OsStr;

    #[inline]
    fn try_from(s: &'s OsStr) -> Result<Self, Self::Error> {
        InlineFlexStr::try_from_type(s)
    }
}
