use core::fmt::{Debug, Formatter};
use core::{fmt, mem};

use crate::StorageType;

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct StaticStr<const PAD: usize> {
    pub(crate) literal: &'static str,
    pad: [mem::MaybeUninit<u8>; PAD],
    pub(crate) marker: StorageType,
}

impl<const PAD: usize> StaticStr<PAD> {
    pub const EMPTY: Self = Self::from_static("");

    #[inline]
    pub const fn from_static(s: &'static str) -> Self {
        Self {
            literal: s,
            // SAFETY: Padding, never actually used
            pad: unsafe { mem::MaybeUninit::uninit().assume_init() },
            marker: StorageType::Static,
        }
    }
}

impl<const PAD: usize> Debug for StaticStr<PAD> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <str as Debug>::fmt(self.literal, f)
    }
}
