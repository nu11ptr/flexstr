use alloc::{borrow::Cow, rc::Rc, sync::Arc};
use core::{convert::Infallible, str::FromStr};
#[cfg(feature = "serde")]
use core::{fmt, marker::PhantomData};
use std::{
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
};

use crate::flex::{
    FlexStr, ImmutableBytes, RefCounted, RefCountedMut, partial_eq_impl, ref_counted_mut_impl,
};

use flexstr_support::StringToFromBytes;

#[cfg(feature = "serde")]
use serde::de::{Error, Unexpected, Visitor};

/// Local `Path` type (NOTE: This can't be shared between threads)
pub type LocalPath = FlexStr<'static, Path, Rc<Path>>;

/// Shared `Path` type
pub type SharedPath = FlexStr<'static, Path, Arc<Path>>;

/// Local `Path` type that can optionally hold borrows (NOTE: This can't be shared between threads)
pub type LocalPathRef<'s> = FlexStr<'s, Path, Rc<Path>>;

/// Shared `Path` type that can optionally hold borrows
pub type SharedPathRef<'s> = FlexStr<'s, Path, Arc<Path>>;

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

// *** Deserialize ***

#[cfg(feature = "serde")]
pub(crate) struct PathVisitor<S: ?Sized, R>(pub(crate) PhantomData<(fn() -> S, R)>);

#[cfg(feature = "serde")]
impl<'de, S: ?Sized + StringToFromBytes, R: RefCounted<S>> Visitor<'de> for PathVisitor<S, R> {
    type Value = FlexStr<'static, S, R>;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a UTF-8 string")
    }

    fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
        // The type guard in FlexStr::deserialize limits S to Path; UTF-8 is valid on Windows too.
        Ok(FlexStr::from_borrowed(S::bytes_as_self(value.as_bytes())).into_owned())
    }

    fn visit_bytes<E: Error>(self, value: &[u8]) -> Result<Self::Value, E> {
        let value = core::str::from_utf8(value)
            .map_err(|_| E::invalid_value(Unexpected::Bytes(value), &self))?;
        self.visit_str(value)
    }
}
