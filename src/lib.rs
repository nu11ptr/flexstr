#![no_std]
#![warn(missing_docs)]

//! A simple to use, immutable, clone-efficient `String` replacement for Rust

extern crate alloc;

mod build;
mod inline;

use alloc::rc::Rc;
use alloc::string::String;
use alloc::sync::Arc;
use core::cmp::Ordering;
use core::fmt::{Arguments, Debug, Display, Formatter, Write};
use core::hash::{Hash, Hasher};
use core::ops::Deref;
use core::{fmt, str};

#[cfg(feature = "serde")]
use serde::de::{Error, Visitor};
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// *** Inline String ***

// *** FlexStr macro ***

macro_rules! flexstr {
    ($name:ident, $name2:ident, $rc:ty, $rc2:ty, $to_func:ident, $to_func2:ident, $visitor_name: ident) => {
        /// The main string enum type that wraps a string literal, inline string, or ref counted `str`
        #[derive(Clone, Debug)]
        pub enum $name {
            /// A wrapped string literal
            Static(&'static str),
            /// An inlined string
            Inlined(inline::InlineFlexStr),
            /// A reference count wrapped `str`
            RefCounted($rc),
        }

        impl $name {
            #[doc = concat!("Returns true if this `", stringify!($name),"` is empty")]
            #[inline]
            pub fn is_empty(&self) -> bool {
                self.deref().is_empty()
            }

            #[doc = concat!("Returns the length of this `", stringify!($name),"` in bytes (not chars/graphemes)")]
            #[inline]
            pub fn len(&self) -> usize {
                self.deref().len()
            }

            #[doc = concat!("Extracts a string slice containing the entire `", stringify!($name), "`")]
            #[inline]
            pub fn as_str(&self) -> &str {
                self.deref()
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

            /// Returns true if this is a wrapped `str` using reference counting
            #[inline]
            pub fn is_ref_counted(&self) -> bool {
                matches!(self, $name::RefCounted(_))
            }
        }

        // *** Hash, PartialEq, Eq, PartialOrd, Ord ***

        impl Hash for $name {
            #[inline]
            fn hash<H: Hasher>(&self, state: &mut H) {
                Hash::hash(self.deref(), state)
            }
        }

        /// ```
        #[doc = concat!("use flexstr::", stringify!($name), ";")]
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
        #[doc = concat!("use flexstr::{", stringify!($name2), ", ", stringify!($name), ", To", stringify!($name2) ,"};")]
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
        #[doc = concat!("use flexstr::{", stringify!($name), ", To", stringify!($name) ,"};")]
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
        #[doc = concat!("use flexstr::{", stringify!($name), ", To", stringify!($name) ,"};")]
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
        #[doc = concat!("use flexstr::", stringify!($name), ";")]
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
                        // TODO: Any more efficient way to do this?
                        // Would like to use `from_raw` and `into_raw`, but need to ensure
                        // exclusive ownership for this to be safe. For `Rc` that might be possible,
                        // but `Arc` could be multi-threaded so needs to be atomic
                        $name::RefCounted(rc.deref().into())
                    }
                }
            }
        }

        #[doc = concat!("Converts a `String` into a `", stringify!($name), "`")]
        /// ```
        #[doc = concat!("use flexstr::", stringify!($name), ";")]
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
                    Err(s) => $name::RefCounted(s.into()),
                }
            }
        }

        #[doc = concat!("Converts a `&String` into a `", stringify!($name), "`")]
        /// ```
        #[doc = concat!("use flexstr::", stringify!($name), ";")]
        ///
        /// let lit = "inlined";
        #[doc = concat!("let s: ", stringify!($name), " = (&lit.to_string()).into();")]
        /// assert!(s.is_inlined());
        /// assert_eq!(&s, lit);
        ///
        /// let lit = "This is too long too be inlined!";
        #[doc = concat!("let s: ", stringify!($name), " = (&lit.to_string()).into();")]
        /// assert!(s.is_ref_counted());
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
        #[doc = concat!("use flexstr::", stringify!($name), ";")]
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

// *** FlexStr ***

