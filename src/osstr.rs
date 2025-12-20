use alloc::{borrow::Cow, rc::Rc, sync::Arc};
use core::{convert::Infallible, str::FromStr};
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

use crate::{
    FlexStr, ImmutableBytes, InlineFlexStr, RefCounted, RefCountedMut, StringToFromBytes,
    inline::{TooLongForInlining, inline_partial_eq_impl},
    partial_eq_impl, ref_counted_mut_impl,
};

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

// *** StringToFromBytes ***

impl StringToFromBytes for OsStr {
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

partial_eq_impl!(OsStr, OsStr);
partial_eq_impl!(&OsStr, OsStr);
partial_eq_impl!(OsString, OsStr);
partial_eq_impl!(Cow<'s, OsStr>, OsStr);

inline_partial_eq_impl!(OsStr, OsStr);
inline_partial_eq_impl!(&OsStr, OsStr);
inline_partial_eq_impl!(OsString, OsStr);
inline_partial_eq_impl!(Cow<'_, OsStr>, OsStr);

// *** AsRef ***

impl<'s, S: ?Sized + StringToFromBytes, R: RefCounted<S>> AsRef<OsStr> for FlexStr<'s, S, R>
where
    S: AsRef<OsStr>,
{
    fn as_ref(&self) -> &OsStr {
        self.as_ref_type().as_ref()
    }
}

impl<S: ?Sized + StringToFromBytes> AsRef<OsStr> for InlineFlexStr<S>
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

impl FromStr for InlineFlexStr<OsStr> {
    type Err = TooLongForInlining;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        InlineFlexStr::try_from_type(OsStr::new(s))
    }
}
