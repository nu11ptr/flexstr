extern crate alloc;

mod boxed;
#[cfg(feature = "bytes")]
mod bytes;
#[cfg(not(feature = "safe"))]
mod small_box;
#[cfg(feature = "str")]
mod str;

pub use boxed::BoxedFlexStr;
#[cfg(feature = "bytes")]
pub use bytes::BoxedBytes;
#[cfg(feature = "str")]
pub use str::BoxedStr;
