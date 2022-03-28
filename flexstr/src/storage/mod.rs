mod borrow;
mod heap;
mod inline;

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::sync::Arc;

pub(crate) use borrow::*;
pub(crate) use heap::*;
pub(crate) use inline::*;

use crate::string::Str;

/// Represents the storage type used by a particular [FlexStrBase](crate::FlexStrBase)
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum StorageType {
    /// Denotes that this [FlexStrBase](crate::FlexStrBase) is a wrapped string literal
    Static,
    /// Denotes that this [FlexStrBase](crate::FlexStrBase) is inlined
    Inline,
    /// Denotes that this [FlexStrBase](crate::FlexStrBase) uses heap-based storage
    Heap,
    /// Denotes that this [FlexStrBase](crate::FlexStrBase) uses borrowed storage
    Borrow,
}

pub trait Storage<STR: Str + ?Sized> {
    fn from_ref(s: &STR) -> Self;
}

impl<STR: Str> Storage<STR> for Rc<STR::HeapType>
where
    Rc<STR::HeapType>: for<'a> From<&'a STR::HeapType>,
{
    #[inline]
    fn from_ref(s: &STR) -> Self {
        s.as_heap_type().into()
    }
}

impl<STR: Str> Storage<STR> for Arc<STR::HeapType>
where
    Arc<STR::HeapType>: for<'a> From<&'a STR::HeapType>,
{
    #[inline]
    fn from_ref(s: &STR) -> Self {
        s.as_heap_type().into()
    }
}

impl<STR: Str> Storage<STR> for Box<STR::HeapType>
where
    Box<STR::HeapType>: for<'a> From<&'a STR::HeapType>,
{
    #[inline]
    fn from_ref(s: &STR) -> Self {
        s.as_heap_type().into()
    }
}
