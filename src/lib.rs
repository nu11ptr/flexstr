#![no_std]
#![warn(missing_docs)]

//! A flexible, simple to use, immutable, clone-efficient `String` replacement for Rust
//!
//! ```
//! use flexstr::{flex_fmt, FlexStr, IntoFlexStr, ToCase, ToFlexStr};
//!
//! // Use an `into` function to wrap a literal, no allocation or copying
//! let static_str = "This will not allocate or copy".into_flex_str();
//! assert!(static_str.is_static());
//!
//! // Strings up to 22 bytes (on 64-bit) will be inlined automatically
//! // (demo only, use `into` for literals as above)
//! let inline_str = "inlined".to_flex_str();
//! assert!(inline_str.is_inlined());
//!
//! // When a string is too long to be wrapped/inlined, it will heap allocate
//! // (demo only, use `into` for literals as above)
//! let rc_str = "This is too long to be inlined".to_flex_str();
//! assert!(rc_str.is_heap());
//!
//! // You can efficiently create a new `FlexStr` (without creating a `String`)
//! // This is equivalent to the stdlib `format!` macro
//! let inline_str2 = flex_fmt!("in{}", "lined");
//! assert!(inline_str2.is_inlined());
//! assert_eq!(inline_str, inline_str2);
//!
//! // We can upper/lowercase strings without converting to a `String` first
//! // This doesn't heap allocate since inlined
//! let inline_str3: FlexStr = "INLINED".to_ascii_lower();
//! assert!(inline_str3.is_inlined());
//! assert_eq!(inline_str, inline_str3);
//!
//! // Concatenation doesn't even copy if we can fit it in the inline string
//! let inline_str4 = inline_str3 + "!!!";
//! assert!(inline_str4.is_inlined());
//! assert_eq!(inline_str4, "inlined!!!");
//!
//! // Clone is almost free, and never allocates
//! // (at most it is a ref count increment for heap allocated strings)
//! let static_str2 = static_str.clone();
//! assert!(static_str2.is_static());
//!
//! // Regardless of storage type, these all operate seamlessly together
//! // and choose storage as required
//! let heap_str2 = static_str2 + &inline_str;
//! assert!(heap_str2.is_heap());
//! assert_eq!(heap_str2, "This will not allocate or copyinlined");  
//! ```

extern crate alloc;

#[macro_use]
mod builder;
mod inline;
mod traits;

pub use inline::STRING_SIZED_INLINE;
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

// Trick to test README samples (from: https://github.com/rust-lang/cargo/issues/383#issuecomment-720873790)
#[cfg(doctest)]
mod test_readme {
    macro_rules! external_doc_test {
        ($x:expr) => {
            #[doc = $x]
            extern "C" {}
        };
    }

    external_doc_test!(include_str!("../README.md"));
}