flexstr!(
    FlexStr,
    AFlexStr,
    Rc<str>,
    Arc<str>,
    to_flexstr,
    to_a_flexstr,
    FlexStrVisitor
);

/// A trait that converts the source to a `FlexStr` without consuming it
pub trait ToFlexStr {
    /// Converts the source to a `FlexStr` without consuming it
    fn to_flexstr(&self) -> FlexStr;
}

impl ToFlexStr for str {
    #[inline]
    fn to_flexstr(&self) -> FlexStr {
        match self.try_into() {
            Ok(s) => FlexStr::Inlined(s),
            Err(_) => FlexStr::RefCounted(self.into()),
        }
    }
}

/// A trait that converts the source to a `FlexStr` while consuming the original
pub trait IntoFlexStr {
    /// Converts the source to a `FlexStr` while consuming the original
    fn into_flexstr(self) -> FlexStr;
}

impl IntoFlexStr for &'static str {
    #[inline]
    fn into_flexstr(self) -> FlexStr {
        self.into()
    }
}

impl IntoFlexStr for String {
    #[inline]
    fn into_flexstr(self) -> FlexStr {
        self.into()
    }
}

/// `FlexStr` equivalent to `format` function from stdlib. Efficiently creates a native `FlexStr`
pub fn format(args: Arguments<'_>) -> FlexStr {
    // NOTE: We have a disadvantage to `String` because we cannot call `estimated_capacity()`
    // As such, start by assuming this might be inlined and then promote buffer sizes as needed
    let mut builder = build::FlexStrBuilder::Small(build::StringBuffer::new());
    builder
        .write_fmt(args)
        .expect("a formatting trait implementation returned an error");
    builder.into_flexstr()
}

/// `FlexStr` equivalent to `format!` macro from stdlib. Efficiently creates a native `FlexStr`
#[macro_export]
macro_rules! flex_fmt {
    ($($arg:tt)*) => {
        format::format(format_args!($($arg)*))
    };
}

// *** AFlexStr ***

flexstr!(
    AFlexStr,
    FlexStr,
    Arc<str>,
    Rc<str>,
    to_a_flexstr,
    to_flexstr,
    AFlexStrVisitor
);

/// A trait that converts the source to an `AFlexStr` without consuming it
pub trait ToAFlexStr {
    /// Converts the source to an `AFlexStr` without consuming it
    fn to_a_flexstr(&self) -> AFlexStr;
}

impl ToAFlexStr for str {
    #[inline]
    fn to_a_flexstr(&self) -> AFlexStr {
        match self.try_into() {
            Ok(s) => AFlexStr::Inlined(s),
            Err(_) => AFlexStr::RefCounted(self.into()),
        }
    }
}

/// A trait that converts the source to an `AFlexStr` while consuming the original
pub trait IntoAFlexStr {
    /// Converts the source to an `AFlexStr` while consuming the original
    fn into_a_flexstr(self) -> AFlexStr;
}

impl IntoAFlexStr for &'static str {
    #[inline]
    fn into_a_flexstr(self) -> AFlexStr {
        self.into()
    }
}

impl IntoAFlexStr for String {
    #[inline]
    fn into_a_flexstr(self) -> AFlexStr {
        self.into()
    }
}

/// `AFlexStr` equivalent to `format` function from stdlib. Efficiently creates a native `AFlexStr`
pub fn a_format(args: Arguments<'_>) -> AFlexStr {
    // NOTE: We have a disadvantage to `String` because we cannot call `estimated_capacity()`
    // As such, start by assuming this might be inlined and then promote buffer sizes as needed
    let mut builder = build::FlexStrBuilder::Small(build::StringBuffer::new());
    builder
        .write_fmt(args)
        .expect("a formatting trait implementation returned an error");
    builder.into_a_flexstr()
}

/// `AFlexStr` equivalent to `format!` macro from stdlib. Efficiently creates a native `AFlexStr`
#[macro_export]
macro_rules! a_flex_fmt {
    ($($arg:tt)*) => {
        format::a_format(format_args!($($arg)*))
    };
}

#[cfg(test)]
mod tests {}
