use core::marker::PhantomData;

use crate::custom::Pad;
use crate::storage::{Storage, StorageType};
use crate::string::Str;

#[derive(Clone)]
#[repr(C)]
pub(crate) struct HeapStr<const PAD: usize, HEAP, STR>
where
    STR: ?Sized,
{
    // TODO: Clone reaches in for performance reasons...revisit that
    pub heap: HEAP,
    pad: Pad<PAD>,
    pub marker: StorageType,
    // TODO: Do research on phantom type as relates to variance and auto traits
    phantom: PhantomData<fn(STR) -> STR>,
}

impl<const PAD: usize, HEAP, STR> HeapStr<PAD, HEAP, STR>
where
    HEAP: Storage<STR>,
    STR: Str + ?Sized,
{
    #[inline]
    pub fn from_heap(t: HEAP) -> Self {
        Self {
            heap: t,
            pad: Pad::new(),
            marker: StorageType::Heap,
            phantom: PhantomData,
        }
    }

    #[inline]
    pub fn from_ref(s: impl AsRef<STR>) -> Self {
        Self::from_heap(HEAP::from_ref(s.as_ref()))
    }

    #[inline]
    pub fn as_str_type(&self) -> &STR {
        STR::from_heap_data(self.heap.as_heap_type())
    }
}
