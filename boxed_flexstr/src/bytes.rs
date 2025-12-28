#[cfg(not(feature = "safe"))]
use crate::small_box::SmallBox;
use crate::{
    BoxedFlexStr,
    boxed::{OwnedOpsMut, OwnedToFromBoxed},
};

#[cfg(feature = "safe")]
pub type BoxedBytes = BoxedFlexStr<[u8], Option<Box<[u8]>>>;

#[cfg(not(feature = "safe"))]
pub type BoxedBytes = BoxedFlexStr<[u8], SmallBox<[u8]>>;

#[cfg(not(feature = "safe"))]
impl OwnedToFromBoxed<[u8]> for Vec<u8> {
    type BoxType = SmallBox<[u8]>;

    #[inline]
    fn into_boxed(self) -> SmallBox<[u8]> {
        let ptr = self.as_ptr();
        let len = self.len();
        let cap = self.capacity();
        core::mem::forget(self);
        // SAFETY: The raw parts are valid as they are not modified
        unsafe { SmallBox::new(ptr, len, cap) }
    }

    #[inline]
    fn from_boxed(boxed: &mut SmallBox<[u8]>) -> Self {
        // SAFETY: The raw parts are valid as they are not modified
        unsafe { Vec::from_raw_parts(boxed.ptr() as *mut u8, boxed.len(), boxed.cap()) }
    }

    #[inline]
    fn clone_boxed(boxed: &SmallBox<[u8]>) -> SmallBox<[u8]> {
        // SAFETY: The raw parts are valid as they are not modified
        let v = unsafe { Vec::from_raw_parts(boxed.ptr() as *mut u8, boxed.len(), boxed.cap()) };
        let v2 = v.clone();
        core::mem::forget(v);
        v2.into_boxed()
    }
}

#[cfg(feature = "safe")]
impl OwnedToFromBoxed<[u8]> for Vec<u8> {
    type BoxType = Option<Box<[u8]>>;

    #[inline]
    fn into_boxed(self) -> Self::BoxType {
        Some(self.into_boxed_slice())
    }

    #[inline]
    fn from_boxed(boxed: &mut Self::BoxType) -> Self {
        boxed
            .take()
            .expect("Expected a vector of bytes, but got None")
            .into_vec()
    }

    #[inline]
    fn clone_boxed(boxed: &Option<Box<[u8]>>) -> Option<Box<[u8]>> {
        boxed.clone()
    }
}

impl OwnedOpsMut<[u8]> for Vec<u8> {
    #[inline]
    fn push_str(&mut self, s: &[u8]) {
        self.extend_from_slice(s);
    }
}
