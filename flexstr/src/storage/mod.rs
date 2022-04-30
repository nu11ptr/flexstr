mod borrow;
mod heap;
mod inline;

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::sync::Arc;
use core::fmt;
use core::fmt::Debug as _;

pub(crate) use borrow::*;
pub(crate) use heap::*;
pub(crate) use inline::*;

use crate::string::Str;

// *** Wrong Storage Type ***

/// Error type returned from [try_as_static_str](crate::FlexStr::try_as_static_str) when this
/// [FlexStr](crate::FlexStr) does not contain the expected type of storage
#[derive(Copy, Clone, Debug)]
pub struct WrongStorageType {
    /// The expected storage type of the string
    pub expected: StorageType,
    /// The actual storage type of the string
    pub actual: StorageType,
}

impl fmt::Display for WrongStorageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("The FlexStr did not use the storage type expected (expected: ")?;
        self.expected.fmt(f)?;
        f.write_str(", actual: ")?;
        self.actual.fmt(f)?;
        f.write_str(")")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for WrongStorageType {}

// *** Storage Type ***

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

// *** Storage ***

/// Trait used for implementing custom heap storage backends
pub trait Storage<STR>
where
    STR: Str + ?Sized,
{
    /// Takes a string reference and returns a newly created inner heap type
    fn from_ref(s: &STR) -> Self;

    /// Returns the contents of this storage
    fn as_heap_type(&self) -> &STR::HeapType;
}

impl<STR> Storage<STR> for Rc<STR::HeapType>
where
    Rc<STR::HeapType>: for<'a> From<&'a STR::HeapType>,
    STR: Str + ?Sized,
{
    #[inline]
    fn from_ref(s: &STR) -> Self {
        s.as_heap_type().into()
    }

    #[inline]
    fn as_heap_type(&self) -> &STR::HeapType {
        self.as_ref()
    }
}

impl<STR> Storage<STR> for Arc<STR::HeapType>
where
    Arc<STR::HeapType>: for<'a> From<&'a STR::HeapType>,
    STR: Str + ?Sized,
{
    #[inline]
    fn from_ref(s: &STR) -> Self {
        s.as_heap_type().into()
    }

    #[inline]
    fn as_heap_type(&self) -> &STR::HeapType {
        self.as_ref()
    }
}

impl<STR> Storage<STR> for Box<STR::HeapType>
where
    Box<STR::HeapType>: for<'a> From<&'a STR::HeapType>,
    STR: Str + ?Sized,
{
    #[inline]
    fn from_ref(s: &STR) -> Self {
        s.as_heap_type().into()
    }

    #[inline]
    fn as_heap_type(&self) -> &STR::HeapType {
        self.as_ref()
    }
}
