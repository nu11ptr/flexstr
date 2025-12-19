use alloc::{ffi::CString, rc::Rc, sync::Arc};
use core::ffi::CStr;

use crate::{FlexStr, ImmutableBytes, InlineFlexStr, RefCounted, RefCountedMut, StringToFromBytes};

/// Local `CStr` type (NOTE: This can't be shared between threads)
pub type LocalCStr<'s> = FlexStr<'s, CStr, Rc<CStr>>;

/// Shared `CStr` type
pub type SharedCStr<'s> = FlexStr<'s, CStr, Arc<CStr>>;

/// Inline `CStr` type
pub type InlineCStr = InlineFlexStr<CStr>;

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

impl<R: RefCounted<CStr>> FlexStr<'_, CStr, R> {
    /// Borrow the CStr as bytes with a trailing NUL byte
    pub fn as_bytes_with_nul(&self) -> &[u8] {
        self.as_raw_bytes()
    }
}

impl InlineFlexStr<CStr> {
    /// Borrow the CStr as bytes with a trailing NUL byte
    #[inline]
    pub fn as_bytes_with_nul(&self) -> &[u8] {
        self.as_raw_bytes()
    }
}

// *** StringToFromBytes ***

impl StringToFromBytes for CStr {
    #[cfg(feature = "safe")]
    #[inline]
    fn bytes_as_self(bytes: &[u8]) -> &Self {
        // PANIC SAFETY: We know the bytes are a valid CStr
        CStr::from_bytes_with_nul(bytes).expect("Missing NUL byte")
    }

    #[cfg(not(feature = "safe"))]
    #[inline]
    fn bytes_as_self(bytes: &[u8]) -> &Self {
        // SAFETY: We know the bytes are a valid CStr
        unsafe { CStr::from_bytes_with_nul_unchecked(bytes) }
    }

    #[inline]
    fn self_as_bytes(&self) -> &[u8] {
        self.to_bytes()
    }

    #[inline]
    fn self_as_raw_bytes(&self) -> &[u8] {
        self.to_bytes_with_nul()
    }
}

// *** ImmutableBytes ***

impl ImmutableBytes for CStr {}

// *** From<CString> ***

// NOTE: Cannot be implemented generically because of impl<T> From<T> for T
impl<'s, R: RefCounted<CStr>> From<CString> for FlexStr<'s, CStr, R> {
    fn from(s: CString) -> Self {
        FlexStr::from_owned(s)
    }
}

// *** TryFrom<&CStr> for InlineFlexStr ***

// NOTE: Cannot be implemented generically because of impl<T, U> TryFrom<U> for T where U: Into<T>
impl<'s> TryFrom<&'s CStr> for InlineFlexStr<CStr> {
    type Error = &'s CStr;

    #[inline]
    fn try_from(s: &'s CStr) -> Result<Self, Self::Error> {
        InlineFlexStr::try_from_type(s)
    }
}

// *** RefCountedMut ***

// NOTE: Cannot be implemented generically because CloneToUninit is needed
// as a bound to `S`, but is unstable.
impl RefCountedMut<CStr> for Arc<CStr> {
    #[inline]
    fn to_mut(&mut self) -> &mut CStr {
        Arc::make_mut(self)
    }

    #[inline]
    fn as_mut(&mut self) -> &mut CStr {
        // PANIC SAFETY: We only use this when we know the Arc is newly created
        Arc::get_mut(self).expect("Arc is shared")
    }
}

// NOTE: Cannot be implemented generically because CloneToUninit is needed
// as a bound to `S`, but is unstable.
impl RefCountedMut<CStr> for Rc<CStr> {
    #[inline]
    fn to_mut(&mut self) -> &mut CStr {
        Rc::make_mut(self)
    }

    #[inline]
    fn as_mut(&mut self) -> &mut CStr {
        // PANIC SAFETY: We only use this when we know the Rc is newly created
        Rc::get_mut(self).expect("Rc is shared")
    }
}
