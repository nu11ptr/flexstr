use std::path::{Path, PathBuf};

use crate::boxed::{BoxedFlexStr, OwnedOpsMut, OwnedToFromBoxed};
#[cfg(not(feature = "safe"))]
use crate::small_box::SmallBox;

pub type BoxedPath = BoxedFlexStr<Path>;

#[cfg(not(feature = "large_strings"))]
const _: () = assert!(
    size_of::<BoxedPath>() <= size_of::<Box<Path>>(),
    "BoxedPath must be less than or equal to the size of Box<Path>"
);

const _: () = assert!(
    size_of::<Option<BoxedPath>>() <= size_of::<PathBuf>(),
    "Option<BoxedPath> must be less than or equal to the size of PathBuf"
);

#[cfg(not(feature = "safe"))]
impl OwnedToFromBoxed<Path> for PathBuf {
    type BoxType = SmallBox<Path>;

    #[inline]
    fn into_boxed(self) -> SmallBox<Path> {
        let bytes = self.into_os_string().into_encoded_bytes();
        let ptr = bytes.as_ptr();
        let len = bytes.len();
        let cap = bytes.capacity();
        core::mem::forget(bytes);
        // SAFETY: The raw parts are valid as they are not modified
        unsafe { SmallBox::new(ptr, len, cap) }
    }

    #[inline]
    fn from_boxed(boxed: &mut SmallBox<Path>) -> Self {
        // SAFETY: The raw parts are valid as they are not modified. This was previously an OsString.
        unsafe {
            let bytes = Vec::from_raw_parts(boxed.ptr() as *mut u8, boxed.len(), boxed.cap());
            std::ffi::OsString::from_encoded_bytes_unchecked(bytes).into()
        }
    }

    #[inline]
    fn clone_boxed(boxed: &SmallBox<Path>) -> SmallBox<Path> {
        // SAFETY: The raw parts are valid as they are not modified. This was previously an OsString.
        let s: PathBuf = unsafe {
            let bytes = Vec::from_raw_parts(boxed.ptr() as *mut u8, boxed.len(), boxed.cap());
            std::ffi::OsString::from_encoded_bytes_unchecked(bytes).into()
        };
        let s2 = s.clone();
        core::mem::forget(s);
        s2.into_boxed()
    }
}

#[cfg(feature = "safe")]
impl OwnedToFromBoxed<Path> for PathBuf {
    type BoxType = Option<Box<Path>>;

    #[inline]
    fn into_boxed(self) -> Self::BoxType {
        Some(self.into_boxed_path())
    }

    #[inline]
    fn from_boxed(boxed: &mut Self::BoxType) -> Self {
        boxed
            .take()
            .expect("Expected a string, but got None")
            .into_path_buf()
    }

    #[inline]
    fn clone_boxed(boxed: &Self::BoxType) -> Self::BoxType {
        boxed.clone()
    }
}

impl OwnedOpsMut<Path> for PathBuf {
    #[inline]
    fn push_str(&mut self, s: &Path) {
        self.push(s);
    }
}
