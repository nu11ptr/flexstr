use alloc::borrow::Cow;
use core::str::FromStr;
use std::ffi::{OsStr, OsString};
use std::path::Path;

use crate::inline::{InlineFlexStr, TooLongForInlining, inline_partial_eq_impl};

use flexstr_support::StringToFromBytes;

/// Inline `OsStr` type
pub type InlineOsStr = InlineFlexStr<OsStr>;

// *** TryFrom for InlineFlexStr ***

// NOTE: Cannot be implemented generically because of impl<T, U> TryFrom<U> for T where U: Into<T>
impl<'s> TryFrom<&'s OsStr> for InlineFlexStr<OsStr> {
    type Error = TooLongForInlining;

    #[inline]
    fn try_from(s: &'s OsStr) -> Result<Self, Self::Error> {
        InlineFlexStr::try_from_type(s)
    }
}

impl<'s> TryFrom<&'s str> for InlineFlexStr<OsStr> {
    type Error = TooLongForInlining;

    #[inline]
    fn try_from(s: &'s str) -> Result<Self, Self::Error> {
        InlineFlexStr::try_from_type(OsStr::new(s))
    }
}

impl<'s> TryFrom<&'s Path> for InlineFlexStr<OsStr> {
    type Error = TooLongForInlining;

    #[inline]
    fn try_from(p: &'s Path) -> Result<Self, Self::Error> {
        InlineFlexStr::try_from_type(p.as_os_str())
    }
}

// *** PartialEq ***

inline_partial_eq_impl!(OsStr, OsStr);
inline_partial_eq_impl!(&OsStr, OsStr);
inline_partial_eq_impl!(OsString, OsStr);
inline_partial_eq_impl!(Cow<'_, OsStr>, OsStr);

// *** AsRef ***

impl<S: ?Sized + StringToFromBytes> AsRef<OsStr> for InlineFlexStr<S>
where
    S: AsRef<OsStr>,
{
    fn as_ref(&self) -> &OsStr {
        self.as_ref_type().as_ref()
    }
}

// *** FromStr ***

impl FromStr for InlineFlexStr<OsStr> {
    type Err = TooLongForInlining;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        InlineFlexStr::try_from_type(OsStr::new(s))
    }
}
