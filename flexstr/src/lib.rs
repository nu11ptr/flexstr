#![no_std]
#![warn(missing_docs)]

//! A flexible, simple to use, immutable, clone-efficient `String` replacement for Rust
//!
//! ```
//! use flexstr::{flex_fmt, flex_str, FlexStr, IntoFlexStr, ToCase, ToFlexStr};
//!
//! // Use `flex_str` macro to wrap literals as compile-time constants
//! const STATIC_STR: FlexStr = flex_str!("This will not allocate or copy");
//! assert!(STATIC_STR.is_static());
//!
//! // Strings up to 22 bytes (on 64-bit) will be inlined automatically
//! // (demo only, use macro or `from_static` for literals as above)
//! let inline_str = "inlined".to_flex_str();
//! assert!(inline_str.is_inlined());
//!
//! // When a string is too long to be wrapped/inlined, it will heap allocate
//! // (demo only, use macro or `from_static` for literals as above)
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
//! // Clone is cheap, and never allocates
//! // (at most it is a ref count increment for heap allocated strings)
//! let rc_str2 = rc_str.clone();
//! assert!(rc_str2.is_heap());
//!
//! // Regardless of storage type, these all operate seamlessly together
//! // and choose storage as required
//! let heap_str2 = STATIC_STR + &inline_str;
//! assert!(heap_str2.is_heap());
//! assert_eq!(heap_str2, "This will not allocate or copyinlined");  
//! ```

extern crate alloc;

#[doc(hidden)]
#[macro_use]
pub mod builder;
#[doc(hidden)]
pub mod inline;
#[doc(hidden)]
pub mod traits;

pub use inline::STRING_SIZED_INLINE;
#[doc(inline)]
pub use traits::*;

use alloc::rc::Rc;
use alloc::string::String;
use alloc::sync::Arc;
use core::cmp::Ordering;
use core::convert::Infallible;
use core::fmt::{Arguments, Debug, Display, Formatter, Write};
use core::hash::{Hash, Hasher};
#[cfg(feature = "serde")]
use core::marker::PhantomData;
use core::mem::ManuallyDrop;
use core::ops::{
    Add, Deref, Index, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};
use core::str::FromStr;
use core::{fmt, mem};

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

    external_doc_test!(include_str!("../../README.md"));
}

const PTR_SIZED_PAD: usize = mem::size_of::<*const ()>() - 1;

/// Error type returned from `try_to_static_str` when  this `FlexStr` does not contain a `'static str`
#[derive(Copy, Clone, Debug)]
pub struct NotStatic;

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
enum FlexMarker {
    Static,
    Inline,
    Heap,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct StaticStr<const N: usize> {
    literal: &'static str,
    pad: [mem::MaybeUninit<u8>; N],
    marker: FlexMarker,
}

impl<const N: usize> StaticStr<N> {
    const EMPTY: Self = Self::from_static("");

