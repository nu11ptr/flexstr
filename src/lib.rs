#![no_std]
#![warn(missing_docs)]

//! A simple to use, immutable, clone-efficient `String` replacement for Rust

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

#[cfg(feature = "serde")]
use serde::de::{Error, Visitor};
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// *** Inline String ***

/// The max capacity of an inline string (in bytes)
pub const MAX_INLINE: usize = mem::size_of::<String>() + mem::size_of::<usize>() - 2;

/// This is the custom inline string type - it is not typically used directly, but instead is used
/// transparently by `Stringy` and `AStringy`
#[derive(Clone, Debug)]
pub struct InlineStringy {
    len: u8,
    data: [u8; MAX_INLINE],
}

impl InlineStringy {
    /// Attempts to return a new `InlineStringy` if the source string is short enough to be copied.
    /// If not, the source is returned as the error.
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
    ($name:ident, $name2:ident, $rc:ty, $rc2:ty, $to_func:ident, $to_func2:ident, $visitor_name: ident) => {
        /// The main string enum type that wraps a string literal, inline string, or ref counted `String`
        #[derive(Clone, Debug)]
        pub enum $name {
            /// A wrapped string literal
            Static(&'static str),
            /// An inlined string
            Inlined(InlineStringy),
            /// A reference count wrapped `String`
            RefCounted($rc),
        }

        impl $name {
            #[doc = concat!("Returns true if this `", stringify!($name),"` is empty")]
            #[inline]
            pub fn is_empty(&self) -> bool {
                (&**self).is_empty()
            }

            #[doc = concat!("Returns the length of this `", stringify!($name),"` in bytes (not chars/graphemes)")]
            #[inline]
            pub fn len(&self) -> usize {
                (&**self).len()
            }

            #[doc = concat!("Extracts a string slice containing the entire `", stringify!($name), "`")]
            #[inline]
            pub fn as_str(&self) -> &str {
                &**self
            }

            /// Returns true if this is a wrapped string literal (`&'static str`)
            #[inline]
            pub fn is_static(&self) -> bool {
                matches!(self, $name::Static(_))
            }

            /// Returns true if this is an inlined string
            #[inline]
            pub fn is_inlined(&self) -> bool {
                matches!(self, $name::Inlined(_))
            }

            /// Returns true if this is a wrapped `String` using reference counting
            #[inline]
            pub fn is_ref_counted(&self) -> bool {
                matches!(self, $name::RefCounted(_))
            }

            /// Returns true if we can unwrap a native `String` without any further allocations/copying
            #[inline]
            pub fn unwrappable_string(&self) -> bool {
                matches!(self, $name::RefCounted(rc) if <$rc>::strong_count(rc) == 1)
            }

            /// Wrap `String` verbatim (without possibility of inlining). This can be useful in exclusive
            /// ownership situations where the original `String` is needed later
            #[inline]
            pub fn wrap(s: String) -> Self {
                $name::RefCounted(<$rc>::new(s))
            }

            /// Try to retrieve the inner `String` if there is one and we have exclusive ownership. If not
            /// or we don't, then create a new `String` and return it instead.
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
            #[doc = concat!("or we don't, then return our `", stringify!($name), "` as the error in the result.")]
            pub fn try_into_string(self) -> Result<String, Self> {
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

        /// ```
        #[doc = concat!("use stringy::", stringify!($name), ";")]
        ///
        #[doc = concat!("let s: ", stringify!($name), " = \"inlined\".into();")]
        #[doc = concat!("let s2: ", stringify!($name), " = s.clone();")]
        /// assert_eq!(s, s2);
        /// ```
        impl PartialEq for $name {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                <&str as PartialEq>::eq(&self.deref(), &other.deref())
            }
        }

        /// ```
        #[doc = concat!("use stringy::{", stringify!($name2), ", ", stringify!($name), ", To", stringify!($name2) ,"};")]
        ///
        #[doc = concat!("let s: ", stringify!($name), " = \"inlined\".into();")]
        #[doc = concat!("let s2: ", stringify!($name2), " = s.", stringify!($to_func2), "();")]
        /// assert_eq!(s, s2);
        /// ```
        impl PartialEq<$name2> for $name {
            fn eq(&self, other: &$name2) -> bool {
                <&str as PartialEq>::eq(&self.deref(), &other.deref())
            }
        }

        /// ```
        #[doc = concat!("use stringy::{", stringify!($name), ", To", stringify!($name) ,"};")]
        ///
        /// let lit = "inlined";
        #[doc = concat!("let s: ", stringify!($name), " = lit.", stringify!($to_func), "();")]
        /// assert_eq!(s, lit);
        /// ```
        impl PartialEq<&str> for $name {
            #[inline]
            fn eq(&self, other: &&str) -> bool {
                <&str as PartialEq>::eq(&self.deref(), &other.deref())
            }
        }

        /// ```
        #[doc = concat!("use stringy::{", stringify!($name), ", To", stringify!($name) ,"};")]
        ///
        /// let lit = "inlined";
        #[doc = concat!("let s: ", stringify!($name), " = lit.", stringify!($to_func), "();")]
        /// assert_eq!(s, lit);
        /// ```
        impl PartialEq<str> for $name {
            #[inline]
            fn eq(&self, other: &str) -> bool {
                <&str as PartialEq>::eq(&self.deref(), &other.deref())
            }
        }

        /// ```
        #[doc = concat!("use stringy::", stringify!($name), ";")]
        ///
        /// let lit = "inlined";
        #[doc = concat!("let s: ", stringify!($name), " = lit.into();")]
        /// assert_eq!(s, lit.to_string());
        /// ```
        impl PartialEq<String> for $name {
            #[inline]
            fn eq(&self, other: &String) -> bool {
                <&str as PartialEq>::eq(&self.deref(), &other.deref())
            }
        }

        impl Eq for $name {}

        impl PartialOrd for $name {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                <&str as PartialOrd>::partial_cmp(&self.deref(), &other.deref())
            }
        }

        impl PartialOrd<str> for $name {
            #[inline]
            fn partial_cmp(&self, other: &str) -> Option<Ordering> {
                <&str as PartialOrd>::partial_cmp(&self.deref(), &other.deref())
            }
        }

        impl PartialOrd<String> for $name {
            #[inline]
            fn partial_cmp(&self, other: &String) -> Option<Ordering> {
                <&str as PartialOrd>::partial_cmp(&self.deref(), &other.deref())
            }
        }

        impl Ord for $name {
            #[inline]
            fn cmp(&self, other: &Self) -> Ordering {
                <&str as Ord>::cmp(&self.deref(), &other.deref())
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

        impl From<&$name2> for $name {
            #[inline]
            fn from(s: &$name2) -> Self {
                s.clone().into()
            }
        }

        impl From<$name2> for $name {
            fn from(s: $name2) -> Self {
                match s {
                    $name2::Static(s) => $name::Static(s),
                    $name2::Inlined(s) => $name::Inlined(s),
                    $name2::RefCounted(rc) => {
                        let s = match <$rc2>::try_unwrap(rc) {
                            Ok(s) => s,
                            Err(rc) => (&*rc).to_owned(),
                        };
                        $name::RefCounted(<$rc>::new(s))
                    }
                }
            }
        }

        #[doc = concat!("Converts a `String` into a `", stringify!($name), "`")]
        /// ```
        #[doc = concat!("use stringy::", stringify!($name), ";")]
        ///
        /// let lit = "inlined";
        #[doc = concat!("let s: ", stringify!($name), " = lit.to_string().into();")]
        /// assert!(s.is_inlined());
        /// assert_eq!(&s, lit);
        ///
        /// let lit = "This is too long too be inlined!";
        #[doc = concat!("let s: ", stringify!($name), " = lit.to_string().into();")]
        /// assert!(s.is_ref_counted());
        /// assert_eq!(&s, lit);
        /// ```
        impl From<String> for $name {
            #[inline]
            fn from(s: String) -> Self {
                match s.try_into() {
                    Ok(s) => $name::Inlined(s),
                    Err(s) => $name::RefCounted(<$rc>::new(s)),
                }
            }
        }

        #[doc = concat!("Converts a `&String` into a `", stringify!($name), "`")]
        /// ```
        #[doc = concat!("use stringy::", stringify!($name), ";")]
        ///
        /// let lit = "inlined";
        #[doc = concat!("let s: ", stringify!($name), " = (&lit.to_string()).into();")]
        /// assert!(s.is_inlined());
        /// assert_eq!(&s, lit);
        ///
        /// let lit = "This is too long too be inlined!";
        #[doc = concat!("let s: ", stringify!($name), " = (&lit.to_string()).into();")]
        /// assert!(s.is_ref_counted());
        /// assert!(s.unwrappable_string());
        /// assert_eq!(&s, lit);
        /// ```
        impl From<&String> for $name {
            #[inline]
            fn from(s: &String) -> Self {
                s.$to_func()
            }
        }

        #[doc = concat!("Converts a string literal (`&static str`) into a `", stringify!($name), "`")]
        /// ```
        #[doc = concat!("use stringy::", stringify!($name), ";")]
        ///
        /// let lit = "static";
        #[doc = concat!("let s: ", stringify!($name), " = lit.into();")]
        /// assert!(s.is_static());
        /// assert_eq!(&s, lit);
        /// ```
        impl From<&'static str> for $name {
            #[inline]
            fn from(s: &'static str) -> Self {
                $name::Static(s)
            }
        }

        #[cfg(feature = "serde")]
        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(self)
            }
        }

        #[cfg(feature = "serde")]
        struct $visitor_name;

        #[cfg(feature = "serde")]
        impl<'de> Visitor<'de> for $visitor_name {
            type Value = $name;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("a string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(v.$to_func())
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(v.into())
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_str($visitor_name)
            }
        }
    };
}

// *** Stringy ***

stringy!(
    Stringy,
    AStringy,
    Rc<String>,
    Arc<String>,
    to_stringy,
    to_a_stringy,
    StringyVisitor
);

/// A trait that converts the source to a `Stringy` without consuming it
pub trait ToStringy {
    /// Converts the source to a `Stringy` without consuming it
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

stringy!(
    AStringy,
    Stringy,
    Arc<String>,
    Rc<String>,
    to_a_stringy,
    to_stringy,
    AStringyVisitor
);

/// A trait that converts the source to an `AStringy` without consuming it
pub trait ToAStringy {
    /// Converts the source to an `AStringy` without consuming it
    fn to_a_stringy(&self) -> AStringy;
}

impl ToAStringy for str {
    #[inline]
    fn to_a_stringy(&self) -> AStringy {
        match self.try_into() {
            Ok(s) => AStringy::Inlined(s),
            Err(_) => AStringy::wrap(self.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {}
