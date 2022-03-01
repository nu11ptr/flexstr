#![no_std]
#![warn(missing_docs)]

//! A flexible, simple to use, immutable, clone-efficient `String` replacement for Rust

extern crate alloc;

mod builder;
mod inline;
mod traits;

pub use traits::*;

use alloc::rc::Rc;
use alloc::string::String;
use alloc::sync::Arc;
use core::borrow::Borrow;
use core::cmp::Ordering;
use core::convert::Infallible;
use core::fmt;
use core::fmt::{Arguments, Debug, Display, Formatter, Write};
use core::hash::{Hash, Hasher};
#[cfg(feature = "serde")]
use core::marker::PhantomData;
use core::ops::{
    Add, Deref, Index, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};
use core::str::FromStr;

#[cfg(feature = "serde")]
use serde::de::{Error, Visitor};
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone)]
enum FlexStrInner<T> {
    /// A wrapped string literal
    Static(&'static str),
    /// An inlined string
    Inlined(inline::InlineFlexStr),
    /// A reference count wrapped `str`
    Heap(T),
}

/// A flexible string type that transparently wraps a string literal, inline string, or an `Rc<str>`
#[derive(Clone)]
pub struct FlexStr<T = Rc<str>>(FlexStrInner<T>);

/// A flexible string type that transparently wraps a string literal, inline string, or an `Arc<str>`
pub type AFlexStr = FlexStr<Arc<str>>;

/// A flexible string type that transparently wraps a string literal, inline string, or a `String`
pub type WFlexStr = FlexStr<String>;

impl<T> FlexStr<T>
where
    T: Deref<Target = str>,
{
    /// Returns true if this `FlexStr` is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.deref().is_empty()
    }

    /// Returns the length of this `FlexStr` in bytes (not chars/graphemes)
    #[inline]
    pub fn len(&self) -> usize {
        self.deref().len()
    }

    /// Extracts a string slice containing the entire `FlexStr`
    #[inline]
    pub fn as_str(&self) -> &str {
        self.deref()
    }

    /// Returns true if this is a wrapped string literal (`&'static str`)
    #[inline]
    pub fn is_static(&self) -> bool {
        matches!(self.0, FlexStrInner::Static(_))
    }

    /// Returns true if this is an inlined string
    #[inline]
    pub fn is_inlined(&self) -> bool {
        matches!(self.0, FlexStrInner::Inlined(_))
    }

    /// Returns true if this is a wrapped string using heap storage
    #[inline]
    pub fn is_heap(&self) -> bool {
        matches!(self.0, FlexStrInner::Heap(_))
    }
}

// *** Deref / Debug / Display ***

impl<T> Deref for FlexStr<T>
where
    T: Deref<Target = str>,
{
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        match &self.0 {
            FlexStrInner::Static(str) => str,
            FlexStrInner::Inlined(ss) => ss,
            FlexStrInner::Heap(rc) => rc,
        }
    }
}

impl<T> Debug for FlexStr<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self.deref(), f)
    }
}

impl<T> Display for FlexStr<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self.deref(), f)
    }
}

// *** Hash, PartialEq, Eq ***

impl<T> Hash for FlexStr<T>
where
    T: Deref<Target = str>,
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(self.deref(), state)
    }
}

/// ```
/// use flexstr::{AFlexStr, FlexStr, ToFlex};
///
/// let lit = "inlined";
/// let s: FlexStr = lit.into();
/// let s2: AFlexStr = lit.into();
/// assert_eq!(s, s2);
/// ```
impl<T, U> PartialEq<FlexStr<U>> for FlexStr<T>
where
    T: Deref<Target = str>,
    U: Deref<Target = str>,
{
    fn eq(&self, other: &FlexStr<U>) -> bool {
        PartialEq::eq(&self.deref(), &other.deref())
    }
}

/// ```
/// use flexstr::{FlexStr, ToFlex};
///
/// let lit = "inlined";
/// let s: FlexStr = lit.to_flex();
/// assert_eq!(s, lit);
/// ```
impl<T> PartialEq<&str> for FlexStr<T>
where
    T: Deref<Target = str>,
{
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        PartialEq::eq(&self.deref(), &other.deref())
    }
}