#[derive(Clone, Debug)]
enum FlexStrInner<const N: usize, T> {
    /// A wrapped string literal
    Static(&'static str),
    /// An inlined string
    Inlined(inline::InlineFlexStr<N>),
    /// A reference count wrapped `str`
    Heap(T),
}

/// A flexible string type that transparently wraps a string literal, inline string, or an `Rc<str>`
#[derive(Clone, Debug)]
pub struct FlexStr<const N: usize = STRING_SIZED_INLINE, T = Rc<str>>(FlexStrInner<N, T>);

/// A flexible string type that transparently wraps a string literal, inline string, or an `Arc<str>`
pub type AFlexStr = FlexStr<STRING_SIZED_INLINE, Arc<str>>;

impl<const N: usize, T> FlexStr<N, T> {
    /// Creates a wrapped static string literal. This function is equivalent to calling the `into`
    /// functions on a static string literal, but is `const fn` so can be used to init a constant.
    /// ```
    /// use flexstr::FlexStr;
    ///
    /// const S: FlexStr = <FlexStr>::from_static("test");
    /// assert!(S.is_static());
    /// ```
    #[inline]
    pub const fn from_static(s: &'static str) -> FlexStr<N, T> {
        FlexStr(FlexStrInner::Static(s))
    }
}

impl<const N: usize, T> FlexStr<N, T>
where
    T: Deref<Target = str>,
{
    /// Attempts to create an inlined string. Returns new inline string on success or original source
    /// string as `Err` if it will not fit. Since the to/into functions will automatically inline when
    /// possible, this function is really only for special use cases.
    /// ```
    /// use flexstr::FlexStr;
    ///
    /// let s = <FlexStr>::try_inline("test").unwrap();
    /// assert!(s.is_inlined());
    /// ```
    #[inline]
    pub fn try_inline(s: &str) -> Result<FlexStr<N, T>, &str> {
        match inline::InlineFlexStr::try_new(s) {
            Ok(s) => Ok(FlexStr(FlexStrInner::Inlined(s))),
            Err(s) => Err(s),
        }
    }

    /// Force the creation of a heap allocated string. Unlike to/into functions, this will not attempt
    /// to inline first even if the string is a candidate for inlining. Using this is generally not
    /// recommended, and the to/into conversion functions should be preferred.
    /// ```
    /// use flexstr::FlexStr;
    ///
    /// let s = <FlexStr>::heap("test");
    /// assert!(s.is_heap());
    /// ```
    #[inline]
    pub fn heap(s: &str) -> FlexStr<N, T>
    where
        T: for<'a> From<&'a str>,
    {
        FlexStr(FlexStrInner::Heap(s.into()))
    }

    /// Returns the size of the maximum possible inline length for this type
    #[inline]
    pub fn inline_capacity() -> usize {
        N
    }

    /// Returns true if this `FlexStr` is empty
    /// ```
    /// use flexstr::ToFlexStr;
    ///
    /// let inlined = "".to_flex_str();
    /// assert!(inlined.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        // Due to how inline does deref, I'm guessing this is slightly cheaper by using inline native is_empty
        match &self.0 {
            FlexStrInner::Static(str) => str.is_empty(),
            FlexStrInner::Inlined(s) => s.is_empty(),
            FlexStrInner::Heap(rc) => rc.is_empty(),
        }
    }

    /// Returns the length of this `FlexStr` in bytes (not chars/graphemes)
    /// ```
    /// use flexstr::ToFlexStr;
    ///
    /// let inlined = "len".to_flex_str();
    /// assert_eq!(inlined.len(), 3);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        // Due to how inline does deref, I'm guessing this is slightly cheaper by using inline native len
        match &self.0 {
            FlexStrInner::Static(str) => str.len(),
            FlexStrInner::Inlined(s) => s.len(),
            FlexStrInner::Heap(rc) => rc.len(),
        }
    }

    /// Extracts a string slice containing the entire `FlexStr`
    #[inline]
    pub fn as_str(&self) -> &str {
        &**self
    }

    /// Converts this `FlexStr` into a `String`. This should be more efficient than using the `ToString`
    /// trait (which we cannot implement due to a blanket stdlib implementation) as this avoids the
    /// `Display`-based implementation.
    /// ```
    /// use flexstr::IntoFlexStr;
    ///
    /// let s = "abc".into_flex_str().to_string();
    /// assert_eq!(s, "abc");
    /// ```
    #[allow(clippy::inherent_to_string_shadow_display)]
    #[inline]
    pub fn to_string(&self) -> String {
        String::from(&**self)
    }

    /// Returns true if this is a wrapped string literal (`&'static str`)
    /// ```
    /// use flexstr::FlexStr;
    ///
    /// let s = <FlexStr>::from_static("test");
    /// assert!(s.is_static());
    /// ```
    #[inline]
    pub fn is_static(&self) -> bool {
        matches!(self.0, FlexStrInner::Static(_))
    }

    /// Returns true if this is an inlined string
    /// ```
    /// use flexstr::FlexStr;
    ///
    /// let s = <FlexStr>::try_inline("test").unwrap();
    /// assert!(s.is_inlined());
    /// ```
    #[inline]
    pub fn is_inlined(&self) -> bool {
        matches!(self.0, FlexStrInner::Inlined(_))
    }

    /// Returns true if this is a wrapped string using heap storage
    /// ```
    /// use flexstr::FlexStr;
    ///
    /// let s = <FlexStr>::heap("test");
    /// assert!(s.is_heap());
    /// ```
    #[inline]
    pub fn is_heap(&self) -> bool {
        matches!(self.0, FlexStrInner::Heap(_))
    }
}

// *** Deref / Debug / Display ***

