use alloc::{rc::Rc, sync::Arc};

use crate::{FlexStr, RefCounted, StringOps};

/// Local `[u8]` type (NOTE: This can't be shared between threads)
pub type LocalBytes<'s> = FlexStr<'s, [u8], Rc<[u8]>>;

/// Shared `[u8]` type
pub type SharedBytes<'s> = FlexStr<'s, [u8], Arc<[u8]>>;

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
impl<'s, R: RefCounted<[u8]>> From<Vec<u8>> for FlexStr<'s, [u8], R> {
    #[inline(always)]
    fn from(v: Vec<u8>) -> Self {
        FlexStr::from_owned(v)
    }
}
