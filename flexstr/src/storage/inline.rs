use crate::custom::Size;
use crate::storage::StorageType;
use crate::string::Str;

#[doc(hidden)]
#[derive(Clone, Copy)]
#[cfg_attr(target_pointer_width = "64", repr(align(8)))]
#[cfg_attr(target_pointer_width = "32", repr(align(4)))]
#[repr(C)]
pub(crate) struct InlineStr<SIZE, STR>
where
    SIZE: Size<STR>,
    STR: Str + ?Sized,
{
    data: SIZE::InlineStorage,
    len: u8,
    pub marker: StorageType,
}
