use alloc::{boxed::Box, rc::Rc, string::String, sync::Arc};
use core::ops::Deref;

use crate::Flex;

pub type LocalStr<'s> = Flex<'s, str, Rc<str>>;
pub type SharedStr<'s> = Flex<'s, str, Arc<str>>;
pub type BoxStr<'s> = Flex<'s, str, Box<str>>;

pub type LocalString<'s> = Flex<'s, String, Rc<String>>;
pub type SharedString<'s> = Flex<'s, String, Arc<String>>;
pub type BoxString<'s> = Flex<'s, String, Box<String>>;

impl<S: Deref<Target = str>> Flex<'_, str, S> {
    pub fn as_str(&self) -> &str {
        match self {
            Flex::Borrowed(s) => s,
            Flex::Inlined(inline) => bytes_as_str(inline.as_ref()),
            Flex::Stored(a) => &*a,
        }
    }
}

#[cfg(feature = "safe")]
#[inline(always)]
fn bytes_as_str(bytes: &[u8]) -> &str {
    // PANIC SAFETY: We know the bytes are valid UTF-8
    str::from_utf8(bytes).expect("Invalid UTF-8")
}

#[cfg(not(feature = "safe"))]
#[inline(always)]
fn bytes_as_str(bytes: &[u8]) -> &str {
    // SAFETY: We know the bytes are valid UTF-8
    unsafe { str::from_utf8_unchecked(bytes) }
}
