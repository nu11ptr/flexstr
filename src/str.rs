use alloc::{rc::Rc, string::String, sync::Arc};

use crate::{FlexStr, InlineFlexStr, RefCounted, StringOps};

/// Local `str` type (NOTE: This can't be shared between threads)
pub type LocalStr<'s> = FlexStr<'s, str, Rc<str>>;

/// Shared `str` type
pub type SharedStr<'s> = FlexStr<'s, str, Arc<str>>;

/// Inline `str` type
pub type InlineStr = InlineFlexStr<str>;

const _: () = assert!(
    size_of::<Option<LocalStr>>() <= size_of::<String>(),
    "Option<LocalStr> must be less than or equal to the size of String"
);
const _: () = assert!(
    size_of::<Option<SharedStr>>() <= size_of::<String>(),
    "Option<SharedStr> must be less than or equal to the size of String"
);

impl StringOps for str {
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

// *** From<String> ***

// NOTE: Cannot be implemented generically because of impl<T> From<T> for T
impl<'s, R: RefCounted<str>> From<String> for FlexStr<'s, str, R> {
    fn from(s: String) -> Self {
        FlexStr::from_owned(s)
    }
}

// *** TryFrom<&str> for InlineFlexStr ***

// NOTE: Cannot be implemented generically because of impl<T, U> TryFrom<U> for T where U: Into<T>
impl<'s> TryFrom<&'s str> for InlineFlexStr<str> {
    type Error = &'s str;

    #[inline]
    fn try_from(s: &'s str) -> Result<Self, Self::Error> {
        InlineFlexStr::try_from_type(s)
    }
}
