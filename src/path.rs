#[cfg(not(feature = "std"))]
compile_error!("Path support is not available without the 'std' feature");

use alloc::{boxed::Box, rc::Rc, sync::Arc};

use std::path::{Path, PathBuf};

use crate::Flex;

pub type LocalPath<'s> = Flex<'s, Path, Rc<Path>>;
pub type SharedPath<'s> = Flex<'s, Path, Arc<Path>>;
pub type BoxPath<'s> = Flex<'s, Path, Box<Path>>;

pub type LocalPathBuf<'s> = Flex<'s, PathBuf, Rc<PathBuf>>;
pub type SharedPathBuf<'s> = Flex<'s, PathBuf, Arc<PathBuf>>;
pub type BoxPathBuf<'s> = Flex<'s, PathBuf, Box<PathBuf>>;
