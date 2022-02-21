#![no_std]

extern crate alloc;

use alloc::borrow::ToOwned;
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use core::cmp::Ordering;
use core::fmt::{Debug, Display, Formatter};
use core::hash::{Hash, Hasher};
use core::mem;
use core::ops::Deref;
use core::str;
use core::{fmt, ptr};

// *** Inline String ***

/// The max capacity of an inline string (in bytes)
pub const MAX_INLINE: usize = mem::size_of::<String>() + mem::size_of::<usize>() - 2;

#[derive(Clone, Debug)]
pub struct InlineStringy {
    len: u8,
    data: [u8; MAX_INLINE],
}

impl InlineStringy {
    #[inline]
    pub fn try_new<T: AsRef<str>>(s: T) -> Result<Self, T> {
        let s_ref = s.as_ref();

        if s_ref.len() > MAX_INLINE {
            Err(s)
        } else {
            unsafe { Ok(Self::new(s_ref)) }
        }
    }

    unsafe fn new(s: &str) -> Self {
        // Safety: This is safe because while uninitialized to start, we copy the the str contents
        // over the top. We check to ensure it is not too long in `try_new` and don't call this
        // function directly. The copy is restrained to the length of the str.

        // Declare array, but keep uninitialized (we will overwrite momentarily)
        let data: [mem::MaybeUninit<u8>; MAX_INLINE] = mem::MaybeUninit::uninit().assume_init();
        let mut data = mem::transmute::<_, [u8; MAX_INLINE]>(data);

        // Copy contents of &str to our data buffer
        ptr::copy_nonoverlapping(s.as_ptr(), &mut data as *mut u8, s.len());

        Self {
            len: s.len() as u8,
            data,
        }
    }
}

impl Display for InlineStringy {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl Deref for InlineStringy {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        // Safety: The contents are always obtained from a valid UTF8 str, so they must be valid
        // Additionally, we clamp the size of the slice passed to be no longer than our str length
        unsafe { str::from_utf8_unchecked(&self.data[..(self.len as usize)]) }
    }
}

impl TryFrom<String> for InlineStringy {
    type Error = String;

    #[inline]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl<'s> TryFrom<&'s String> for InlineStringy {
    type Error = &'s String;