    #[inline]
    const fn from_static(s: &'static str) -> Self {
        Self {
            literal: s,
            // SAFETY: Padding, never actually used
            pad: unsafe { mem::MaybeUninit::uninit().assume_init() },
            marker: FlexMarker::Static,
        }
    }
}

#[cfg_attr(target_pointer_width = "64", repr(align(8)))]
#[cfg_attr(target_pointer_width = "32", repr(align(4)))]
#[repr(C)]
#[derive(Clone)]
struct HeapStr<const N: usize, T> {
    heap: T,
    pad: [mem::MaybeUninit<u8>; N],
    marker: FlexMarker,
}

impl<const N: usize, T> HeapStr<N, T> {
    #[inline]
    fn new(t: T) -> Self {
        Self {
            heap: t,
            // SAFETY: Padding, never actually used
            pad: unsafe { mem::MaybeUninit::uninit().assume_init() },
            marker: FlexMarker::Heap,
        }
    }
}

impl<const N: usize, T> HeapStr<N, T>
where
    T: for<'a> From<&'a str>,
{
    #[inline]
    fn from_ref(s: impl AsRef<str>) -> Self {
        Self::new(s.as_ref().into())
    }
}

/// A flexible string type that transparently wraps a string literal, inline string, or a heap allocated type
pub union Flex_<const N: usize, const N2: usize, const N3: usize, T> {
    literal: StaticStr<N2>,
    inline: inline::InlineFlexStr<N>,
    heap: mem::ManuallyDrop<HeapStr<N3, T>>,
}

/// A flexible string type that transparently wraps a string literal, inline string, or an `Rc<str>`
pub type FlexStr_ = Flex_<STRING_SIZED_INLINE, PTR_SIZED_PAD, PTR_SIZED_PAD, Rc<str>>;

/// A flexible string type that transparently wraps a string literal, inline string, or an `Arc<str>`
pub type AFlexStr_ = Flex_<STRING_SIZED_INLINE, PTR_SIZED_PAD, PTR_SIZED_PAD, Arc<str>>;

impl<const N: usize, const N2: usize, const N3: usize, T> Clone for Flex_<N, N2, N3, T>
where
    T: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        // SAFETY: Marker check is aligned to correct accessed field
        unsafe {
            match self.literal.marker {
                FlexMarker::Static => Flex_ {
                    literal: self.literal,
                },
                FlexMarker::Inline => Flex_ {
                    inline: self.inline,
                },
                FlexMarker::Heap => Flex_ {
                    // Recreating vs. calling clone at the top is 30% faster in benchmarks
                    heap: ManuallyDrop::new(HeapStr::new(self.heap.heap.clone())),
                },
            }
        }
    }
}

impl<const N: usize, const N2: usize, const N3: usize, T> Drop for Flex_<N, N2, N3, T> {
    #[inline]
    fn drop(&mut self) {
        // SAFETY: Marker check is aligned to correct accessed field
        unsafe {
            if let FlexMarker::Heap = self.heap.marker {
                ManuallyDrop::drop(&mut self.heap);
            }
        }
    }
}

