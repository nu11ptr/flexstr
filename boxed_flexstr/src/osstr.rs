use std::ffi::{OsStr, OsString};

#[cfg(not(feature = "safe"))]
use crate::small_box::SmallBox;
use crate::{
    BoxedFlexStr,
    boxed::{OwnedOpsMut, OwnedToFromBoxed},
};

#[cfg(feature = "safe")]
pub type BoxedOsStr = BoxedFlexStr<OsStr, Option<Box<OsStr>>>;

#[cfg(not(feature = "safe"))]
pub type BoxedOsStr = BoxedFlexStr<OsStr, SmallBox<OsStr>>;

#[cfg(not(feature = "large_strings"))]
const _: () = assert!(
    size_of::<BoxedOsStr>() <= size_of::<Box<OsStr>>(),
    "BoxedOsStr must be less than or equal to the size of Box<OsStr>"
);

const _: () = assert!(
    size_of::<Option<BoxedOsStr>>() <= size_of::<OsString>(),
    "Option<BoxedOsStr> must be less than or equal to the size of OsString"
);

#[cfg(not(feature = "safe"))]
impl OwnedToFromBoxed<OsStr> for OsString {
    type BoxType = SmallBox<OsStr>;

    #[inline]
    fn into_boxed(self) -> SmallBox<OsStr> {
        let bytes = self.into_encoded_bytes();
        let ptr = bytes.as_ptr();
        let len = bytes.len();
        let cap = bytes.capacity();
        core::mem::forget(bytes);
        // SAFETY: The raw parts are valid as they are not modified
        unsafe { SmallBox::new(ptr, len, cap) }
    }

    #[inline]
    fn from_boxed(boxed: &mut SmallBox<OsStr>) -> Self {
        // SAFETY: The raw parts are valid as they are not modified. This was previously an OsString.
        unsafe {
            let bytes = Vec::from_raw_parts(boxed.ptr() as *mut u8, boxed.len(), boxed.cap());
            OsString::from_encoded_bytes_unchecked(bytes)
        }
    }

    #[inline]
    fn clone_boxed(boxed: &SmallBox<OsStr>) -> SmallBox<OsStr> {
        // SAFETY: The raw parts are valid as they are not modified. This was previously an OsString.
        let s = unsafe {
            let bytes = Vec::from_raw_parts(boxed.ptr() as *mut u8, boxed.len(), boxed.cap());
            OsString::from_encoded_bytes_unchecked(bytes)
        };
        let s2 = s.clone();
        core::mem::forget(s);
        s2.into_boxed()
    }
}

#[cfg(feature = "safe")]
impl OwnedToFromBoxed<OsStr> for OsString {
    type BoxType = Option<Box<OsStr>>;

    #[inline]
    fn into_boxed(self) -> Self::BoxType {
        Some(self.into_boxed_os_str())
    }

    #[inline]
    fn from_boxed(boxed: &mut Self::BoxType) -> Self {
        boxed
            .take()
            .expect("Expected a string, but got None")
            .into_os_string()
    }

    #[inline]
    fn clone_boxed(boxed: &Option<Box<OsStr>>) -> Option<Box<OsStr>> {
        boxed.clone()
    }
}

impl OwnedOpsMut<OsStr> for OsString {
    #[inline]
    fn push_str(&mut self, s: &OsStr) {
        self.push(s);
    }
}
