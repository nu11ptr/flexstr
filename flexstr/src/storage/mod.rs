pub(crate) mod heap;
pub mod inline;
pub(crate) mod static_ref;

use core::fmt;
use core::fmt::{Debug, Display, Formatter};

// *** Wrong Storage Type ***

/// Error type returned from [try_as_static_str](FlexStr::try_as_static_str) or
/// [try_to_heap](FlexStr::try_to_heap) when this [FlexStr] does not contain the expected type of storage
#[derive(Copy, Clone, Debug)]
pub struct WrongStorageType {
    /// The expected storage type of the string
    pub expected: StorageType,
    /// The actual storage type of the string
    pub actual: StorageType,
}

impl Display for WrongStorageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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

/// Represents the storage type used by a particular [FlexStr]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum StorageType {
    /// Denotes that this [FlexStr] is a wrapper string literal
    Static,
    /// Denotes that this [FlexStr] is inlined
    Inline,
    /// Denotes that this [FlexStr] uses heap-based storage
    Heap,
}
