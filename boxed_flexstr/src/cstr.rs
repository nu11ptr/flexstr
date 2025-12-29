use alloc::ffi::CString;
#[cfg(not(feature = "safe"))]
use alloc::vec::Vec;
use core::ffi::CStr;

#[cfg(not(feature = "safe"))]
use crate::small_box::SmallBox;
use crate::{BoxedFlexStr, boxed::OwnedToFromBoxed};

#[cfg(feature = "safe")]
pub type BoxedCStr = BoxedFlexStr<CStr, Option<Box<CStr>>>;

#[cfg(not(feature = "safe"))]
pub type BoxedCStr = BoxedFlexStr<CStr, SmallBox<CStr>>;

#[cfg(not(feature = "safe"))]
impl OwnedToFromBoxed<CStr> for CString {
    type BoxType = SmallBox<CStr>;

    #[inline]
    fn into_boxed(self) -> SmallBox<CStr> {
        let bytes = self.into_bytes_with_nul();
        let ptr = bytes.as_ptr();
        let len = bytes.len() - 1;
        let cap = bytes.len();
        core::mem::forget(bytes);
        // SAFETY: The raw parts are valid as they are not modified
        unsafe { SmallBox::new(ptr, len, cap - 1) }
    }

    #[inline]
    fn from_boxed(boxed: &mut SmallBox<CStr>) -> Self {
        // SAFETY: The raw parts are valid as they are not modified. This was previously a CString.
        unsafe {
            let bytes = Vec::from_raw_parts(boxed.ptr() as *mut u8, boxed.len(), boxed.cap());
            CString::from_vec_with_nul_unchecked(bytes)
        }
    }

    #[inline]
    fn clone_boxed(boxed: &SmallBox<CStr>) -> SmallBox<CStr> {
        // SAFETY: The raw parts are valid as they are not modified. This was previously a CString.
        let s = unsafe {
            let bytes = Vec::from_raw_parts(boxed.ptr() as *mut u8, boxed.len(), boxed.cap());
            CString::from_vec_with_nul_unchecked(bytes)
        };
        let s2 = s.clone();
        core::mem::forget(s);
        s2.into_boxed()
    }
}

#[cfg(feature = "safe")]
impl OwnedToFromBoxed<CStr> for CString {
    type BoxType = Option<Box<CStr>>;

    #[inline]
    fn into_boxed(self) -> Self::BoxType {
        Some(self.into_boxed_c_str())
    }

    #[inline]
    fn from_boxed(boxed: &mut Self::BoxType) -> Self {
        boxed
            .take()
            .expect("Expected a string, but got None")
            .into_c_string()
    }

    #[inline]
    fn clone_boxed(boxed: &Option<Box<CStr>>) -> Option<Box<CStr>> {
        boxed.clone()
    }
}
