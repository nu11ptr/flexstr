use alloc::{borrow::Cow, rc::Rc, sync::Arc};
use core::{convert::Infallible, str::FromStr};
use std::{
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
};

use crate::{
    FlexStr, ImmutableBytes, InlineFlexStr, RefCounted, RefCountedMut, StringToFromBytes,
    inline::{TooLongForInlining, inline_partial_eq_impl},
    partial_eq_impl, ref_counted_mut_impl,
};

/// Local `Path` type (NOTE: This can't be shared between threads)
pub type LocalPath<'s> = FlexStr<'s, Path, Rc<Path>>;

/// Shared `Path` type
pub type SharedPath<'s> = FlexStr<'s, Path, Arc<Path>>;

/// Inline `Path` type
pub type InlinePath = InlineFlexStr<Path>;

const _: () = assert!(
    size_of::<Option<LocalPath>>() <= size_of::<PathBuf>(),
    "Option<LocalPath> must be less than or equal to the size of PathBuf"
);
const _: () = assert!(
    size_of::<Option<SharedPath>>() <= size_of::<PathBuf>(),
    "Option<SharedPath> must be less than or equal to the size of PathBuf"
);

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

// *** ImmutableBytes ***

impl ImmutableBytes for Path {}

// *** RefCountedMut ***

ref_counted_mut_impl!(Path);

// *** From for FlexStr ***

// NOTE: Cannot be implemented generically because of impl<T> From<T> for T
impl<'s, R: RefCounted<Path>> From<PathBuf> for FlexStr<'s, Path, R> {
    fn from(p: PathBuf) -> Self {
        FlexStr::from_owned(p)
    }
}

impl<'s, R: RefCounted<Path>> From<String> for FlexStr<'s, Path, R> {
    fn from(s: String) -> Self {
        FlexStr::from_owned(s.into())
    }
}

impl<'s, R: RefCounted<Path>> From<OsString> for FlexStr<'s, Path, R> {
    fn from(s: OsString) -> Self {
        FlexStr::from_owned(s.into())
    }
}

impl<'s, R: RefCounted<Path>> From<&'s str> for FlexStr<'s, Path, R> {
    fn from(s: &'s str) -> Self {
        FlexStr::from_borrowed(Path::new(s))
    }
}

impl<'s, R: RefCounted<Path>> From<&'s OsStr> for FlexStr<'s, Path, R> {
    fn from(s: &'s OsStr) -> Self {
        FlexStr::from_borrowed(Path::new(s))
    }
}

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

partial_eq_impl!(Path, Path);
partial_eq_impl!(&Path, Path);
partial_eq_impl!(PathBuf, Path);
partial_eq_impl!(Cow<'s, Path>, Path);

inline_partial_eq_impl!(Path, Path);
inline_partial_eq_impl!(&Path, Path);
inline_partial_eq_impl!(PathBuf, Path);
inline_partial_eq_impl!(Cow<'_, Path>, Path);

// *** AsRef ***

impl<'s, S: ?Sized + StringToFromBytes, R: RefCounted<S>> AsRef<Path> for FlexStr<'s, S, R>
where
    S: AsRef<Path>,
{
    fn as_ref(&self) -> &Path {
        self.as_ref_type().as_ref()
    }
}

impl<S: ?Sized + StringToFromBytes> AsRef<Path> for InlineFlexStr<S>
where
    S: AsRef<Path>,
{
    fn as_ref(&self) -> &Path {
        self.as_ref_type().as_ref()
    }
}

// *** FromStr ***

impl<R: RefCounted<Path>> FromStr for FlexStr<'static, Path, R> {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(FlexStr::from_borrowed(Path::new(s)).into_owned())
    }
}

impl FromStr for InlineFlexStr<Path> {
    type Err = TooLongForInlining;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        InlineFlexStr::try_from_type(Path::new(s))
    }
}
