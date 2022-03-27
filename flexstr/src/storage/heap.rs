use crate::custom::Pad;
use crate::storage::StorageType;
use crate::string::Str;
use core::marker::PhantomData;

#[derive(Clone)]
#[repr(C)]
pub(crate) struct HeapStr<const PAD: usize, HEAP, STR>
where
    STR: Str + ?Sized,
{
    pub heap: HEAP,
    pad: Pad<PAD>,
    pub marker: StorageType,
    phantom: PhantomData<STR>,
}
