use core::mem;

use crate::storage::StorageType;
use crate::string::Str;

/// Type representing the inline storage including its size and string type
type InlineStorage<const N: usize, STR> = [mem::MaybeUninit<<STR as Str>::InlineType>; N];

#[doc(hidden)]
#[derive(Clone, Copy)]
#[cfg_attr(target_pointer_width = "64", repr(align(8)))]
#[cfg_attr(target_pointer_width = "32", repr(align(4)))]
#[repr(C)]
pub(crate) struct InlineStr<const SIZE: usize, STR>
where
    STR: Str + ?Sized,
{
    data: InlineStorage<SIZE, STR>,
    len: u8,
    pub marker: StorageType,
}