impl<const N: usize, T> Deref for FlexStr<N, T>
where
    T: Deref<Target = str>,
{
    type Target = str;

    /// ```
    /// use flexstr::IntoFlexStr;
    ///
    /// let a = "test";
    /// let b = a.into_flex_str();
    /// assert_eq!(&*b, a);
    /// ```
    #[inline]
    fn deref(&self) -> &Self::Target {
        match &self.0 {
            FlexStrInner::Static(str) => str,
            FlexStrInner::Inlined(ss) => ss,
            FlexStrInner::Heap(rc) => rc,
        }
    }
}

impl<const N: usize, T> Display for FlexStr<N, T>
where
    T: Deref<Target = str>,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <str as Display>::fmt(self, f)
    }
}

// *** Hash, PartialEq, Eq ***

impl<const N: usize, T> Hash for FlexStr<N, T>
where
    T: Deref<Target = str>,
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        str::hash(self, state)
    }
}

impl<const N: usize, T, T2> PartialEq<FlexStr<N, T2>> for FlexStr<N, T>
where
    T: Deref<Target = str>,
    T2: Deref<Target = str>,
{
    /// ```
    /// use flexstr::{AFlexStr, FlexStr, ToFlex};
    ///
    /// let lit = "inlined";
    /// let s: FlexStr = lit.into();
    /// let s2: AFlexStr = lit.into();
    /// assert_eq!(s, s2);
    /// ```
    #[inline]
    fn eq(&self, other: &FlexStr<N, T2>) -> bool {
        str::eq(self, &**other)
    }
}

impl<const N: usize, T, T2> PartialEq<FlexStr<N, T2>> for &FlexStr<N, T>
where
    T: Deref<Target = str>,
    T2: Deref<Target = str>,
{
    /// ```
    /// use flexstr::{AFlexStr, FlexStr, ToFlex};
    ///
    /// let lit = "inlined";
    /// let s: FlexStr = lit.into();
    /// let s2: AFlexStr = lit.into();
    /// assert_eq!(&s, s2);
    /// ```
    #[inline]
    fn eq(&self, other: &FlexStr<N, T2>) -> bool {
        str::eq(self, &**other)
    }
}

impl<const N: usize, T> PartialEq<&str> for FlexStr<N, T>
where
    T: Deref<Target = str>,
{
    /// ```
    /// use flexstr::{FlexStr, ToFlex};
    ///
    /// let lit = "inlined";
    /// let s: FlexStr = lit.to_flex();
    /// assert_eq!(s, lit);
    /// ```
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        str::eq(self, *other)
    }
}

impl<const N: usize, T> PartialEq<str> for FlexStr<N, T>
where
    T: Deref<Target = str>,
{
    /// ```
    /// use flexstr::{FlexStr, ToFlex};
    ///
    /// let lit = "inlined";
    /// let s: FlexStr = lit.to_flex();
    /// assert_eq!(s, lit);
    /// ```
    #[inline]
    fn eq(&self, other: &str) -> bool {
        str::eq(self, other)
    }
}

impl<const N: usize, T> PartialEq<String> for FlexStr<N, T>
where
    T: Deref<Target = str>,
{
    /// ```
    /// use flexstr::FlexStr;
    ///
    /// let lit = "inlined";
    /// let s: FlexStr = lit.into();
    /// assert_eq!(s, lit.to_string());
    /// ```
    #[inline]
    fn eq(&self, other: &String) -> bool {
        str::eq(self, other)
    }
}

impl<const N: usize, T> Eq for FlexStr<N, T> where T: Deref<Target = str> {}

// *** PartialOrd / Ord ***

impl<const N: usize, T> PartialOrd for FlexStr<N, T>
where
    T: Deref<Target = str>,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        str::partial_cmp(self, other)
    }
}

impl<const N: usize, T> PartialOrd<str> for FlexStr<N, T>
where
    T: Deref<Target = str>,
{
    #[inline]
    fn partial_cmp(&self, other: &str) -> Option<Ordering> {
        str::partial_cmp(self, other)
    }
}

impl<const N: usize, T> PartialOrd<String> for FlexStr<N, T>
where
    T: Deref<Target = str>,
{
    #[inline]
    fn partial_cmp(&self, other: &String) -> Option<Ordering> {
        str::partial_cmp(self, other)
    }
}

impl<const N: usize, T> Ord for FlexStr<N, T>
where
    T: Deref<Target = str>,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        str::cmp(self, other)
    }
}

// *** Index ***