impl<const N: usize, const N2: usize, const N3: usize, T> From<&str> for Flex_<N, N2, N3, T>
where
    T: for<'a> From<&'a str>,
{
    #[inline]
    fn from(s: &str) -> Self {
        if s.is_empty() {
            Flex_ {
                literal: StaticStr::EMPTY,
            }
        } else {
            match inline::InlineFlexStr::try_new(s) {
                Ok(s) => Flex_ { inline: s },
                Err(_) => Flex_ {
                    heap: ManuallyDrop::new(HeapStr::from_ref(s)),
                },
            }
        }
    }
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub enum FlexInner<const N: usize, T> {
    // A wrapped string literal
    Static(&'static str),
    // An inlined string
    Inlined(inline::InlineFlexStr<N>),
    // A reference count wrapped `str`
    Heap(T),
}

/// A flexible string type that transparently wraps a string literal, inline string, or a heap allocated type
#[derive(Clone, Debug)]
pub struct Flex<const N: usize, T>(#[doc(hidden)] pub FlexInner<N, T>);

/// A flexible string type that transparently wraps a string literal, inline string, or an `Rc<str>`
pub type FlexStr = Flex<STRING_SIZED_INLINE, Rc<str>>;

/// A flexible string type that transparently wraps a string literal, inline string, or an `Arc<str>`
pub type AFlexStr = Flex<STRING_SIZED_INLINE, Arc<str>>;

impl<const N: usize, T> Flex<N, T> {
    /// Creates a wrapped static string literal. This function is equivalent to calling the `into`
    /// functions on a static string literal, but is `const fn` so can be used to init a constant.
    /// ```
    /// use flexstr::FlexStr;
    ///
    /// const S: FlexStr = FlexStr::from_static("test");
    /// assert!(S.is_static());
    /// ```
    #[inline]
    pub const fn from_static(s: &'static str) -> Flex<N, T> {
        Flex(FlexInner::Static(s))
    }
}

impl<const N: usize, T> Flex<N, T>
where
    T: Deref<Target = str>,
{
    /// Attempts to create an inlined string. Returns new inline string on success or original source
    /// string as `Err` if it will not fit. Since the to/into functions will automatically inline when
    /// possible, this function is really only for special use cases.
    /// ```
    /// use flexstr::FlexStr;
    ///
    /// let s = FlexStr::try_inline("test").unwrap();
    /// assert!(s.is_inlined());
    /// ```
    #[inline]
    pub fn try_inline<S: AsRef<str>>(s: S) -> Result<Flex<N, T>, S> {
        match inline::InlineFlexStr::try_new(s) {
            Ok(s) => Ok(Flex(FlexInner::Inlined(s))),
            Err(s) => Err(s),
        }
    }

    /// Force the creation of a heap allocated string. Unlike to/into functions, this will not attempt
    /// to inline first even if the string is a candidate for inlining. Using this is generally not
    /// recommended, and the to/into conversion functions should be preferred.
    /// ```
    /// use flexstr::FlexStr;
    ///
    /// let s = FlexStr::heap("test");
    /// assert!(s.is_heap());
    /// ```
    #[inline]
    pub fn heap(s: impl AsRef<str>) -> Flex<N, T>
    where
        T: for<'a> From<&'a str>,
    {
        Flex(FlexInner::Heap(s.as_ref().into()))
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
            FlexInner::Static(str) => str.is_empty(),
            FlexInner::Inlined(s) => s.is_empty(),
            FlexInner::Heap(rc) => rc.is_empty(),
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
            FlexInner::Static(str) => str.len(),
            FlexInner::Inlined(s) => s.len(),
            FlexInner::Heap(rc) => rc.len(),
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
    /// use flexstr::flex_str;
    ///
    /// let s = flex_str!("abc").to_std_string();
    /// assert_eq!(s, "abc");
    /// ```
    #[inline]
    pub fn to_std_string(&self) -> String {
        String::from(&**self)
    }

    /// Attempts to extract a static inline string literal if one is stored inside this `FlexStr`.
    /// Returns `NotStatic` as an `Err` if this is not a static string literal.
    /// ```
    /// use flexstr::flex_str;
    ///
    /// let s = "abc";
    /// let s2 = flex_str!(s);
    /// assert_eq!(s2.try_to_static_str().unwrap(), s);
    /// ```
    #[inline]
    pub fn try_to_static_str(&self) -> Result<&'static str, NotStatic> {
        match self.0 {
            FlexInner::Static(s) => Ok(s),
            _ => Err(NotStatic),
        }
    }

    /// Returns true if this is a wrapped string literal (`&'static str`)
    /// ```
    /// use flexstr::FlexStr;
    ///
    /// let s = FlexStr::from_static("test");
    /// assert!(s.is_static());
    /// ```
    #[inline]
    pub fn is_static(&self) -> bool {
        matches!(self.0, FlexInner::Static(_))
    }

    /// Returns true if this is an inlined string
    /// ```
    /// use flexstr::FlexStr;
    ///
    /// let s = FlexStr::try_inline("test").unwrap();
    /// assert!(s.is_inlined());
    /// ```
    #[inline]
    pub fn is_inlined(&self) -> bool {
        matches!(self.0, FlexInner::Inlined(_))
    }

    /// Returns true if this is a wrapped string using heap storage
    /// ```
    /// use flexstr::FlexStr;
    ///
    /// let s = FlexStr::heap("test");
    /// assert!(s.is_heap());
    /// ```
    #[inline]
    pub fn is_heap(&self) -> bool {
        matches!(self.0, FlexInner::Heap(_))
    }
}

// *** Deref / Debug / Display ***

impl<const N: usize, T> Deref for Flex<N, T>
where
    T: Deref<Target = str>,
{
    type Target = str;

    /// ```
    /// use flexstr::flex_str;
    ///
    /// let a = "test";
    /// let b = flex_str!(a);
    /// assert_eq!(&*b, a);
    /// ```
    #[inline]
    fn deref(&self) -> &Self::Target {
        match &self.0 {
            FlexInner::Static(str) => str,
            FlexInner::Inlined(ss) => ss,
            FlexInner::Heap(rc) => rc,
        }
    }
}

impl<const N: usize, T> Display for Flex<N, T>
where
    T: Deref<Target = str>,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <str as Display>::fmt(self, f)
    }
}

#[cfg(feature = "fast_format")]
impl<const N: usize, T> ufmt::uDisplay for Flex<N, T>
where
    T: Deref<Target = str>,
{
    #[inline]
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt_write::uWrite + ?Sized,
    {
        <str as ufmt::uDisplay>::fmt(self, f)
    }
}

#[cfg(feature = "fast_format")]
impl<const N: usize, T> ufmt::uDebug for Flex<N, T>
where
    T: Deref<Target = str>,
{
    #[inline]
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt_write::uWrite + ?Sized,
    {
        // uDebug is not implemented for str it seems which means we can't derive
        <str as ufmt::uDisplay>::fmt(self, f)
    }
}

// *** Hash, PartialEq, Eq ***

impl<const N: usize, T> Hash for Flex<N, T>
where
    T: Deref<Target = str>,
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        str::hash(self, state)
    }
}

