use alloc::borrow::Cow;
use core::str::FromStr;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use crate::inline::{InlineFlexStr, TooLongForInlining, inline_partial_eq_impl};

use flexstr_support::StringToFromBytes;

/// Inline `Path` type
pub type InlinePath = InlineFlexStr<Path>;

// *** TryFrom for InlineFlexStr ***

// NOTE: Cannot be implemented generically because of impl<T, U> TryFrom<U> for T where U: Into<T>
impl<'s> TryFrom<&'s Path> for InlineFlexStr<Path> {
    type Error = TooLongForInlining;

    #[inline]
    fn try_from(s: &'s Path) -> Result<Self, Self::Error> {
        InlineFlexStr::try_from_type(s)
    }
}

impl<'s> TryFrom<&'s str> for InlineFlexStr<Path> {
    type Error = TooLongForInlining;

    #[inline]
    fn try_from(s: &'s str) -> Result<Self, Self::Error> {
        InlineFlexStr::try_from_type(Path::new(s))
    }
}

impl<'s> TryFrom<&'s OsStr> for InlineFlexStr<Path> {
    type Error = TooLongForInlining;

    #[inline]
    fn try_from(s: &'s OsStr) -> Result<Self, Self::Error> {
        InlineFlexStr::try_from_type(Path::new(s))
    }
}

// *** PartialEq ***

inline_partial_eq_impl!(Path, Path);
inline_partial_eq_impl!(&Path, Path);
inline_partial_eq_impl!(PathBuf, Path);
inline_partial_eq_impl!(Cow<'_, Path>, Path);

// *** AsRef ***

impl<S: ?Sized + StringToFromBytes> AsRef<Path> for InlineFlexStr<S>
where
    S: AsRef<Path>,
{
    fn as_ref(&self) -> &Path {
        self.as_ref_type().as_ref()
    }
}

// *** FromStr ***

impl FromStr for InlineFlexStr<Path> {
    type Err = TooLongForInlining;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        InlineFlexStr::try_from_type(Path::new(s))
    }
}