macro_rules! impl_ranges {
    ($($type:ty),+) => {
        $(impl<const N: usize, T> Index<$type> for FlexStr<N, T>
        where
            T: Deref<Target = str>,
        {
            type Output = str;

            #[inline]
            fn index(&self, index: $type) -> &Self::Output {
                str::index(self, index)
            }
        })+
    }
}

impl_ranges!(
    Range<usize>,
    RangeTo<usize>,
    RangeFrom<usize>,
    RangeFull,
    RangeInclusive<usize>,
    RangeToInclusive<usize>
);

// *** Add ***

#[inline]
fn concat<const N: usize, T>(s1: &str, s2: &str) -> FlexStr<N, T>
where
    T: From<String> + for<'a> From<&'a str>,
{
    let mut buffer = buffer_new!(N);
    let mut builder = builder_new!(buffer, s1.len() + s2.len());
    builder.str_write(s1);
    builder.str_write(s2);
    builder_into!(builder, buffer)
}

impl<const N: usize, T> Add<&str> for FlexStr<N, T>
where
    T: From<String> + for<'a> From<&'a str> + Deref<Target = str>,
{
    type Output = FlexStr<N, T>;

    /// ```
    /// use flexstr::IntoFlexStr;
    ///
    /// let a = "in".into_flex_str() + "line";
    /// assert!(a.is_inlined());
    /// assert_eq!(a, "inline");
    ///
    /// let a = "in".to_string().into_flex_str() + "line";
    /// assert!(a.is_inlined());
    /// assert_eq!(a, "inline");
    /// ```
    #[inline]
    fn add(mut self, rhs: &str) -> Self::Output {
        match self.0 {
            FlexStrInner::Static(s) => concat(s, rhs),
            FlexStrInner::Inlined(ref mut s) => {
                if s.try_concat(rhs) {
                    self
                } else {
                    concat(s, rhs)
                }
            }
            FlexStrInner::Heap(s) => concat(&s, rhs),
        }
    }
}

// *** Misc. standard traits ***

impl<const N: usize, T> AsRef<str> for FlexStr<N, T>
where
    T: Deref<Target = str>,
{
    #[inline]
    fn as_ref(&self) -> &str {
        self
    }
}

impl<const N: usize, T> Default for FlexStr<N, T>
where
    T: Deref<Target = str>,
{
    #[inline]
    fn default() -> Self {
        Self::from_static("")
    }
}

impl<const N: usize, T> Borrow<str> for FlexStr<N, T>
where
    T: Deref<Target = str>,
{
    #[inline]
    fn borrow(&self) -> &str {
        str::borrow(self)
    }
}

impl<const N: usize, T> FromStr for FlexStr<N, T>
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

impl<const N: usize, T, T2> From<&FlexStr<N, T2>> for FlexStr<N, T>
where
    T2: Clone,
    FlexStr<N, T>: From<FlexStr<N, T2>>,
{
    #[inline]
    fn from(s: &FlexStr<N, T2>) -> Self {
        s.clone().into()
    }
}

impl<const N: usize, T> From<String> for FlexStr<N, T>
where
    T: From<String>,
{
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
    #[inline]
    fn from(s: String) -> Self {
        FlexStr(match s.try_into() {
            Ok(s) => FlexStrInner::Inlined(s),
            Err(s) => FlexStrInner::Heap(s.into()),
        })
    }
}

impl<const N: usize, T> From<&String> for FlexStr<N, T>
where
    T: for<'a> From<&'a str>,
{
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
    #[inline]
    fn from(s: &String) -> Self {
        s.to_flex()
    }
}

impl<const N: usize, T> From<&'static str> for FlexStr<N, T>
where
    T: Deref<Target = str>,
{
    /// ```
    /// use flexstr::FlexStr;
    ///
    /// let lit = "static";
    /// let s: FlexStr  = lit.into();
    /// assert!(s.is_static());
    /// assert_eq!(&s, lit);
    /// ```
    #[inline]
    fn from(s: &'static str) -> Self {
        Self::from_static(s)
    }
}

impl<const N: usize, T> From<char> for FlexStr<N, T>
where
    T: From<String> + for<'a> From<&'a str> + Deref<Target = str>,
{
    /// ```
    /// use flexstr::FlexStr;
    ///
    /// let s: FlexStr  = 't'.into();
    /// assert!(s.is_inlined());
    /// assert_eq!(&s, "t");
    /// ```
    #[inline]
    fn from(ch: char) -> Self {
        // SAFETY: Regardless of architecture, 4 bytes will always fit in an inline string
        unsafe { Self::try_inline(ch.encode_utf8(&mut [0; 4])).unwrap_unchecked() }
    }
}