impl<const N: usize, T, T2> PartialEq<Flex<N, T2>> for Flex<N, T>
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
    fn eq(&self, other: &Flex<N, T2>) -> bool {
        str::eq(self, &**other)
    }
}

impl<const N: usize, T, T2> PartialEq<Flex<N, T2>> for &Flex<N, T>
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
    fn eq(&self, other: &Flex<N, T2>) -> bool {
        str::eq(self, &**other)
    }
}

impl<const N: usize, T> PartialEq<&str> for Flex<N, T>
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

impl<const N: usize, T> PartialEq<str> for Flex<N, T>
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

impl<const N: usize, T> PartialEq<String> for Flex<N, T>
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

impl<const N: usize, T> Eq for Flex<N, T> where T: Deref<Target = str> {}

// *** PartialOrd / Ord ***

impl<const N: usize, T> PartialOrd for Flex<N, T>
where
    T: Deref<Target = str>,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        str::partial_cmp(self, other)
    }
}

impl<const N: usize, T> PartialOrd<str> for Flex<N, T>
where
    T: Deref<Target = str>,
{
    #[inline]
    fn partial_cmp(&self, other: &str) -> Option<Ordering> {
        str::partial_cmp(self, other)
    }
}

impl<const N: usize, T> PartialOrd<String> for Flex<N, T>
where
    T: Deref<Target = str>,
{
    #[inline]
    fn partial_cmp(&self, other: &String) -> Option<Ordering> {
        str::partial_cmp(self, other)
    }
}

impl<const N: usize, T> Ord for Flex<N, T>
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
        $(impl<const N: usize, T> Index<$type> for Flex<N, T>
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
fn concat<const N: usize, T>(s1: &str, s2: &str) -> Flex<N, T>
where
    T: for<'a> From<&'a str>,
{
    let mut buffer = buffer_new!(N);
    let mut builder = builder_new!(buffer, s1.len() + s2.len());
    builder.str_write(s1);
    builder.str_write(s2);
    builder_into!(builder, buffer)
}

impl<const N: usize, T> Add<&str> for Flex<N, T>
where
    T: for<'a> From<&'a str> + Deref<Target = str>,
{
    type Output = Flex<N, T>;

    /// ```
    /// use flexstr::{flex_str, IntoFlexStr};
    ///
    /// let a = flex_str!("in") + "line";
    /// assert!(a.is_inlined());
    /// assert_eq!(a, "inline");
    ///
    /// let a = "in".to_string().into_flex_str() + "line";
    /// assert!(a.is_inlined());
    /// assert_eq!(a, "inline");
    /// ```
    #[inline]
    fn add(mut self, rhs: &str) -> Self::Output {
        if rhs.is_empty() {
            self
        } else if self.is_empty() {
            rhs.into()
        } else {
            match self.0 {
                FlexInner::Static(s) => concat(s, rhs),
                FlexInner::Inlined(ref mut s) => {
                    if s.try_concat(rhs) {
                        self
                    } else {
                        concat(s, rhs)
                    }
                }
                FlexInner::Heap(s) => concat(&s, rhs),
            }
        }
    }
}

// *** Misc. standard traits ***

impl<const N: usize, T> AsRef<str> for Flex<N, T>
where
    T: Deref<Target = str>,
{
    #[inline]
    fn as_ref(&self) -> &str {
        self
    }
}

impl<const N: usize, T> Default for Flex<N, T> {
    #[inline]
    fn default() -> Self {
        Self::from_static("")
    }
}

impl<const N: usize, T> FromStr for Flex<N, T>
where
    T: for<'a> From<&'a str>,
{
    type Err = Infallible;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

// *** From ***

impl<const N: usize, T, T2> From<&Flex<N, T2>> for Flex<N, T>
where
    T: for<'a> From<&'a str>,
    T2: Clone + Deref<Target = str>,
{
    #[inline]
    fn from(s: &Flex<N, T2>) -> Self {
        s.clone().into_flex()
    }
}

impl<const N: usize, T> From<String> for Flex<N, T>
where
    T: for<'a> From<&'a str>,
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
        <Self as From<&str>>::from(&s)
    }
}