    #[inline]
    fn try_from(value: &'s String) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl<'s> TryFrom<&'s str> for InlineStringy {
    type Error = &'s str;

    #[inline]
    fn try_from(value: &'s str) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl From<InlineStringy> for String {
    #[inline]
    fn from(s: InlineStringy) -> Self {
        (&*s).to_string()
    }
}

// *** Stringy macro ***

macro_rules! stringy {
    ($name:ident, $rc:ty, $to_func:ident) => {
        #[derive(Clone, Debug)]
        pub enum $name {
            Static(&'static str),
            Inlined(InlineStringy),
            RefCounted($rc),
        }

        impl $name {
            /// Returns true if this is a wrapped &'static str (string literal)
            #[inline]
            pub fn is_static(&self) -> bool {
                matches!(self, $name::Static(_))
            }

            /// Returns true if this is an inlined string
            #[inline]
            pub fn is_inline(&self) -> bool {
                matches!(self, $name::Inlined(_))
            }

            /// Returns true if this Stringy is a wrapped String using reference counting
            #[inline]
            pub fn is_ref_counted(&self) -> bool {
                matches!(self, $name::RefCounted(_))
            }

            /// Returns true if we can unwrap a native Rust String without allocating else false
            #[inline]
            pub fn can_unwrap_string(&self) -> bool {
                match self {
                    $name::RefCounted(rc) => <$rc>::strong_count(rc) == 1,
                    _ => false,
                }
            }

            /// Wrap string verbatim (without possibility of inlining). This can be useful in exclusive
            /// ownership situations where you need to extract the original String later
            pub fn wrap(s: String) -> Self {
                $name::RefCounted(<$rc>::new(s))
            }

            /// Try to retrieve the inner `String` if there is one and we have exclusive ownership. If not
            /// or we don't, then create a new String and return it instead.
            pub fn into_string(self) -> String {
                match self {
                    $name::Static(str) => str.to_string(),
                    $name::Inlined(ss) => ss.into(),
                    $name::RefCounted(rc) => match <$rc>::try_unwrap(rc) {
                        Ok(s) => s,
                        Err(rc) => (&*rc).to_owned(),
                    },
                }
            }

            /// Try to retrieve the inner `String` if there is one and we have exclusive ownership. If not
            /// or we don't, then return our Stringy as the error in the result.
            pub fn try_into_string(self) -> Result<String, $name> {
                match self {
                    s @ $name::Static(_) => Err(s),
                    ss @ $name::Inlined(_) => Err(ss),
                    $name::RefCounted(rc) => <$rc>::try_unwrap(rc).map_err($name::RefCounted),
                }
            }
        }

        // *** Hash, PartialEq, Eq, PartialOrd, Ord ***

        impl Hash for $name {
            #[inline]
            fn hash<H: Hasher>(&self, state: &mut H) {
                Hash::hash(&**self, state)
            }
        }

        impl PartialEq for $name {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                PartialEq::eq(&**self, &**other)
            }
        }

        impl PartialEq<str> for $name {
            #[inline]
            fn eq(&self, other: &str) -> bool {
                PartialEq::eq(&**self, other)
            }
        }

        impl PartialEq<String> for $name {
            #[inline]
            fn eq(&self, other: &String) -> bool {
                PartialEq::eq(&**self, other)
            }
        }

        impl Eq for $name {}

        impl PartialOrd for $name {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                PartialOrd::partial_cmp(&**self, &**other)
            }
        }

        impl PartialOrd<str> for $name {
            #[inline]
            fn partial_cmp(&self, other: &str) -> Option<Ordering> {
                PartialOrd::partial_cmp(&**self, other)
            }
        }

        impl PartialOrd<String> for $name {
            #[inline]
            fn partial_cmp(&self, other: &String) -> Option<Ordering> {
                PartialOrd::partial_cmp(&**self, other)
            }
        }

        impl Ord for $name {
            #[inline]
            fn cmp(&self, other: &Self) -> Ordering {
                Ord::cmp(&**self, &**other)
            }
        }

        // *** Deref / Display ***

        impl Deref for $name {
            type Target = str;

            #[inline]
            fn deref(&self) -> &Self::Target {
                match self {
                    $name::Static(str) => str,
                    $name::Inlined(ss) => ss,
                    $name::RefCounted(rc) => rc,
                }
            }
        }

        impl Display for $name {
            #[inline]
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                match self {
                    $name::Static(str) => Display::fmt(str, f),
                    $name::Inlined(ss) => Display::fmt(ss, f),
                    $name::RefCounted(s) => Display::fmt(s, f),
                }
            }
        }

        // *** From ***

        impl From<String> for $name {
            #[inline]
            fn from(s: String) -> Self {
                match s.try_into() {
                    Ok(s) => $name::Inlined(s),
                    Err(s) => $name::RefCounted(<$rc>::new(s)),
                }
            }
        }

        impl From<&String> for $name {
            #[inline]
            fn from(s: &String) -> Self {
                s.$to_func()
            }
        }

        impl From<&'static str> for $name {
            #[inline]
            fn from(s: &'static str) -> Self {
                $name::Static(s)
            }
        }
    };
}

stringy!(Stringy, Rc<String>, to_stringy);
stringy!(AStringy, Arc<String>, to_astringy);

// *** Stringy ***

impl From<&AStringy> for Stringy {
    #[inline]
    fn from(s: &AStringy) -> Self {
        s.clone().into()
    }
}

impl From<AStringy> for Stringy {
    fn from(s: AStringy) -> Self {
        match s {
            AStringy::Static(s) => Stringy::Static(s),
            AStringy::Inlined(s) => Stringy::Inlined(s),
            AStringy::RefCounted(rc) => {
                let s = match Arc::try_unwrap(rc) {
                    Ok(s) => s,
                    Err(rc) => (&*rc).to_owned(),
                };
                Stringy::RefCounted(Rc::new(s))
            }
        }
    }
}

pub trait ToStringy {
    fn to_stringy(&self) -> Stringy;
}

impl ToStringy for str {
    #[inline]
    fn to_stringy(&self) -> Stringy {
        match self.try_into() {
            Ok(s) => Stringy::Inlined(s),
            Err(_) => Stringy::wrap(self.to_string()),
        }
    }
}

// *** AStringy ***

impl From<&Stringy> for AStringy {
    #[inline]
    fn from(s: &Stringy) -> Self {
        s.clone().into()
    }
}

impl From<Stringy> for AStringy {
    fn from(s: Stringy) -> Self {
        match s {
            Stringy::Static(s) => AStringy::Static(s),
            Stringy::Inlined(s) => AStringy::Inlined(s),
            Stringy::RefCounted(rc) => {
                let s = match Rc::try_unwrap(rc) {
                    Ok(s) => s,
                    Err(rc) => (&*rc).to_owned(),
                };
                AStringy::RefCounted(Arc::new(s))
            }
        }
    }
}

pub trait ToAStringy {
    fn to_astringy(&self) -> AStringy;
}

impl ToAStringy for str {
    #[inline]
    fn to_astringy(&self) -> AStringy {
        match self.try_into() {
            Ok(s) => AStringy::Inlined(s),
            Err(_) => AStringy::wrap(self.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {}