// *** FromIterator ***

#[inline]
fn from_iter_str<const N: usize, I, T, U>(iter: I) -> FlexStr<N, T>
where
    I: IntoIterator<Item = U>,
    T: From<String> + for<'b> From<&'b str>,
    U: AsRef<str>,
{
    let iter = iter.into_iter();

    // Since `IntoIterator` consumes, we cannot loop over it twice to find lengths of strings
    // for a good capacity # without cloning it (which might be expensive)
    let mut buffer = buffer_new!(N);
    let mut builder = builder_new!(buffer);
    for s in iter {
        builder.str_write(s.as_ref());
    }
    builder_into!(builder, buffer)
}

#[inline]
fn from_iter_char<const N: usize, I, F, T, U>(iter: I, f: F) -> FlexStr<N, T>
where
    I: IntoIterator<Item = U>,
    F: Fn(U) -> char,
    T: From<String> + for<'b> From<&'b str>,
{
    let iter = iter.into_iter();
    let (lower, _) = iter.size_hint();

    let mut buffer = buffer_new!(N);
    let mut builder = builder_new!(buffer, lower);
    for ch in iter {
        builder.char_write(f(ch));
    }
    builder_into!(builder, buffer)
}

impl<const N: usize, T, T2> FromIterator<FlexStr<N, T2>> for FlexStr<N, T>
where
    T: From<String> + for<'b> From<&'b str>,
    T2: Deref<Target = str>,
{
    /// ```
    /// use flexstr::{FlexStr};
    ///
    /// let v: Vec<FlexStr> = vec!["best".into(), "test".into()];
    /// let s: FlexStr = v.into_iter().map(|s| if s == "best" { "test".into() } else { s }).collect();
    /// assert!(s.is_inlined());
    /// assert_eq!(s, "testtest");
    /// ```
    #[inline]
    fn from_iter<I: IntoIterator<Item = FlexStr<N, T2>>>(iter: I) -> Self {
        from_iter_str(iter)
    }
}

impl<'a, const N: usize, T, T2> FromIterator<&'a FlexStr<N, T2>> for FlexStr<N, T>
where
    T: From<String> + for<'b> From<&'b str>,
    T2: Deref<Target = str> + 'a,
{
    /// ```
    /// use flexstr::{FlexStr};
    ///
    /// let v: Vec<FlexStr> = vec!["best".into(), "test".into()];
    /// let s: FlexStr = v.iter().filter(|s| *s == "best").collect();
    /// assert!(s.is_inlined());
    /// assert_eq!(s, "best");
    /// ```
    #[inline]
    fn from_iter<I: IntoIterator<Item = &'a FlexStr<N, T2>>>(iter: I) -> Self {
        from_iter_str(iter)
    }
}

impl<const N: usize, T> FromIterator<String> for FlexStr<N, T>
where
    T: From<String> + for<'b> From<&'b str>,
{
    /// ```
    /// use flexstr::{FlexStr};
    ///
    /// let v = vec!["best".to_string(), "test".to_string()];
    /// let s: FlexStr = v.into_iter().map(|s| if s == "best" { "test".into() } else { s }).collect();
    /// assert!(s.is_inlined());
    /// assert_eq!(s, "testtest");
    /// ```
    #[inline]
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        from_iter_str(iter)
    }
}

impl<'a, const N: usize, T> FromIterator<&'a str> for FlexStr<N, T>
where
    T: From<String> + for<'b> From<&'b str>,
{
    /// ```
    /// use flexstr::{FlexStr};
    ///
    /// let v = vec!["best", "test"];
    /// let s: FlexStr = v.into_iter().map(|s| if s == "best" { "test" } else { s }).collect();
    /// assert!(s.is_inlined());
    /// assert_eq!(s, "testtest");
    /// ```
    #[inline]
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        from_iter_str(iter)
    }
}

impl<const N: usize, T> FromIterator<char> for FlexStr<N, T>
where
    T: From<String> + for<'b> From<&'b str>,
{
    /// ```
    /// use flexstr::{FlexStr};
    ///
    /// let v = "besttest";
    /// let s: FlexStr = v.chars().map(|c| if c == 'b' { 't' } else { c }).collect();
    /// assert!(s.is_inlined());
    /// assert_eq!(s, "testtest");
    /// ```
    #[inline]
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
        from_iter_char(iter, |ch| ch)
    }
}

