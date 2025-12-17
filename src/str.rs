use alloc::{rc::Rc, string::String, sync::Arc};

use crate::{FlexStr, InlineFlexStr, RefCounted, RefCountedMut, StringToFromBytes};

/// Local `str` type (NOTE: This can't be shared between threads)
pub type LocalStr<'s> = FlexStr<'s, str, Rc<str>>;

/// Shared `str` type
pub type SharedStr<'s> = FlexStr<'s, str, Arc<str>>;

/// Inline `str` type
pub type InlineStr = InlineFlexStr<str>;

const _: () = assert!(
    size_of::<Option<LocalStr>>() <= size_of::<String>(),
    "Option<LocalStr> must be less than or equal to the size of String"
);
const _: () = assert!(
    size_of::<Option<SharedStr>>() <= size_of::<String>(),
    "Option<SharedStr> must be less than or equal to the size of String"
);

impl<'s, R: RefCountedMut<str>> FlexStr<'s, str, R> {
    /// Borrow the string as a mutable string reference, converting if needed. If the string is borrowed,
    /// it is made into an owned string first. RefCounted variants will allocate + copy
    /// if they are shared. In all other cases, the string is borrowed as a mutable reference
    /// directly.
    pub fn to_mut_type(&mut self) -> &mut str {
        match self {
            FlexStr::Borrowed(s) => {
                *self = FlexStr::copy_into_owned(s);
                // copy_into_owned will never return a borrowed variant
                match self {
                    FlexStr::Inlined(s) => s.as_mut_type(),
                    FlexStr::RefCounted(s) => s.as_mut(),
                    FlexStr::Boxed(s) => s.as_mut(),
                    FlexStr::Borrowed(_) => unreachable!("Unexpected borrowed variant"),
                }
            }
            FlexStr::Inlined(s) => s.as_mut_type(),
            FlexStr::RefCounted(s) => s.to_mut(),
            FlexStr::Boxed(s) => s.as_mut(),
        }
    }
}

// *** StringToFromBytes ***

impl StringToFromBytes for str {
    #[cfg(feature = "safe")]
    #[inline]
    fn bytes_as_self(bytes: &[u8]) -> &Self {
        // PANIC SAFETY: We know the bytes are valid UTF-8
        str::from_utf8(bytes).expect("Invalid UTF-8")
    }

    #[cfg(not(feature = "safe"))]
    #[inline]
    fn bytes_as_self(bytes: &[u8]) -> &Self {
        // SAFETY: We know the bytes are valid UTF-8
        unsafe { str::from_utf8_unchecked(bytes) }
    }

    #[inline]
    fn self_as_raw_bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}

// *** From<String> ***

// NOTE: Cannot be implemented generically because of impl<T> From<T> for T
impl<'s, R: RefCounted<str>> From<String> for FlexStr<'s, str, R> {
    fn from(s: String) -> Self {
        FlexStr::from_owned(s)
    }
}

// *** TryFrom<&str> for InlineFlexStr ***

// NOTE: Cannot be implemented generically because of impl<T, U> TryFrom<U> for T where U: Into<T>
impl<'s> TryFrom<&'s str> for InlineFlexStr<str> {
    type Error = &'s str;

    #[inline]
    fn try_from(s: &'s str) -> Result<Self, Self::Error> {
        InlineFlexStr::try_from_type(s)
    }
}

// *** RefCountedMut ***

// NOTE: Cannot be implemented generically because CloneToUninit is needed
// as a bound to `S`, but is unstable.
impl RefCountedMut<str> for Arc<str> {
    #[inline]
    fn to_mut(&mut self) -> &mut str {
        Arc::make_mut(self)
    }

    #[inline]
    fn as_mut(&mut self) -> &mut str {
        // PANIC SAFETY: We only use this when we know the Arc is newly created
        Arc::get_mut(self).expect("Arc is shared")
    }
}

// NOTE: Cannot be implemented generically because CloneToUninit is needed
// as a bound to `S`, but is unstable.
impl RefCountedMut<str> for Rc<str> {
    #[inline]
    fn to_mut(&mut self) -> &mut str {
        Rc::make_mut(self)
    }

    #[inline]
    fn as_mut(&mut self) -> &mut str {
        // PANIC SAFETY: We only use this when we know the Rc is newly created
        Rc::get_mut(self).expect("Rc is shared")
    }
}
