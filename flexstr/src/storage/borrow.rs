use crate::custom::Pad;
use crate::storage::StorageType;

// Cannot yet reference associated types from a generic param (impl trait) for const generic params,
// so we are forced to work with raw const generics for now
#[derive(Clone, Copy)]
#[repr(C)]
pub(crate) struct BorrowStr<const PAD: usize, REF> {
    pub string: REF,
    pad: Pad<PAD>,
    pub marker: StorageType,
}

impl<const PAD: usize, REF> BorrowStr<PAD, REF> {
    #[inline]
    pub const fn from_static(s: REF) -> Self {
        Self {
            string: s,
            // Must be const fn, so can't use default
            pad: Pad::new(),
            marker: StorageType::Static,
        }
    }

    #[inline]
    pub fn from_borrow(s: REF) -> Self {
        Self {
            string: s,
            pad: Pad::new(),
            marker: StorageType::Borrow,
        }
    }
}
