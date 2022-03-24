use core::fmt::{Debug, Formatter};
use core::ops::Deref;
use core::{fmt, mem};

use crate::StorageType;

// HEAP will likely align this just fine, but since we don't know the size, this is safest
#[cfg_attr(target_pointer_width = "64", repr(align(8)))]
#[cfg_attr(target_pointer_width = "32", repr(align(4)))]
#[repr(C)]
#[derive(Clone)]
pub(crate) struct HeapStr<const PAD: usize, HEAP> {
    pub heap: HEAP,
    pad: [mem::MaybeUninit<u8>; PAD],
    pub marker: StorageType,
}

impl<const PAD: usize, HEAP> HeapStr<PAD, HEAP> {
    #[inline]
    pub fn from_heap(t: HEAP) -> Self {
        Self {
            heap: t,
            // SAFETY: Padding, never actually used
            pad: unsafe { mem::MaybeUninit::uninit().assume_init() },
            marker: StorageType::Heap,
        }
    }

    #[inline]
    pub fn from_ref(s: impl AsRef<str>) -> Self
    where
        HEAP: for<'a> From<&'a str>,
    {
        Self::from_heap(s.as_ref().into())
    }
}

impl<const PAD: usize, HEAP> Debug for HeapStr<PAD, HEAP>
where
    HEAP: Deref<Target = str>,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <str as Debug>::fmt(&self.heap, f)
    }
}
