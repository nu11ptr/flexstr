#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use alloc::{borrow::Cow, rc::Rc, sync::Arc};
use core::{convert::Infallible, str::FromStr};

use crate::{FlexStr, RefCounted, RefCountedMut, partial_eq_impl, ref_counted_mut_impl};

use flexstr_support::StringToFromBytes;

/// Local `[u8]` type (NOTE: This can't be shared between threads)
pub type LocalBytes<'s> = FlexStr<'s, [u8], Rc<[u8]>>;

/// Shared `[u8]` type
pub type SharedBytes<'s> = FlexStr<'s, [u8], Arc<[u8]>>;

const _: () = assert!(
    size_of::<Option<LocalBytes>>() <= size_of::<Vec<u8>>(),
    "Option<LocalBytes> must be less than or equal to the size of Vec<u8>"
);
const _: () = assert!(
    size_of::<Option<SharedBytes>>() <= size_of::<Vec<u8>>(),
    "Option<SharedBytes> must be less than or equal to the size of Vec<u8>"
);

// *** RefCountedMut ***

ref_counted_mut_impl!([u8]);

// *** From for FlexStr ***

// NOTE: Cannot be implemented generically because of impl<T> From<T> for T
impl<'s, R: RefCounted<[u8]>> From<Vec<u8>> for FlexStr<'s, [u8], R> {
    fn from(v: Vec<u8>) -> Self {
        FlexStr::from_owned(v)
    }
}

impl<'s, R: RefCounted<[u8]>> From<&'s str> for FlexStr<'s, [u8], R> {
    fn from(s: &'s str) -> Self {
        FlexStr::from_borrowed(s.as_bytes())
    }
}

// *** PartialEq ***

partial_eq_impl!([u8], [u8]);
partial_eq_impl!(&[u8], [u8]);
partial_eq_impl!(Vec<u8>, [u8]);
partial_eq_impl!(Cow<'s, [u8]>, [u8]);

// *** AsRef ***

impl<'s, S: ?Sized + StringToFromBytes, R: RefCounted<S>> AsRef<[u8]> for FlexStr<'s, S, R>
where
    S: AsRef<[u8]>,
{
    fn as_ref(&self) -> &[u8] {
        self.as_ref_type().as_ref()
    }
}

// *** FromStr ***

impl<R: RefCounted<[u8]>> FromStr for FlexStr<'static, [u8], R> {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(FlexStr::from_borrowed(s.as_bytes()).into_owned())
    }
}