impl<const N: usize, T> From<&String> for Flex<N, T>
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
        <Self as From<&str>>::from(s)
    }
}

impl<const N: usize, T> From<&str> for Flex<N, T>
where
    T: for<'a> From<&'a str>,
{
    /// ```
    /// use flexstr::FlexStr;
    ///
    /// let lit = "inline";
    /// let s: FlexStr  = lit.into();
    /// assert!(s.is_inlined());
    /// assert_eq!(&s, lit);
    /// ```
    #[inline]
    fn from(s: &str) -> Self {
        if s.is_empty() {
            Self::default()
        } else {
            Flex(match s.try_into() {
                Ok(s) => FlexInner::Inlined(s),
                Err(_) => FlexInner::Heap(s.into()),
            })
        }
    }
}

impl<const N: usize, T> From<char> for Flex<N, T>
where
    T: Deref<Target = str>,
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
fn from_iter_str<const N: usize, I, T, U>(iter: I) -> Flex<N, T>
where
    I: IntoIterator<Item = U>,
    T: for<'b> From<&'b str>,
    U: AsRef<str>,
{
    let iter = iter.into_iter();

    // Since `IntoIterator` consumes, we cannot loop over it twice to find lengths of strings
    // for a good capacity # without cloning it (which might be expensive)
    let mut buffer = buffer_new!(N);
    let mut builder = builder_new!(buffer);
    for s in iter {
        builder.str_write(s);
    }
    builder_into!(builder, buffer)
}

#[inline]
fn from_iter_char<const N: usize, I, F, T, U>(iter: I, f: F) -> Flex<N, T>
where
    I: IntoIterator<Item = U>,
    F: Fn(U) -> char,
    T: for<'b> From<&'b str>,
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

impl<const N: usize, T, T2> FromIterator<Flex<N, T2>> for Flex<N, T>
where
    T: for<'b> From<&'b str>,
    T2: Deref<Target = str>,
{
    /// ```
    /// use flexstr::FlexStr;
    ///
    /// let v: Vec<FlexStr> = vec!["best".into(), "test".into()];
    /// let s: FlexStr = v.into_iter().map(|s| if s == "best" { "test".into() } else { s }).collect();
    /// assert!(s.is_inlined());
    /// assert_eq!(s, "testtest");
    /// ```
    #[inline]
    fn from_iter<I: IntoIterator<Item = Flex<N, T2>>>(iter: I) -> Self {
        from_iter_str(iter)
    }
}

impl<'a, const N: usize, T, T2> FromIterator<&'a Flex<N, T2>> for Flex<N, T>
where
    T: for<'b> From<&'b str>,
    T2: Deref<Target = str> + 'a,
{
    /// ```
    /// use flexstr::FlexStr;
    ///
    /// let v: Vec<FlexStr> = vec!["best".into(), "test".into()];
    /// let s: FlexStr = v.iter().filter(|s| *s == "best").collect();
    /// assert!(s.is_inlined());
    /// assert_eq!(s, "best");
    /// ```
    #[inline]
    fn from_iter<I: IntoIterator<Item = &'a Flex<N, T2>>>(iter: I) -> Self {
        from_iter_str(iter)
    }
}

