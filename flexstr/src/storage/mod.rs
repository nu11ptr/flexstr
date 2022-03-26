mod borrow;
mod heap;
mod inline;

pub(crate) use borrow::*;
pub(crate) use heap::*;
pub(crate) use inline::*;

/// Represents the storage type used by a particular [FlexStr](crate::FlexStr)
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum StorageType {
    /// Denotes that this [FlexStr](crate::FlexStr) is a wrapped string literal
    Static,
    /// Denotes that this [FlexStr](crate::FlexStr) is inlined
    Inline,
    /// Denotes that this [FlexStr](crate::FlexStr) uses heap-based storage
    Heap,
    /// Denotes that this [FlexStr](crate::FlexStr) uses borrowed storage
    Borrow,
}
