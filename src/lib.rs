#![no_std]
#![warn(missing_docs)]

//! A flexible, simple to use, immutable, clone-efficient `String` replacement for Rust

extern crate alloc;

mod builder;
mod inline;

use alloc::rc::Rc;
use alloc::string::String;
use alloc::sync::Arc;
use core::borrow::Borrow;
use core::cmp::Ordering;
use core::convert::Infallible;
use core::fmt;
use core::fmt::{Arguments, Debug, Display, Formatter, Write};
use core::hash::{Hash, Hasher};
use core::ops::{
    Add, Deref, Index, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};
use core::str::FromStr;

use paste::paste;
#[cfg(feature = "serde")]
use serde::de::{Error, Visitor};
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// *** FlexStr macro ***

macro_rules! flexstr {
    ($name:ident, $name2:ident, $rc:ty, $lower_name:ident, $lower_name2:ident) => {
        paste! {
            #[derive(Clone)]
            enum [<$name Inner>] {
                /// A wrapped string literal
                Static(&'static str),
                /// An inlined string
                Inlined(inline::InlineFlexStr),
                /// A reference count wrapped `str`
                RefCounted($rc),
            }

            impl $name {
                #[doc = "Returns true if this `" $name "` is empty"]
                #[inline]
                pub fn is_empty(&self) -> bool {
                    self.deref().is_empty()
                }

                #[doc = "Returns the length of this `" $name "` in bytes (not chars/graphemes)"]
                #[inline]
                pub fn len(&self) -> usize {
                    self.deref().len()
                }

                #[doc = "Extracts a string slice containing the entire `" $name "`"]
                #[inline]
                pub fn as_str(&self) -> &str {
                    self.deref()
                }

                /// Returns true if this is a wrapped string literal (`&'static str`)
                #[inline]
                pub fn is_static(&self) -> bool {
                    matches!(self.0, [<$name Inner>]::Static(_))
                }

                /// Returns true if this is an inlined string
                #[inline]
                pub fn is_inlined(&self) -> bool {
                    matches!(self.0, [<$name Inner>]::Inlined(_))
                }

                /// Returns true if this is a wrapped `str` using reference counting
                #[inline]
                pub fn is_ref_counted(&self) -> bool {
                    matches!(self.0, [<$name Inner>]::RefCounted(_))
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
            #[doc = "use flexstr::" $name ";"]
            ///
            #[doc = "let s: " $name " = \"inlined\".into();"]
            #[doc = "let s2: " $name " = s.clone();"]
            /// assert_eq!(s, s2);
            /// ```
            impl PartialEq for $name {
                #[inline]
                fn eq(&self, other: &Self) -> bool {
                    <&str as PartialEq>::eq(&self.deref(), &other.deref())
                }
            }

            /// ```
            #[doc = "use flexstr::{" $name2 ", " $name ", To" $name2 "};"]
            ///
            #[doc = "let s: " $name " = \"inlined\".into();"]
            #[doc = "let s2: " $name2 " = s.to_" $lower_name2 "();"]
            /// assert_eq!(s, s2);
            /// ```
            impl PartialEq<$name2> for $name {
                fn eq(&self, other: &$name2) -> bool {
                    <&str as PartialEq>::eq(&self.deref(), &other.deref())
                }
            }

            /// ```
            #[doc = "use flexstr::{" $name ", To" $name "};"]
            ///
            /// let lit = "inlined";
            #[doc = "let s: " $name " = lit.to_" $lower_name "();"]
            /// assert_eq!(s, lit);
            /// ```
            impl PartialEq<&str> for $name {
                #[inline]
                fn eq(&self, other: &&str) -> bool {
                    <&str as PartialEq>::eq(&self.deref(), &other.deref())
                }
            }

            /// ```
            #[doc = "use flexstr::{" $name ", To" $name "};"]
            ///
            /// let lit = "inlined";
            #[doc = "let s: " $name " = lit.to_" $lower_name "();"]
            /// assert_eq!(s, lit);
            /// ```
            impl PartialEq<str> for $name {
                #[inline]
                fn eq(&self, other: &str) -> bool {
                    <&str as PartialEq>::eq(&self.deref(), &other.deref())
                }
            }

            /// ```
            #[doc = "use flexstr::" $name ";"]
            ///
            /// let lit = "inlined";
            #[doc = "let s: " $name " = lit.into();"]
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

            // *** Deref / Debug / Display ***

            impl Deref for $name {
                type Target = str;

                #[inline]
                fn deref(&self) -> &Self::Target {
                    match &self.0 {
                        [<$name Inner>]::Static(str) => str,
                        [<$name Inner>]::Inlined(ss) => ss,
                        [<$name Inner>]::RefCounted(rc) => rc,
                    }
                }
            }

            impl Debug for $name {
                #[inline]
                fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                    Debug::fmt(self.deref(), f)
                }
            }

            impl Display for $name {
                #[inline]
                fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                    Display::fmt(self.deref(), f)
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
                    $name(match s.0 {
                        [<$name2 Inner>]::Static(s) => [<$name Inner>]::Static(s),
                        [<$name2 Inner>]::Inlined(s) => [<$name Inner>]::Inlined(s),
                        [<$name2 Inner>]::RefCounted(rc) => {
                            // TODO: Any more efficient way to do this?
                            // Would like to use `from_raw` and `into_raw`, but need to ensure
                            // exclusive ownership for this to be safe. For `Rc` that might be possible,
                            // but `Arc` could be multi-threaded so needs to be atomic
                            [<$name Inner>]::RefCounted(rc.deref().into())
                        }
                    })
                }
            }

            impl From<builder::FlexStrBuilder> for $name {
                #[inline]
                fn from(builder: builder::FlexStrBuilder) -> Self {
                    match builder {
                        builder::FlexStrBuilder::Small(buffer) => {
                            let len: u8 = buffer.len() as u8;
                            $name([<$name Inner>]::Inlined(inline::InlineFlexStr::from_array(
                                buffer.into_inner(),
                                len,
                            )))
                        }
                        builder::FlexStrBuilder::Regular(buffer) => buffer.[<to_ $lower_name>](),
                        builder::FlexStrBuilder::Large(s) => s.into(),
                    }
                }
            }

            #[doc = "Converts a `String` into a `" $name "`"]
            /// ```
            #[doc = "use flexstr::" $name ";"]
            ///
            /// let lit = "inlined";
            #[doc = "let s: " $name " = lit.to_string().into();"]
            /// assert!(s.is_inlined());
            /// assert_eq!(&s, lit);
            ///
            /// let lit = "This is too long too be inlined!";
            #[doc = "let s: " $name " = lit.to_string().into();"]
            /// assert!(s.is_ref_counted());
            /// assert_eq!(&s, lit);
            /// ```
            impl From<String> for $name {
                #[inline]
                fn from(s: String) -> Self {
                    $name(match s.try_into() {
                        Ok(s) => [<$name Inner>]::Inlined(s),
                        Err(s) => [<$name Inner>]::RefCounted(s.into()),
                    })
                }
            }

            #[doc = "Converts a `&String` into a `" $name "`"]
            /// ```
            #[doc = "use flexstr::" $name ";"]
            ///
            /// let lit = "inlined";
            #[doc = "let s: " $name " = (&lit.to_string()).into();"]
            /// assert!(s.is_inlined());
            /// assert_eq!(&s, lit);
            ///
            /// let lit = "This is too long too be inlined!";
            #[doc = "let s: " $name " = (&lit.to_string()).into();"]
            /// assert!(s.is_ref_counted());
            /// assert_eq!(&s, lit);
            /// ```
            impl From<&String> for $name {
                #[inline]
                fn from(s: &String) -> Self {
                    s.[<to_ $lower_name>]()
                }
            }

            #[doc = "Converts a string literal (`&static str`) into a `" $name "`"]
            /// ```
            #[doc = "use flexstr::" $name ";"]
            ///
            /// let lit = "static";
            #[doc = "let s: " $name " = lit.into();"]
            /// assert!(s.is_static());
            /// assert_eq!(&s, lit);
            /// ```
            impl From<&'static str> for $name {
                #[inline]
                fn from(s: &'static str) -> Self {
                    $name([<$name Inner>]::Static(s))
                }
            }

            // *** Index ***

            impl Index<Range<usize>> for $name {
                type Output = str;

                #[inline]
                fn index(&self, index: Range<usize>) -> &Self::Output {
                    &self.deref()[index]
                }
            }

            impl Index<RangeTo<usize>> for $name {
                type Output = str;

                #[inline]
                fn index(&self, index: RangeTo<usize>) -> &Self::Output {
                    &self.deref()[index]
                }
            }

            impl Index<RangeFrom<usize>> for $name {
                type Output = str;

                #[inline]
                fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
                    &self.deref()[index]
                }
            }

            impl Index<RangeFull> for $name {
                type Output = str;

                #[inline]
                fn index(&self, _index: RangeFull) -> &Self::Output {
                    self.deref()
                }
            }

            impl Index<RangeInclusive<usize>> for $name {
                type Output = str;

                #[inline]
                fn index(&self, index: RangeInclusive<usize>) -> &Self::Output {
                    &self.deref()[index]
                }
            }

            impl Index<RangeToInclusive<usize>> for $name {
                type Output = str;

                #[inline]
                fn index(&self, index: RangeToInclusive<usize>) -> &Self::Output {
                    &self.deref()[index]
                }
            }

            // *** Add ***

            // TODO: Is there value in making this a public macro with varargs? Hmm...
            fn [<$lower_name _concat>](s1: &str, s2: &str) -> $name {
                let mut builder = builder::FlexStrBuilder::with_capacity(s1.len() + s2.len());
                unsafe {
                    // Safety: write_str always succeeds
                    builder.write_str(s1).unwrap_unchecked();
                    builder.write_str(s2).unwrap_unchecked();
                }
                builder.[<into_ $lower_name>]()
            }

            impl Add<&str> for $name {
                type Output = $name;

                #[inline]
                fn add(self, rhs: &str) -> Self::Output {
                    match self.0 {
                        [<$name Inner>]::Static(s) => [<$lower_name _concat>](s, rhs),
                        [<$name Inner>]::Inlined(mut s) => {
                            if s.try_concat(rhs) {
                                $name([<$name Inner>]::Inlined(s))
                            } else {
                                [<$lower_name _concat>](&s, rhs)
                            }
                        }
                        [<$name Inner>]::RefCounted(s) => [<$lower_name _concat>](&s, rhs),
                    }
                }
            }

            // *** Misc. standard traits ***

            impl AsRef<str> for $name {
                #[inline]
                fn as_ref(&self) -> &str {
                    self
                }
            }

            impl Default for $name {
                #[inline]
                fn default() -> Self {
                    $name([<$name Inner>]::Static(""))
                }
            }

            impl Borrow<str> for $name {
                #[inline]
                fn borrow(&self) -> &str {
                    self.deref()
                }
            }

            impl FromStr for $name {
                type Err = Infallible;

                #[inline]
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    Ok(s.[<to_ $lower_name>]())
                }
            }

            // *** To/Into Custom Traits ***

            #[doc = "A trait that converts the source to a `" $name "` without consuming it"]
            pub trait [<To $name>] {
                #[doc = "Converts the source to a `" $name "` without consuming it"]
                fn [<to_ $lower_name>](&self) -> $name;
            }

            impl [<To $name>] for str {
                #[inline]
                fn [<to_ $lower_name>](&self) -> $name {
                    $name(match self.try_into() {
                        Ok(s) => [<$name Inner>]::Inlined(s),
                        Err(_) => [<$name Inner>]::RefCounted(self.into()),
                    })
                }
            }

            #[doc = "A trait that converts the source to a `" $name "` while consuming the original"]
            pub trait [<Into $name>] {
                #[doc = "Converts the source to a `" $name "` while consuming the original"]
                fn [<into_ $lower_name>](self) -> $name;
            }

            impl [<Into $name>] for &'static str {
                #[inline]
                fn [<into_ $lower_name>](self) -> $name{
                    self.into()
                }
            }

            impl [<Into $name>] for String {
                #[inline]
                fn [<into_ $lower_name>](self) -> $name {
                    self.into()
                }
            }

            impl [<Into $name>] for builder::FlexStrBuilder {
                #[inline]
                fn [<into_ $lower_name>](self) -> $name {
                    self.into()
                }
            }

            // *** Optional serialization support ***

            #[cfg(feature = "serde")]
            impl Serialize for $name {
                #[inline]
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    serializer.serialize_str(self)
                }
            }

            #[cfg(feature = "serde")]
            struct [<$name Visitor>];

            #[cfg(feature = "serde")]
            impl<'de> Visitor<'de> for [<$name Visitor>] {
                type Value = $name;

                #[inline]
                fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                    formatter.write_str("a string")
                }

                #[inline]
                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    Ok(v.[<to_ $lower_name>]())
                }

                #[inline]
                fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    Ok(v.into())
                }
            }

            #[cfg(feature = "serde")]
            impl<'de> Deserialize<'de> for $name {
                #[inline]
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    deserializer.deserialize_str([<$name Visitor>])
                }
            }
        }
    };
}

