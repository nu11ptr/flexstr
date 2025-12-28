#[cfg(not(feature = "safe"))]
use crate::small_box::SmallBox;
use crate::{
    BoxedFlexStr,
    boxed::{OwnedOpsMut, OwnedToFromBoxed},
};

#[cfg(feature = "safe")]
pub type BoxedStr = BoxedFlexStr<str, Option<Box<str>>>;

#[cfg(not(feature = "safe"))]
pub type BoxedStr = BoxedFlexStr<str, SmallBox<str>>;

#[cfg(not(feature = "safe"))]
impl OwnedToFromBoxed<str> for String {
    type BoxType = SmallBox<str>;

    #[inline]
    fn into_boxed(self) -> SmallBox<str> {
        let ptr = self.as_str().as_ptr();
        let len = self.len();
        let cap = self.capacity();
        core::mem::forget(self);
        unsafe { SmallBox::new(ptr, len, cap) }
    }

    #[inline]
    fn from_boxed(boxed: &mut SmallBox<str>) -> Self {
        unsafe { String::from_raw_parts(boxed.ptr() as *mut u8, boxed.len(), boxed.cap()) }
    }
}

#[cfg(feature = "safe")]
impl OwnedToFromBoxed<str> for String {
    type BoxType = Option<Box<str>>;

    fn into_boxed(self) -> Self::BoxType {
        Some(self.into_boxed_str())
    }

    fn from_boxed(boxed: &mut Self::BoxType) -> Self {
        boxed
            .take()
            .expect("Expected a string, but got None")
            .into_string()
    }
}

impl OwnedOpsMut<str> for String {
    fn push_str(&mut self, s: &str) {
        self.push_str(s);
    }
}