/// ```
/// use flexstr::{FlexStr, ToFlex};
///
/// let lit = "inlined";
/// let s: FlexStr = lit.to_flex();
/// assert_eq!(s, lit);
/// ```
impl<T> PartialEq<str> for FlexStr<T>
where
    T: Deref<Target = str>,
{
    #[inline]
    fn eq(&self, other: &str) -> bool {
        PartialEq::eq(&self.deref(), &other.deref())
    }
}

/// ```
/// use flexstr::FlexStr;
///
/// let lit = "inlined";
/// let s: FlexStr = lit.into();
/// assert_eq!(s, lit.to_string());
/// ```
impl<T> PartialEq<String> for FlexStr<T>
where
    T: Deref<Target = str>,
{
    #[inline]
    fn eq(&self, other: &String) -> bool {
        PartialEq::eq(&self.deref(), &other.deref())
    }
}

impl<T> Eq for FlexStr<T> where T: Deref<Target = str> {}

// *** PartialOrd / Ord ***

impl<T> PartialOrd for FlexStr<T>
where
    T: Deref<Target = str>,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.deref(), &other.deref())
    }
}

impl<T> PartialOrd<str> for FlexStr<T>
where
    T: Deref<Target = str>,
{
    #[inline]
    fn partial_cmp(&self, other: &str) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.deref(), &other.deref())
    }
}

impl<T> PartialOrd<String> for FlexStr<T>
where
    T: Deref<Target = str>,
{
    #[inline]
    fn partial_cmp(&self, other: &String) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.deref(), &other.deref())
    }
}

impl<T> Ord for FlexStr<T>
where
    T: Deref<Target = str>,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&self.deref(), &other.deref())
    }
}

// *** Index ***

impl<T> Index<Range<usize>> for FlexStr<T>
where
    T: Deref<Target = str>,
{
    type Output = str;

    #[inline]
    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.deref()[index]
    }
}

impl<T> Index<RangeTo<usize>> for FlexStr<T>
where
    T: Deref<Target = str>,
{
    type Output = str;

    #[inline]
    fn index(&self, index: RangeTo<usize>) -> &Self::Output {
        &self.deref()[index]
    }
}

impl<T> Index<RangeFrom<usize>> for FlexStr<T>
where
    T: Deref<Target = str>,
{
    type Output = str;

    #[inline]
    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        &self.deref()[index]
    }
}

impl<T> Index<RangeFull> for FlexStr<T>
where
    T: Deref<Target = str>,
{
    type Output = str;

    #[inline]
    fn index(&self, _index: RangeFull) -> &Self::Output {
        self.deref()
    }
}

impl<T> Index<RangeInclusive<usize>> for FlexStr<T>
where
    T: Deref<Target = str>,
{
    type Output = str;

    #[inline]
    fn index(&self, index: RangeInclusive<usize>) -> &Self::Output {
        &self.deref()[index]
    }
}

impl<T> Index<RangeToInclusive<usize>> for FlexStr<T>
where
    T: Deref<Target = str>,
{
    type Output = str;

    #[inline]
    fn index(&self, index: RangeToInclusive<usize>) -> &Self::Output {
        &self.deref()[index]
    }
}

// *** Add ***

// TODO: Is there value in making this a public macro with varargs? Hmm...
fn concat<T>(s1: &str, s2: &str) -> FlexStr<T>
where
    T: From<String> + for<'a> From<&'a str>,
{
    let mut builder = builder::FlexStrBuilder::with_capacity(s1.len() + s2.len());
    unsafe {
        // Safety: write_str always succeeds
        builder.write_str(s1).unwrap_unchecked();
        builder.write_str(s2).unwrap_unchecked();
    }
    builder.into()
}

impl<T> Add<&str> for FlexStr<T>
where
    T: From<String> + for<'a> From<&'a str> + Deref<Target = str>,
{
    type Output = FlexStr<T>;

    #[inline]
    fn add(self, rhs: &str) -> Self::Output {
        match self.0 {
            FlexStrInner::Static(s) => concat(s, rhs),
            FlexStrInner::Inlined(mut s) => {
                if s.try_concat(rhs) {
                    FlexStr(FlexStrInner::Inlined(s))
                } else {
                    concat(&s, rhs)
                }
            }
            FlexStrInner::Heap(s) => concat(&s, rhs),
        }
    }
}

