use alloc::{rc::Rc, sync::Arc};

use crate::{Flex, RefCounted, StringOps};

pub type LocalBytes<'s> = Flex<'s, [u8], Rc<[u8]>>;
pub type SharedBytes<'s> = Flex<'s, [u8], Arc<[u8]>>;

const _: () = assert!(
    size_of::<Option<LocalBytes>>() <= size_of::<Vec<u8>>(),
    "Option<LocalBytes> must be less than or equal to the size of Vec<u8>"
);
const _: () = assert!(
    size_of::<Option<SharedBytes>>() <= size_of::<Vec<u8>>(),
    "Option<SharedBytes> must be less than or equal to the size of Vec<u8>"
);

impl StringOps for [u8] {
    #[inline(always)]
    fn bytes_as_self(bytes: &[u8]) -> &Self {
        bytes
    }

    #[inline(always)]
    fn self_as_bytes(&self) -> &[u8] {
        self
    }
}

// *** From<Vec<u8>> ***

// NOTE: Cannot be implemented generically because of impl<T> From<T> for T
impl<'s, R: RefCounted<[u8]>> From<Vec<u8>> for Flex<'s, [u8], R> {
    #[inline(always)]
    fn from(v: Vec<u8>) -> Self {
        Flex::from_owned(v)
    }
}