// *** FlexStr ***

/// A Flexible string type that transparently wraps a string literal, inline string, or an `Rc<str>`
#[derive(Clone)]
pub struct FlexStr(FlexStrInner);

// TODO: If we stick with these lower names, we can use paste's :lower feature instead
flexstr!(FlexStr, AFlexStr, Rc<str>, flex_str, a_flex_str);

/// `FlexStr` equivalent to `format` function from stdlib. Efficiently creates a native `FlexStr`
pub fn flex_fmt(args: Arguments<'_>) -> FlexStr {
    // NOTE: We have a disadvantage to `String` because we cannot call `estimated_capacity()` on args
    // As such, we cannot assume a given needed capacity - we start with a stack allocated buffer
    // and only promote to a heap buffer if a write won't fit
    let mut builder = builder::FlexStrBuilder::new();
    builder
        .write_fmt(args)
        .expect("a formatting trait implementation returned an error");
    builder.into_flex_str()
}

/// `FlexStr` equivalent to `format!` macro from stdlib. Efficiently creates a native `FlexStr`
#[macro_export]
macro_rules! flex_fmt {
    ($($arg:tt)*) => {
        flex_fmt(format_args!($($arg)*))
    };
}

// *** AFlexStr ***

/// A flexible string type that transparently wraps a string literal, inline string, or an `Arc<str>`
#[derive(Clone)]
pub struct AFlexStr(AFlexStrInner);

// TODO: If we stick with these lower names, we can use paste's :lower feature instead
flexstr!(AFlexStr, FlexStr, Arc<str>, a_flex_str, flex_str);

/// `AFlexStr` equivalent to `format` function from stdlib. Efficiently creates a native `AFlexStr`
pub fn a_flex_fmt(args: Arguments<'_>) -> AFlexStr {
    // NOTE: We have a disadvantage to `String` because we cannot call `estimated_capacity()` on args
    // As such, we cannot assume a given needed capacity - we start with a stack allocated buffer
    // and only promote to a heap buffer if a write won't fit
    let mut builder = builder::FlexStrBuilder::new();
    builder
        .write_fmt(args)
        .expect("a formatting trait implementation returned an error");
    builder.into_a_flex_str()
}

/// `AFlexStr` equivalent to `format!` macro from stdlib. Efficiently creates a native `AFlexStr`
#[macro_export]
macro_rules! a_flex_fmt {
    ($($arg:tt)*) => {
        a_flex_fmt(format_args!($($arg)*))
    };
}

#[cfg(test)]
mod tests {}