// *** Misc. standard traits ***

impl<T> AsRef<str> for FlexStr<T>
where
    T: Deref<Target = str>,
{
    #[inline]
    fn as_ref(&self) -> &str {
        self
    }
}

impl<T> Default for FlexStr<T> {
    #[inline]
    fn default() -> Self {
        FlexStr(FlexStrInner::Static(""))
    }
}

impl<T> Borrow<str> for FlexStr<T>
where
    T: Deref<Target = str>,
{
    #[inline]
    fn borrow(&self) -> &str {
        self.deref()
    }
}

impl<T> FromStr for FlexStr<T>
where
    T: for<'a> From<&'a str>,
{
    type Err = Infallible;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.to_flex())
    }
}

// *** From ***

impl<T, U> From<&FlexStr<U>> for FlexStr<T>
where
    U: Clone,
    FlexStr<T>: From<FlexStr<U>>,
{
    #[inline]
    fn from(s: &FlexStr<U>) -> Self {
        s.clone().into()
    }
}

impl<T> From<builder::FlexStrBuilder> for FlexStr<T>
where
    T: From<String> + for<'a> From<&'a str>,
{
    #[inline]
    fn from(builder: builder::FlexStrBuilder) -> Self {
        match builder {
            builder::FlexStrBuilder::Small(buffer) => {
                let len: u8 = buffer.len() as u8;
                FlexStr(FlexStrInner::Inlined(inline::InlineFlexStr::from_array(
                    buffer.into_inner(),
                    len,
                )))
            }
            builder::FlexStrBuilder::Regular(buffer) => buffer.to_flex(),
            builder::FlexStrBuilder::Large(s) => s.into(),
        }
    }
}

/// Converts a `String` into a `FlexStr`
/// ```
/// use flexstr::FlexStr;
///
/// let lit = "inlined";
/// let s: FlexStr = lit.to_string().into();
/// assert!(s.is_inlined());
/// assert_eq!(&s, lit);
///
/// let lit = "This is too long too be inlined!";
/// let s: FlexStr = lit.to_string().into();
/// assert!(s.is_heap());
/// assert_eq!(&s, lit);
/// ```
impl<T> From<String> for FlexStr<T>
where
    T: From<String>,
{
    #[inline]
    fn from(s: String) -> Self {
        FlexStr(match s.try_into() {
            Ok(s) => FlexStrInner::Inlined(s),
            Err(s) => FlexStrInner::Heap(s.into()),
        })
    }
}

/// Converts a `&String` into a `FlexStr`
/// ```
/// use flexstr::FlexStr;
///
/// let lit = "inlined";
/// let s: FlexStr = (&lit.to_string()).into();
/// assert!(s.is_inlined());
/// assert_eq!(&s, lit);
///
/// let lit = "This is too long too be inlined!";
/// let s: FlexStr = (&lit.to_string()).into();
/// assert!(s.is_heap());
/// assert_eq!(&s, lit);
/// ```
impl<T> From<&String> for FlexStr<T>
where
    T: for<'a> From<&'a str>,
{
    #[inline]
    fn from(s: &String) -> Self {
        s.to_flex()
    }
}

/// Converts a string literal (`&static str`) into a `FlexStr`
/// ```
/// use flexstr::FlexStr;
///
/// let lit = "static";
/// let s: FlexStr  = lit.into();
/// assert!(s.is_static());
/// assert_eq!(&s, lit);
/// ```
impl<T> From<&'static str> for FlexStr<T> {
    #[inline]
    fn from(s: &'static str) -> Self {
        FlexStr(FlexStrInner::Static(s))
    }
}

// *** ToCase custom trait ***

/// Trait that provides uppercase/lowercase conversion functions for `FlexStr`
pub trait ToCase<T> {
    /// Converts string to uppercase and returns a `FlexStr`
    fn to_uppercase(&self) -> FlexStr<T>;

    /// Converts string to lowercase and returns a `FlexStr`
    fn to_lowercase(&self) -> FlexStr<T>;

    /// Converts string to ASCII uppercase and returns a `FlexStr`
    fn to_ascii_uppercase(&self) -> FlexStr<T>;

