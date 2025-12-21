use alloc::{borrow::Cow, rc::Rc, sync::Arc};
use core::{convert::Infallible, str::FromStr};
use std::ffi::{OsStr, OsString};
use std::path::Path;
#[cfg(feature = "path")]
use std::path::PathBuf;

use crate::flex::{
    FlexStr, ImmutableBytes, RefCounted, RefCountedMut, partial_eq_impl, ref_counted_mut_impl,
};

use flexstr_support::StringToFromBytes;

/// Local `OsStr` type (NOTE: This can't be shared between threads)
pub type LocalOsStr<'s> = FlexStr<'s, OsStr, Rc<OsStr>>;

/// Shared `OsStr` type
pub type SharedOsStr<'s> = FlexStr<'s, OsStr, Arc<OsStr>>;

const _: () = assert!(
    size_of::<Option<LocalOsStr>>() <= size_of::<OsString>(),
    "Option<LocalOsStr> must be less than or equal to the size of OsString"
);
const _: () = assert!(
    size_of::<Option<SharedOsStr>>() <= size_of::<OsString>(),
    "Option<SharedOsStr> must be less than or equal to the size of OsString"
);

// *** ImmutableBytes ***

impl ImmutableBytes for OsStr {}

// *** RefCountedMut ***

ref_counted_mut_impl!(OsStr);

// *** From for FlexStr ***

// NOTE: Cannot be implemented generically because of impl<T> From<T> for T
impl<'s, R: RefCounted<OsStr>> From<OsString> for FlexStr<'s, OsStr, R> {
    fn from(s: OsString) -> Self {
        FlexStr::from_owned(s)
    }
}

impl<'s, R: RefCounted<OsStr>> From<String> for FlexStr<'s, OsStr, R> {
    fn from(s: String) -> Self {
        FlexStr::from_owned(s.into())
    }
}

#[cfg(feature = "path")]
impl<'s, R: RefCounted<OsStr>> From<PathBuf> for FlexStr<'s, OsStr, R> {
    fn from(p: PathBuf) -> Self {
        FlexStr::from_owned(p.into())
    }
}

impl<'s, R: RefCounted<OsStr>> From<&'s str> for FlexStr<'s, OsStr, R> {
    fn from(s: &'s str) -> Self {
        FlexStr::from_borrowed(OsStr::new(s))
    }
}

impl<'s, R: RefCounted<OsStr>> From<&'s Path> for FlexStr<'s, OsStr, R> {
    fn from(p: &'s Path) -> Self {
        FlexStr::from_borrowed(p.as_os_str())
    }
}

// *** PartialEq ***

partial_eq_impl!(OsStr, OsStr);
partial_eq_impl!(&OsStr, OsStr);
partial_eq_impl!(OsString, OsStr);
partial_eq_impl!(Cow<'s, OsStr>, OsStr);

// *** AsRef ***

impl<'s, S: ?Sized + StringToFromBytes, R: RefCounted<S>> AsRef<OsStr> for FlexStr<'s, S, R>
where
    S: AsRef<OsStr>,
{
    fn as_ref(&self) -> &OsStr {
        self.as_ref_type().as_ref()
    }
}

// *** FromStr ***

impl<R: RefCounted<OsStr>> FromStr for FlexStr<'static, OsStr, R> {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(FlexStr::from_borrowed(OsStr::new(s)).into_owned())
    }
}
