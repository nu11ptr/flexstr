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
        unsafe { SmallBox::new(ptr, len, cap) }
    }

    #[inline]
    fn from_boxed(boxed: &mut SmallBox<[u8]>) -> Self {
        unsafe { Vec::from_raw_parts(boxed.ptr() as *mut u8, boxed.len(), boxed.cap()) }
    }
}

#[cfg(feature = "safe")]
impl OwnedToFromBoxed<[u8]> for Vec<u8> {
    type BoxType = Option<Box<[u8]>>;

    fn into_boxed(self) -> Self::BoxType {
        Some(self.into_boxed_slice())
    }

    fn from_boxed(boxed: &mut Self::BoxType) -> Self {
        boxed
            .take()
            .expect("Expected a vector of bytes, but got None")
            .into_vec()
    }
}

impl OwnedOpsMut<[u8]> for Vec<u8> {
    fn push_str(&mut self, s: &[u8]) {
        self.extend_from_slice(s);
    }
}
