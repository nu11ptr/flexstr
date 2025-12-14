#[cfg(not(feature = "std"))]
compile_error!("Path support is not available without the 'std' feature");

use alloc::{rc::Rc, sync::Arc};

use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use crate::{Flex, RefCounted, StringOps};

pub type LocalPath<'s> = Flex<'s, Path, Rc<Path>>;
pub type SharedPath<'s> = Flex<'s, Path, Arc<Path>>;

const _: () = assert!(
    size_of::<Option<LocalPath>>() <= size_of::<PathBuf>(),
    "Option<LocalPath> must be less than or equal to the size of PathBuf"
);
const _: () = assert!(
    size_of::<Option<SharedPath>>() <= size_of::<PathBuf>(),
    "Option<SharedPath> must be less than or equal to the size of PathBuf"
);

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

impl<R: RefCounted<Path>> Flex<'_, Path, R> {
    pub fn as_path(&self) -> &Path {
        self.as_borrowed_type()
    }

    pub fn as_os_str(&self) -> &OsStr {
        self.as_path().as_os_str()
    }
}

// *** From<PathBuf> ***

// NOTE: Cannot be implemented generically because of impl<T> From<T> for T
impl<'s, R: RefCounted<Path>> From<PathBuf> for Flex<'s, Path, R> {
    #[inline(always)]
    fn from(p: PathBuf) -> Self {
        Flex::from_owned(p)
    }
}

// *** AsRef<OsStr> ***

impl<R: RefCounted<Path>> AsRef<OsStr> for Flex<'_, Path, R> {
    fn as_ref(&self) -> &OsStr {
        self.as_os_str()
    }
}
