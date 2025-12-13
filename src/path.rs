#[cfg(not(feature = "std"))]
compile_error!("Path support is not available without the 'std' feature");

use alloc::{rc::Rc, sync::Arc};

use std::path::Path;

use crate::Flex;

pub type LocalPath<'s> = Flex<'s, Path, Rc<Path>>;
pub type SharedPath<'s> = Flex<'s, Path, Arc<Path>>;

// impl<R: RefCounted<Path>> Flex<'_, Path, R> {
//     pub fn as_path(&self) -> &Path {
//         self.as_borrowed_type()
//     }

//     pub fn as_os_str(&self) -> &OsStr {
//         self.as_path().as_os_str()
//     }
// }
