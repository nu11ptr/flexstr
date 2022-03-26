use crate::custom::Size;
use crate::storage::StorageType;
use crate::string::Str;

#[derive(Clone, Copy)]
#[repr(C)]
pub(crate) struct BorrowStr<SIZE, STR, REF>
where
    SIZE: Size<STR>,
    STR: Str + ?Sized,
{
    pub string: REF,
    pad: SIZE::BorrowPad,
    pub marker: StorageType,
}
