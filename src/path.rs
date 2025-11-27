#[cfg(not(feature = "std"))]
compile_error!("Path support is not available without the 'std' feature");

use alloc::{boxed::Box, rc::Rc, sync::Arc};
use core::ops::Deref;

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use crate::{Flex, osstr::bytes_as_os_str};

pub type LocalPath<'s> = Flex<'s, Path, Rc<Path>>;
pub type SharedPath<'s> = Flex<'s, Path, Arc<Path>>;
pub type BoxPath<'s> = Flex<'s, Path, Box<Path>>;

pub type LocalPathBuf<'s> = Flex<'s, PathBuf, Rc<PathBuf>>;
pub type SharedPathBuf<'s> = Flex<'s, PathBuf, Arc<PathBuf>>;
pub type BoxPathBuf<'s> = Flex<'s, PathBuf, Box<PathBuf>>;

impl<S: Deref<Target = Path>> Flex<'_, Path, S> {
    pub fn as_os_str(&self) -> &OsStr {
        match self {
            Flex::Borrowed(s) => s.as_os_str(),
            Flex::Inlined(inline) => bytes_as_os_str(inline.as_ref()),
            Flex::Stored(a) => a.as_os_str(),
        }
    }

    pub fn as_path(&self) -> &Path {
        match self {
            Flex::Borrowed(s) => s,
            Flex::Inlined(inline) => Path::new(bytes_as_os_str(inline.as_ref())),
            Flex::Stored(a) => &*a,
        }
    }
}