impl<const N: usize, T> FromIterator<String> for Flex<N, T>
where
    T: for<'b> From<&'b str>,
{
    /// ```
    /// use flexstr::FlexStr;
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

impl<'a, const N: usize, T> FromIterator<&'a str> for Flex<N, T>
where
    T: for<'b> From<&'b str>,
{
    /// ```
    /// use flexstr::FlexStr;
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

impl<const N: usize, T> FromIterator<char> for Flex<N, T>
where
    T: for<'b> From<&'b str>,
{
    /// ```
    /// use flexstr::FlexStr;
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

impl<'a, const N: usize, T> FromIterator<&'a char> for Flex<N, T>
where
    T: for<'b> From<&'b str>,
{
    /// ```
    /// use flexstr::FlexStr;
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
impl<const N: usize, T> Serialize for Flex<N, T>
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
    T: for<'a> From<&'a str>,
{
    type Value = Flex<N, T>;

    #[inline]
    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }

    #[inline]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v.into())
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
impl<'de, const N: usize, T> Deserialize<'de> for Flex<N, T>
where
    T: for<'a> From<&'a str>,
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(FlexStrVisitor(PhantomData))
    }
}

/// Create compile time constant `FlexStr` (equivalent, but less typing than:
/// `<FlexStr>::from_static("my_literal")`
/// ```
/// use flexstr::{flex_str, FlexStr};
///
/// const STR: FlexStr = flex_str!("This is a constant!");
/// assert!(STR.is_static())
/// ```
#[macro_export]
macro_rules! flex_str {
    ($str:expr) => {
        <$crate::FlexStr>::from_static($str)
    };
}

/// Create compile time constant `AFlexStr` (equivalent, but less typing than:
/// `<AFlexStr>::from_static("my_literal")`
/// ```
/// use flexstr::{a_flex_str, AFlexStr};
///
/// const STR: AFlexStr = a_flex_str!("This is a constant!");
/// assert!(STR.is_static())
/// ```
#[macro_export]
macro_rules! a_flex_str {
    ($str:expr) => {
        <$crate::AFlexStr>::from_static($str)
    };
}

/// `FlexStr` equivalent to `format` function from stdlib. Efficiently creates a native `FlexStr`
pub fn flex_fmt<const N: usize, T>(args: Arguments<'_>) -> Flex<N, T>
where
    T: for<'a> From<&'a str>,
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

/// Equivalent to `flex_fmt` except that it uses `ufmt` which is much faster, but has limitations.
/// See [ufmt docs](https://docs.rs/ufmt/latest/ufmt/) for more details
/// ```
/// use flexstr::{flex_str, flex_ufmt};
///
/// let a = flex_ufmt!("Is {}{}", flex_str!("inlined"), "!");
/// assert!(a.is_inlined());
/// assert_eq!(a, "Is inlined!");
/// ```
#[cfg(feature = "fast_format")]
#[macro_export(local_inner_macros)]
macro_rules! flex_ufmt {
    ($($arg:tt)*) => {{
        let mut buffer = buffer_new!({ $crate::STRING_SIZED_INLINE });
        let mut builder = builder_new!(buffer);

        ufmt::uwrite!(&mut builder, $($arg)*).expect("a formatting trait implementation returned an error");
        let s: $crate::FlexStr = builder_into!(builder, buffer);
        s
    }}
}

/// Equivalent to `a_flex_fmt` except that it uses `ufmt` which is much faster, but has limitations.
/// See [ufmt docs](https://docs.rs/ufmt/latest/ufmt/) for more details
/// ```
/// use flexstr::{a_flex_str, a_flex_ufmt};
///
/// let a = a_flex_ufmt!("Is {}{}", a_flex_str!("inlined"), "!");
/// assert!(a.is_inlined());
/// assert_eq!(a, "Is inlined!");
/// ```
#[cfg(feature = "fast_format")]
#[macro_export(local_inner_macros)]
macro_rules! a_flex_ufmt {
    ($($arg:tt)*) => {{
        let mut buffer = buffer_new!({ $crate::STRING_SIZED_INLINE });
        let mut builder = builder_new!(buffer);

        ufmt::uwrite!(&mut builder, $($arg)*).expect("a formatting trait implementation returned an error");
        let s: $crate::AFlexStr = builder_into!(builder, buffer);
        s
    }}
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
            a: flex_str!(a),
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
