use alloc::{ffi::CString, rc::Rc, sync::Arc};
use core::ffi::CStr;

use crate::{Flex, RefCounted, StringOps};

/// Local `CStr` type (NOTE: This can't be shared between threads)
pub type LocalCStr<'s> = Flex<'s, CStr, Rc<CStr>>;

/// Shared `CStr` type
pub type SharedCStr<'s> = Flex<'s, CStr, Arc<CStr>>;

// NOTE: This one is a bit different because CString is just a Box<[u8]>. Instead of equal size,
// we should be at most one machine word larger.
const _: () = assert!(
    size_of::<Option<LocalCStr>>() <= size_of::<CString>() + size_of::<usize>(),
    "Option<LocalCStr> must be less than or equal to the size of CString plus one machine word"
);
const _: () = assert!(
    size_of::<Option<SharedCStr>>() <= size_of::<CString>() + size_of::<usize>(),
    "Option<SharedCStr> must be less than or equal to the size of CString plus one machine word"
);

impl<R: RefCounted<CStr>> Flex<'_, CStr, R> {
    /// Borrow the CStr as an `&CStr`
    pub fn as_cstr(&self) -> &CStr {
        self.as_borrowed_type()
    }
}

impl StringOps for CStr {
    #[cfg(feature = "safe")]
    #[inline(always)]
    fn bytes_as_self(bytes: &[u8]) -> &Self {
        // PANIC SAFETY: We know the bytes are a valid CStr
        CStr::from_bytes_with_nul(bytes).expect("Missing NUL byte")
    }

    #[cfg(not(feature = "safe"))]
    #[inline(always)]
    fn bytes_as_self(bytes: &[u8]) -> &Self {
        // SAFETY: We know the bytes are a valid CStr
        unsafe { CStr::from_bytes_with_nul_unchecked(bytes) }
    }

    #[inline(always)]
    fn self_as_bytes(&self) -> &[u8] {
        self.to_bytes_with_nul()
    }
}

// *** From<CString> ***

// NOTE: Cannot be implemented generically because of impl<T> From<T> for T
impl<'s, R: RefCounted<CStr>> From<CString> for Flex<'s, CStr, R> {
    #[inline(always)]
    fn from(s: CString) -> Self {
        Flex::from_owned(s)
    }
}

// *** AsRef<[u8]> ***

// NOTE: Cannot be implemented generically because it conflicts with AsRef<S> for Bytes
impl<R: RefCounted<CStr>> AsRef<[u8]> for Flex<'_, CStr, R> {
    #[inline(always)]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}
