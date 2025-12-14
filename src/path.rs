#[cfg(not(feature = "std"))]
compile_error!("Path support is not available without the 'std' feature");

use alloc::{rc::Rc, sync::Arc};

use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use crate::{FlexStr, RefCounted, StringOps};

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

impl<R: RefCounted<Path>> FlexStr<'_, Path, R> {
    /// Borrow the Path as a `&Path`
    pub fn as_path(&self) -> &Path {
        self.as_borrowed_type()
    }

    /// Borrow the Path as an `&OsStr`
    pub fn as_os_str(&self) -> &OsStr {
        self.as_path().as_os_str()
    }
}

impl StringOps for Path {
    #[inline(always)]
    fn bytes_as_self(bytes: &[u8]) -> &Self {
        Path::new(OsStr::bytes_as_self(bytes))
    }

    #[inline(always)]
    fn self_as_bytes(&self) -> &[u8] {
        OsStr::self_as_bytes(self.as_os_str())
    }
}

// *** From<PathBuf> ***

// NOTE: Cannot be implemented generically because of impl<T> From<T> for T
impl<'s, R: RefCounted<Path>> From<PathBuf> for FlexStr<'s, Path, R> {
    #[inline(always)]
    fn from(p: PathBuf) -> Self {
        FlexStr::from_owned(p)
    }
}

// *** AsRef<OsStr>, and AsRef<[u8]> ***

// NOTE: Cannot be implemented generically because it conflicts with AsRef<S> for Bytes
impl<R: RefCounted<Path>> AsRef<[u8]> for FlexStr<'_, Path, R> {
    #[inline(always)]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<R: RefCounted<Path>> AsRef<OsStr> for FlexStr<'_, Path, R> {
    #[inline(always)]
    fn as_ref(&self) -> &OsStr {
        self.as_os_str()
    }
}