impl<'a, const N: usize, T> FromIterator<&'a char> for FlexStr<N, T>
where
    T: From<String> + for<'b> From<&'b str>,
{
    /// ```
    /// use flexstr::{FlexStr};
    ///
    /// let v = vec!['b', 'e', 's', 't', 't', 'e', 's', 't'];
    /// let s: FlexStr = v.iter().filter(|&ch| *ch != 'b').collect();
    /// assert!(s.is_inlined());
    /// assert_eq!(s, "esttest");
    /// ```
    #[inline]
    fn from_iter<I: IntoIterator<Item = &'a char>>(iter: I) -> Self {
        from_iter_char(iter, |ch| *ch)
    }
}

// *** Optional serialization support ***

#[cfg(feature = "serde")]
impl<const N: usize, T> Serialize for FlexStr<N, T>
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
struct FlexStrVisitor<const N: usize, T>(PhantomData<*const T>);

#[cfg(feature = "serde")]
impl<'de, const N: usize, T> Visitor<'de> for FlexStrVisitor<N, T>
where
    T: From<String> + for<'a> From<&'a str>,
{
    type Value = FlexStr<N, T>;

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
impl<'de, const N: usize, T> Deserialize<'de> for FlexStr<N, T>
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
pub fn flex_fmt<const N: usize, T>(args: Arguments<'_>) -> FlexStr<N, T>
where
    T: From<String> + for<'a> From<&'a str>,
{
    // NOTE: We have a disadvantage to `String` because we cannot call `estimated_capacity()` on args
    // As such, we cannot assume a given needed capacity - we start with a stack allocated buffer
    // and only promote to a heap buffer if a write won't fit
    let mut buffer = buffer_new!(N);
    let mut builder = builder_new!(buffer);
    builder
        .write_fmt(args)
        .expect("a formatting trait implementation returned an error");
    builder_into!(builder, buffer)
}

/// `FlexStr` equivalent to `format!` macro from stdlib. Efficiently creates a native `FlexStr`
/// ```
/// use flexstr::flex_fmt;
///
/// let a = flex_fmt!("Is {}", "inlined");
/// assert!(a.is_inlined());
/// assert_eq!(a, "Is inlined")
/// ```
#[macro_export]
macro_rules! flex_fmt {
    ($($arg:tt)*) => {{
        let s: flexstr::FlexStr = flexstr::flex_fmt(format_args!($($arg)*));
        s
    }}
}

/// `AFlexStr` equivalent to `format!` macro from stdlib. Efficiently creates a native `AFlexStr`
/// ```
/// use flexstr::a_flex_fmt;
///
/// let a = a_flex_fmt!("Is {}", "inlined");
/// assert!(a.is_inlined());
/// assert_eq!(a, "Is inlined")
/// ```

#[macro_export]
macro_rules! a_flex_fmt {
    ($($arg:tt)*) => {{
        let s: flexstr::AFlexStr = flexstr::flex_fmt(format_args!($($arg)*));
        s
    }}
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "serde")]
    #[test]
    fn serialization() {
        use crate::{AFlexStr, FlexStr};
        use alloc::string::ToString;
        use serde_json::json;

        #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq)]
        struct Test {
            a: FlexStr,
            b: AFlexStr,
            c: FlexStr,
        }

        let a = "test";
        let b = "testing";
        let c = "testing testing testing testing testing testing testing testing testing";

        // Create our struct and values and verify storage
        let test = Test {
            a: a.into(),
            b: b.to_string().into(),
            c: c.to_string().into(),
        };
        assert!(test.a.is_static());
        assert!(test.b.is_inlined());
        assert!(test.c.is_heap());

        // Serialize and ensure our JSON value actually matches
        let val = serde_json::to_value(test.clone()).unwrap();
        assert_eq!(json!({"a": a, "b": b, "c": c}), val);

        // Deserialize and validate storage and contents
        let test2: Test = serde_json::from_value(val).unwrap();
        assert!(test2.a.is_inlined());
        assert!(test2.b.is_inlined());
        assert!(test2.c.is_heap());

        assert_eq!(&test, &test2);
    }
}
