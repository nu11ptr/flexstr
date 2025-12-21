use alloc::{borrow::Cow, rc::Rc, sync::Arc};
use core::{convert::Infallible, str::FromStr};
use std::{
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
};

use crate::flex::{
    FlexStr, ImmutableBytes, RefCounted, RefCountedMut, partial_eq_impl, ref_counted_mut_impl,
};

use flexstr_support::StringToFromBytes;

/// Local `Path` type (NOTE: This can't be shared between threads)
pub type LocalPath<'s> = FlexStr<'s, Path, Rc<Path>>;

/// Shared `Path` type
pub type SharedPath<'s> = FlexStr<'s, Path, Arc<Path>>;

const _: () = assert!(
    size_of::<Option<LocalPath>>() <= size_of::<PathBuf>(),
    "Option<LocalPath> must be less than or equal to the size of PathBuf"
);
const _: () = assert!(
    size_of::<Option<SharedPath>>() <= size_of::<PathBuf>(),
    "Option<SharedPath> must be less than or equal to the size of PathBuf"
);

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

// *** PartialEq ***

partial_eq_impl!(Path, Path);
partial_eq_impl!(&Path, Path);
partial_eq_impl!(PathBuf, Path);
partial_eq_impl!(Cow<'s, Path>, Path);

// *** AsRef ***

impl<'s, S: ?Sized + StringToFromBytes, R: RefCounted<S>> AsRef<Path> for FlexStr<'s, S, R>
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
