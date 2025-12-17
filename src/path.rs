use alloc::{rc::Rc, sync::Arc};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use crate::{FlexStr, ImmutableBytes, InlineFlexStr, RefCounted, RefCountedMut, StringToFromBytes};

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

// *** From<PathBuf> ***

// NOTE: Cannot be implemented generically because of impl<T> From<T> for T
impl<'s, R: RefCounted<Path>> From<PathBuf> for FlexStr<'s, Path, R> {
    fn from(p: PathBuf) -> Self {
        FlexStr::from_owned(p)
    }
}

// *** TryFrom<&Path> for InlineFlexStr ***

// NOTE: Cannot be implemented generically because of impl<T, U> TryFrom<U> for T where U: Into<T>
impl<'s> TryFrom<&'s Path> for InlineFlexStr<Path> {
    type Error = &'s Path;

    #[inline]
    fn try_from(s: &'s Path) -> Result<Self, Self::Error> {
        InlineFlexStr::try_from_type(s)
    }
}

// NOTE: Cannot be implemented generically because CloneToUninit is needed
// as a bound to `S`, but is unstable.
impl RefCountedMut<Path> for Arc<Path> {
    #[inline]
    fn to_mut(&mut self) -> &mut Path {
        Arc::make_mut(self)
    }

    #[inline]
    fn as_mut(&mut self) -> &mut Path {
        // PANIC SAFETY: We only use this when we know the Arc is newly created
        Arc::get_mut(self).expect("Arc is shared")
    }
}

// NOTE: Cannot be implemented generically because CloneToUninit is needed
// as a bound to `S`, but is unstable.
impl RefCountedMut<Path> for Rc<Path> {
    #[inline]
    fn to_mut(&mut self) -> &mut Path {
        Rc::make_mut(self)
    }

    #[inline]
    fn as_mut(&mut self) -> &mut Path {
        // PANIC SAFETY: We only use this when we know the Rc is newly created
        Rc::get_mut(self).expect("Rc is shared")
    }
}