    /// Converts string to ASCII lowercase and returns a `FlexStr`
    fn to_ascii_lowercase(&self) -> FlexStr<T>;
}

impl<T> ToCase<T> for str
where
    T: From<String> + for<'a> From<&'a str>,
{
    fn to_uppercase(&self) -> FlexStr<T> {
        // We estimate capacity based on previous string, but if not ASCII this might be wrong
        let mut builder = builder::FlexStrBuilder::with_capacity(self.len());

        for ch in self.chars() {
            let upper_chars = ch.to_uppercase();
            for ch in upper_chars {
                // Safety: Wraps `write_str` which always succeeds
                unsafe { builder.write_char(ch).unwrap_unchecked() }
            }
        }

        builder.into()
    }

    fn to_lowercase(&self) -> FlexStr<T> {
        // We estimate capacity based on previous string, but if not ASCII this might be wrong
        let mut builder = builder::FlexStrBuilder::with_capacity(self.len());

        for ch in self.chars() {
            let lower_chars = ch.to_lowercase();
            for ch in lower_chars {
                // Safety: Wraps `write_str` which always succeeds
                unsafe { builder.write_char(ch).unwrap_unchecked() }
            }
        }

        builder.into()
    }

    fn to_ascii_uppercase(&self) -> FlexStr<T> {
        let mut builder = builder::FlexStrBuilder::with_capacity(self.len());

        for mut ch in self.chars() {
            char::make_ascii_uppercase(&mut ch);
            // Safety: Wraps `write_str` which always succeeds
            unsafe { builder.write_char(ch).unwrap_unchecked() }
        }

        builder.into()
    }

    fn to_ascii_lowercase(&self) -> FlexStr<T> {
        let mut builder = builder::FlexStrBuilder::with_capacity(self.len());

        for mut ch in self.chars() {
            char::make_ascii_lowercase(&mut ch);
            // Safety: Wraps `write_str` which always succeeds
            unsafe { builder.write_char(ch).unwrap_unchecked() }
        }

        builder.into()
    }
}

// *** Optional serialization support ***

#[cfg(feature = "serde")]
impl<T> Serialize for FlexStr<T>
where
    T: Deref<Target = str>,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self)
    }
}

// Uses *const T because we don't want it to actually own a `T`
#[cfg(feature = "serde")]
struct FlexStrVisitor<T>(PhantomData<*const T>);

#[cfg(feature = "serde")]
impl<'de, T> Visitor<'de> for FlexStrVisitor<T>
where
    T: From<String> + for<'a> From<&'a str>,
{
    type Value = FlexStr<T>;

    #[inline]
    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }

    #[inline]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v.to_flex())
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
impl<'de, T> Deserialize<'de> for FlexStr<T>
where
    T: From<String> + for<'a> From<&'a str>,
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(FlexStrVisitor(PhantomData))
    }
}

/// `FlexStr` equivalent to `format` function from stdlib. Efficiently creates a native `FlexStr`
pub fn flex_fmt<T>(args: Arguments<'_>) -> FlexStr<T>
where
    T: From<String> + for<'a> From<&'a str>,
{
    // NOTE: We have a disadvantage to `String` because we cannot call `estimated_capacity()` on args
    // As such, we cannot assume a given needed capacity - we start with a stack allocated buffer
    // and only promote to a heap buffer if a write won't fit
    let mut builder = builder::FlexStrBuilder::new();
    builder
        .write_fmt(args)
        .expect("a formatting trait implementation returned an error");
    builder.into_flex()
}

/// `FlexStr` equivalent to `format!` macro from stdlib. Efficiently creates a native `FlexStr`
#[macro_export]
macro_rules! flex_fmt {
    ($($arg:tt)*) => {
        flex_fmt::<FlexStr>(format_args!($($arg)*))
    };
}

/// `AFlexStr` equivalent to `format!` macro from stdlib. Efficiently creates a native `AFlexStr`
#[macro_export]
macro_rules! a_flex_fmt {
    ($($arg:tt)*) => {
        flex_fmt::<AFlexStr>(format_args!($($arg)*))
    };
}

#[cfg(test)]
mod tests {}
