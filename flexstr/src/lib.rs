#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

//! A flexible, simple to use, immutable, clone-efficient `String` replacement for Rust
//!
//! ```
//! use flexstr::{local_fmt, local_str, LocalStr, IntoLocalStr, ToCase, ToLocalStr};
//!
//! // Use `local_str` macro to wrap literals as compile-time constants
//! const STATIC_STR: LocalStr = local_str!("This will not allocate or copy");
//! assert!(STATIC_STR.is_static());
//!
//! // Strings up to 22 bytes (on 64-bit) will be inlined automatically
//! // (demo only, use macro or `from_static` for literals as above)
//! let inline_str = "inlined".to_local_str();
//! assert!(inline_str.is_inline());
//!
//! // When a string is too long to be wrapped/inlined, it will heap allocate
//! // (demo only, use macro or `from_static` for literals as above)
//! let rc_str = "This is too long to be inlined".to_local_str();
//! assert!(rc_str.is_heap());
//!
//! // You can efficiently create a new `LocalStr` (without creating a `String`)
//! // This is equivalent to the stdlib `format!` macro
//! let inline_str2 = local_fmt!("in{}", "lined");
//! assert!(inline_str2.is_inline());
//! assert_eq!(inline_str, inline_str2);
//!
//! // We can upper/lowercase strings without converting to a `String` first
//! // This doesn't heap allocate since inlined
//! let inline_str3: LocalStr = "INLINED".to_ascii_lower();
//! assert!(inline_str3.is_inline());
//! assert_eq!(inline_str, inline_str3);
//!
//! // Concatenation doesn't even copy if we can fit it in the inline string
//! let inline_str4 = inline_str3 + "!!!";
//! assert!(inline_str4.is_inline());
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

/// Padding the size of a pointer for this platform minus one
pub const PTR_SIZED_PAD: usize = mem::size_of::<*const ()>() - 1;

/// Error type returned from [try_as_static_str] or [try_into_heap] when  this [FlexStr] does not contain a `'static str`
#[derive(Copy, Clone, Debug)]
pub struct WrongStorageType {
    /// The expected storage type of the string
    pub expected: StorageType,
    /// The actual storage type of the string
    pub actual: StorageType,
}

impl Display for WrongStorageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("The FlexStr did no use the storage type expected (expected: ")?;
        self.expected.fmt(f)?;
        f.write_str(", actual: ")?;
        self.actual.fmt(f)?;
        f.write_str(")")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for WrongStorageType {}

/// Represents the storage type used by a particular [FlexStr]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum StorageType {
    /// Denotes that this [FlexStr] is a wrapper string literal
    Static,
    /// Denotes that this [FlexStr] is inlined
    Inline,
    /// Denotes that this [FlexStr] uses heap-based storage
    Heap,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct StaticStr<const PAD: usize> {
    literal: &'static str,
    pad: [mem::MaybeUninit<u8>; PAD],
    marker: StorageType,
}

impl<const PAD: usize> StaticStr<PAD> {
    const EMPTY: Self = Self::from_static("");

    #[inline]
    const fn from_static(s: &'static str) -> Self {
        Self {
            literal: s,
            // SAFETY: Padding, never actually used
            pad: unsafe { mem::MaybeUninit::uninit().assume_init() },
            marker: StorageType::Static,
        }
    }
}

impl<const PAD: usize> Debug for StaticStr<PAD> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <str as Debug>::fmt(self.literal, f)
    }
}

// T will likely align this just fine, but since we don't know the size, this is safest
#[cfg_attr(target_pointer_width = "64", repr(align(8)))]
#[cfg_attr(target_pointer_width = "32", repr(align(4)))]
#[repr(C)]
#[derive(Clone)]
struct HeapStr<const PAD: usize, HEAP> {
    heap: HEAP,
    pad: [mem::MaybeUninit<u8>; PAD],
    marker: StorageType,
}

impl<const PAD: usize, HEAP> HeapStr<PAD, HEAP> {
    #[inline]
    fn from_heap(t: HEAP) -> Self {
        Self {
            heap: t,
            // SAFETY: Padding, never actually used
            pad: unsafe { mem::MaybeUninit::uninit().assume_init() },
            marker: StorageType::Heap,
        }
    }

    #[inline]
    fn from_ref(s: impl AsRef<str>) -> Self
    where
        HEAP: for<'a> From<&'a str>,
    {
        Self::from_heap(s.as_ref().into())
    }
}

impl<const PAD: usize, HEAP> Debug for HeapStr<PAD, HEAP>
where
    HEAP: Deref<Target = str>,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <str as Debug>::fmt(&self.heap, f)
    }
}

/// A flexible string type that transparently wraps a string literal, inline string, or a heap allocated type
pub union FlexStrWrapper<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> {
    static_str: StaticStr<PAD1>,
    #[doc(hidden)]
    pub inline_str: inline::InlineFlexStr<SIZE>,
    heap_str: mem::ManuallyDrop<HeapStr<PAD2, HEAP>>,
}

/// A flexible string type that transparently wraps a string literal, inline string, or a HEAP type
pub type FlexStr<HEAP> = FlexStrWrapper<STRING_SIZED_INLINE, PTR_SIZED_PAD, PTR_SIZED_PAD, HEAP>;

/// A flexible string type that transparently wraps a string literal, inline string, or an `Rc<str>`
pub type LocalStr = FlexStr<Rc<str>>;

/// A flexible string type that transparently wraps a string literal, inline string, or an `Arc<str>`
pub type SharedStr = FlexStr<Arc<str>>;

// *** Clone ***

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> Clone
    for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        // SAFETY: Marker check is aligned to correct accessed field
        unsafe {
            match self.static_str.marker {
                StorageType::Static => FlexStrWrapper {
                    static_str: self.static_str,
                },
                StorageType::Inline => FlexStrWrapper {
                    inline_str: self.inline_str,
                },
                StorageType::Heap => FlexStrWrapper {
                    // Recreating vs. calling clone at the top is 30% faster in benchmarks
                    heap_str: ManuallyDrop::new(HeapStr::from_heap(self.heap_str.heap.clone())),
                },
            }
        }
    }
}

// *** Drop ***

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> Drop
    for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
{
    #[inline]
    fn drop(&mut self) {
        // SAFETY: Marker check is aligned to correct accessed field
        unsafe {
            if let StorageType::Heap = self.heap_str.marker {
                ManuallyDrop::drop(&mut self.heap_str);
            }
        }
    }
}

// *** Deref ***

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> Deref
    for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: Deref<Target = str>,
{
    type Target = str;

    /// ```
    /// use flexstr::local_str;
    ///
    /// let a = "test";
    /// let b = local_str!(a);
    /// assert_eq!(&*b, a);
    /// ```
    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: Marker check is aligned to correct accessed field
        unsafe {
            match self.static_str.marker {
                StorageType::Static => self.static_str.literal,
                StorageType::Inline => &self.inline_str,
                StorageType::Heap => &self.heap_str.heap,
            }
        }
    }
}

// *** Non-trait functions ***

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP>
    FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
{
    /// An empty ("") static constant string
    pub const EMPTY: Self = FlexStrWrapper {
        static_str: StaticStr::EMPTY,
    };

    /// Creates a wrapped static string literal. This function is equivalent to using the macro and
    /// is `const fn` so it can be used to initialize a constant at compile time with zero runtime cost.
    /// ```
    /// use flexstr::LocalStr;
    ///
    /// const S: LocalStr = LocalStr::from_static("test");
    /// assert!(S.is_static());
    /// ```
    #[inline]
    pub const fn from_static(s: &'static str) -> FlexStrWrapper<SIZE, PAD1, PAD2, HEAP> {
        FlexStrWrapper {
            static_str: StaticStr::from_static(s),
        }
    }

    /// Creates a new string from a str reference. If the string is empty, an empty static string
    /// is returned. If at or under the inline length limit, an inline string will be returned.
    /// Otherwise, a heap based string will be allocated and returned.
    #[inline]
    pub fn from_ref(s: impl AsRef<str>) -> Self
    where
        HEAP: for<'a> From<&'a str>,
    {
        let s = s.as_ref();

        if s.is_empty() {
            Self::EMPTY
        } else {
            match Self::try_inline(s) {
                Ok(s) => s,
                Err(_) => Self::from_ref_heap(s),
            }
        }
    }

    /// Attempts to create an inlined string. Returns a new inline string on success or the original
    /// source string if it will not fit. Since the to/into/from_ref functions will automatically
    /// inline when possible, this function is really only for special use cases.
    /// ```
    /// use flexstr::LocalStr;
    ///
    /// let s = LocalStr::try_inline("test").unwrap();
    /// assert!(s.is_inline());
    /// ```
    #[inline]
    pub fn try_inline<S: AsRef<str>>(s: S) -> Result<FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>, S> {
        match inline::InlineFlexStr::try_new(s) {
            Ok(s) => Ok(FlexStrWrapper { inline_str: s }),
            Err(s) => Err(s),
        }
    }

    /// Force the creation of a heap allocated string. Unlike to/into/from_ref functions, this will
    /// not attempt to inline first even if the string is a candidate for inlining. Using this is
    /// generally only recommended when using the associated `to_heap` and `try_to_heap` functions.
    /// ```
    /// use flexstr::LocalStr;
    ///
    /// let s = LocalStr::from_ref_heap("test");
    /// assert!(s.is_heap());
    /// ```
    #[inline]
    pub fn from_ref_heap(s: impl AsRef<str>) -> FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
    where
        HEAP: for<'a> From<&'a str>,
    {
        FlexStrWrapper {
            heap_str: ManuallyDrop::new(HeapStr::from_ref(s)),
        }
    }

    /// Create a new heap based string by wrapping the existing user provided heap string type (T).
    /// For `LocalStr` this will be an `Rc<str>` and for `SharedStr` it will be an `Arc<str>`.
    #[inline]
    pub fn from_heap(t: HEAP) -> FlexStrWrapper<SIZE, PAD1, PAD2, HEAP> {
        FlexStrWrapper {
            heap_str: ManuallyDrop::new(HeapStr::from_heap(t)),
        }
    }

    /// Returns the size of the maximum possible inline length for this type
    #[inline]
    pub fn inline_capacity() -> usize {
        SIZE
    }

    /// Attempts to extract a static inline string literal if one is stored inside this `LocalStr`.
    /// Returns `WrongStorageType` if this is not a static string literal.
    /// ```
    /// use flexstr::local_str;
    ///
    /// let s = "abc";
    /// let s2 = local_str!(s);
    /// assert_eq!(s2.try_as_static_str().unwrap(), s);
    /// ```
    #[inline]
    pub fn try_as_static_str(&self) -> Result<&'static str, WrongStorageType> {
        // SAFETY: Marker check is aligned to correct accessed field
        unsafe {
            match self.static_str.marker {
                StorageType::Static => Ok(self.static_str.literal),
                actual => Err(WrongStorageType {
                    expected: StorageType::Static,
                    actual,
                }),
            }
        }
    }

    /// Attempts to extract a copy of the heap value (for `LocalStr` this will be an `Rc<str>` and
    /// for `SharedStr` an `Arc<str>`) via cloning. If this is not a heap based string, a `WrongStorageType`
    /// error will be returned.
    #[inline]
    pub fn try_to_heap(&self) -> Result<HEAP, WrongStorageType>
    where
        HEAP: Clone,
    {
        // SAFETY: Marker check is aligned to correct accessed field
        unsafe {
            match self.heap_str.marker {
                StorageType::Heap => Ok(self.heap_str.heap.clone()),
                actual => Err(WrongStorageType {
                    expected: StorageType::Heap,
                    actual,
                }),
            }
        }
    }

    /// Returns a copy of the heap value (for `FlexStr` this will be an `Rc<str>` and
    /// for `SharedStr` an `Arc<str>`). If this is not a heap based string, a new value will be allocated
    /// and returned
    #[inline]
    pub fn to_heap(&self) -> HEAP
    where
        HEAP: Clone + for<'a> From<&'a str> + Deref<Target = str>,
    {
        match self.try_to_heap() {
            Ok(heap) => heap,
            Err(_) => self.as_str().into(),
        }
    }

    /// Returns true if this is a wrapped string literal (`&'static str`)
    /// ```
    /// use flexstr::LocalStr;
    ///
    /// let s = LocalStr::from_static("test");
    /// assert!(s.is_static());
    /// ```
    #[inline]
    pub fn is_static(&self) -> bool {
        // SAFETY: Marker is identical in all union fields
        unsafe { matches!(self.static_str.marker, StorageType::Static) }
    }

    /// Returns true if this is an inlined string
    /// ```
    /// use flexstr::LocalStr;
    ///
    /// let s = LocalStr::try_inline("test").unwrap();
    /// assert!(s.is_inline());
    /// ```
    #[inline]
    pub fn is_inline(&self) -> bool {
        // SAFETY: Marker is identical in all union fields
        unsafe { matches!(self.static_str.marker, StorageType::Inline) }
    }

    /// Returns true if this is a wrapped string using heap storage
    /// ```
    /// use flexstr::LocalStr;
    ///
    /// let s = LocalStr::from_ref_heap("test");
    /// assert!(s.is_heap());
    /// ```
    #[inline]
    pub fn is_heap(&self) -> bool {
        // SAFETY: Marker is identical in all union fields
        unsafe { matches!(self.static_str.marker, StorageType::Heap) }
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP>
    FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: Deref<Target = str>,
{
    /// Returns true if this `FlexStr` is empty
    /// ```
    /// use flexstr::ToLocalStr;
    ///
    /// let inlined = "".to_local_str();
    /// assert!(inlined.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the length of this `FlexStr` in bytes (not chars or graphemes)
    /// ```
    /// use flexstr::ToLocalStr;
    ///
    /// let inlined = "len".to_local_str();
    /// assert_eq!(inlined.len(), 3);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        // SAFETY: Marker check is aligned to correct accessed field
        unsafe {
            // Due to how inline does deref, I'm guessing this is slightly cheaper by using
            // inline native len instead of using len() off of `&str` at the top
            match self.static_str.marker {
                StorageType::Static => self.static_str.literal.len(),
                StorageType::Inline => self.inline_str.len(),
                StorageType::Heap => self.heap_str.heap.len(),
            }
        }
    }

    /// Extracts a string slice containing the entire `FlexStr`
    #[inline]
    pub fn as_str(&self) -> &str {
        self
    }

    /// Converts this `FlexStr` into a `String`. This should be more efficient than using the `ToString`
    /// trait (which we cannot implement due to a blanket stdlib implementation) as this avoids the
    /// `Display`-based implementation.
    /// ```
    /// use flexstr::local_str;
    ///
    /// let s = local_str!("abc").to_std_string();
    /// assert_eq!(s, "abc");
    /// ```
    #[inline]
    pub fn to_std_string(&self) -> String {
        String::from(&**self)
    }
}

// *** Debug / Display ***

// FIXME: Do we want to do something custom?
impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> Debug
    for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: Deref<Target = str>,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <str as Debug>::fmt(self, f)
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> Display
    for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: Deref<Target = str>,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <str as Display>::fmt(self, f)
    }
}

#[cfg(feature = "fast_format")]
impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> ufmt::uDisplay
    for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: Deref<Target = str>,
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
impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> ufmt::uDebug
    for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: Deref<Target = str>,
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

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> Hash
    for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: Deref<Target = str>,
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        str::hash(self, state)
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, HEAP2>
    PartialEq<FlexStrWrapper<SIZE, PAD1, PAD2, HEAP2>> for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: Deref<Target = str>,
    HEAP2: Deref<Target = str>,
{
    /// ```
    /// use flexstr::{SharedStr, LocalStr, ToFlex};
    ///
    /// let lit = "inlined";
    /// let s: LocalStr = lit.into();
    /// let s2: SharedStr = lit.into();
    /// assert_eq!(s, s2);
    /// ```
    #[inline]
    fn eq(&self, other: &FlexStrWrapper<SIZE, PAD1, PAD2, HEAP2>) -> bool {
        str::eq(self, &**other)
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, HEAP2>
    PartialEq<FlexStrWrapper<SIZE, PAD1, PAD2, HEAP2>> for &FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: Deref<Target = str>,
    HEAP2: Deref<Target = str>,
{
    /// ```
    /// use flexstr::{SharedStr, LocalStr, ToFlex};
    ///
    /// let lit = "inlined";
    /// let s: LocalStr = lit.into();
    /// let s2: SharedStr = lit.into();
    /// assert_eq!(&s, s2);
    /// ```
    #[inline]
    fn eq(&self, other: &FlexStrWrapper<SIZE, PAD1, PAD2, HEAP2>) -> bool {
        str::eq(self, &**other)
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> PartialEq<&str>
    for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: Deref<Target = str>,
{
    /// ```
    /// use flexstr::{LocalStr, ToFlex};
    ///
    /// let lit = "inlined";
    /// let s: LocalStr = lit.to_flex();
    /// assert_eq!(s, lit);
    /// ```
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        str::eq(self, *other)
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> PartialEq<str>
    for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: Deref<Target = str>,
{
    /// ```
    /// use flexstr::{LocalStr, ToFlex};
    ///
    /// let lit = "inlined";
    /// let s: LocalStr = lit.to_flex();
    /// assert_eq!(s, lit);
    /// ```
    #[inline]
    fn eq(&self, other: &str) -> bool {
        str::eq(self, other)
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> PartialEq<String>
    for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: Deref<Target = str>,
{
    /// ```
    /// use flexstr::LocalStr;
    ///
    /// let lit = "inlined";
    /// let s: LocalStr = lit.into();
    /// assert_eq!(s, lit.to_string());
    /// ```
    #[inline]
    fn eq(&self, other: &String) -> bool {
        str::eq(self, other)
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> Eq
    for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: Deref<Target = str>,
{
}

// *** PartialOrd / Ord ***

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> PartialOrd
    for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: Deref<Target = str>,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        str::partial_cmp(self, other)
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> PartialOrd<str>
    for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: Deref<Target = str>,
{
    #[inline]
    fn partial_cmp(&self, other: &str) -> Option<Ordering> {
        str::partial_cmp(self, other)
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> PartialOrd<String>
    for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: Deref<Target = str>,
{
    #[inline]
    fn partial_cmp(&self, other: &String) -> Option<Ordering> {
        str::partial_cmp(self, other)
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> Ord
    for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: Deref<Target = str>,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        str::cmp(self, other)
    }
}

// *** Index ***

macro_rules! impl_ranges {
    ($($type:ty),+) => {
        $(impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> Index<$type> for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
        where
            HEAP: Deref<Target = str>,
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
fn concat<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP>(
    s1: &str,
    s2: &str,
) -> FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: for<'a> From<&'a str>,
{
    let mut buffer = buffer_new!(SIZE);
    let mut builder = builder_new!(buffer, s1.len() + s2.len());
    builder.str_write(s1);
    builder.str_write(s2);
    builder_into!(builder, buffer)
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> Add<&str>
    for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: for<'a> From<&'a str> + Deref<Target = str>,
{
    type Output = FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>;

    /// ```
    /// use flexstr::{local_str, IntoLocalStr};
    ///
    /// let a = local_str!("in") + "line";
    /// assert!(a.is_inline());
    /// assert_eq!(a, "inline");
    ///
    /// let a = "in".to_string().into_local_str() + "line";
    /// assert!(a.is_inline());
    /// assert_eq!(a, "inline");
    /// ```
    #[inline]
    fn add(mut self, rhs: &str) -> Self::Output {
        if rhs.is_empty() {
            self
        } else if self.is_empty() {
            rhs.into()
        } else {
            // SAFETY: Marker check is aligned to correct accessed field
            unsafe {
                match self.static_str.marker {
                    StorageType::Static => concat(self.static_str.literal, rhs),
                    StorageType::Inline => {
                        let s = &mut self.inline_str;

                        if s.try_concat(rhs) {
                            self
                        } else {
                            concat(s, rhs)
                        }
                    }
                    StorageType::Heap => concat(&self.heap_str.heap, rhs),
                }
            }
        }
    }
}

// *** Misc. standard traits ***

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> AsRef<str>
    for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: Deref<Target = str>,
{
    #[inline]
    fn as_ref(&self) -> &str {
        self
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> Default
    for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
{
    #[inline]
    fn default() -> Self {
        Self::from_static("")
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> FromStr
    for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: for<'a> From<&'a str>,
{
    type Err = Infallible;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

// *** From ***

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, HEAP2>
    From<&FlexStrWrapper<SIZE, PAD1, PAD2, HEAP2>> for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: for<'a> From<&'a str>,
    HEAP2: Clone + Deref<Target = str>,
{
    #[inline]
    fn from(s: &FlexStrWrapper<SIZE, PAD1, PAD2, HEAP2>) -> Self {
        s.clone().into_flex()
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> From<String>
    for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: for<'a> From<&'a str>,
{
    /// ```
    /// use flexstr::LocalStr;
    ///
    /// let lit = "inlined";
    /// let s: LocalStr = lit.to_string().into();
    /// assert!(s.is_inline());
    /// assert_eq!(&s, lit);
    ///
    /// let lit = "This is too long too be inlined!";
    /// let s: LocalStr = lit.to_string().into();
    /// assert!(s.is_heap());
    /// assert_eq!(&s, lit);
    /// ```
    #[inline]
    fn from(s: String) -> Self {
        <Self as From<&str>>::from(&s)
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> From<&String>
    for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: for<'a> From<&'a str>,
{
    /// ```
    /// use flexstr::LocalStr;
    ///
    /// let lit = "inlined";
    /// let s: LocalStr = (&lit.to_string()).into();
    /// assert!(s.is_inline());
    /// assert_eq!(&s, lit);
    ///
    /// let lit = "This is too long too be inlined!";
    /// let s: LocalStr = (&lit.to_string()).into();
    /// assert!(s.is_heap());
    /// assert_eq!(&s, lit);
    /// ```
    #[inline]
    fn from(s: &String) -> Self {
        <Self as From<&str>>::from(s)
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> From<&str>
    for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: for<'a> From<&'a str>,
{
    /// ```
    /// use flexstr::LocalStr;
    ///
    /// let lit = "inline";
    /// let s: LocalStr  = lit.into();
    /// assert!(s.is_inline());
    /// assert_eq!(&s, lit);
    /// ```
    #[inline]
    fn from(s: &str) -> Self {
        Self::from_ref(s)
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> From<char>
    for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: Deref<Target = str>,
{
    /// ```
    /// use flexstr::LocalStr;
    ///
    /// let s: LocalStr  = 't'.into();
    /// assert!(s.is_inline());
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
fn from_iter_str<const SIZE: usize, const PAD1: usize, const PAD2: usize, I, HEAP, U>(
    iter: I,
) -> FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    I: IntoIterator<Item = U>,
    HEAP: for<'b> From<&'b str>,
    U: AsRef<str>,
{
    let iter = iter.into_iter();

    // Since `IntoIterator` consumes, we cannot loop over it twice to find lengths of strings
    // for a good capacity # without cloning it (which might be expensive)
    let mut buffer = buffer_new!(SIZE);
    let mut builder = builder_new!(buffer);
    for s in iter {
        builder.str_write(s);
    }
    builder_into!(builder, buffer)
}

#[inline]
fn from_iter_char<const SIZE: usize, const PAD1: usize, const PAD2: usize, I, F, HEAP, U>(
    iter: I,
    f: F,
) -> FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    I: IntoIterator<Item = U>,
    F: Fn(U) -> char,
    HEAP: for<'b> From<&'b str>,
{
    let iter = iter.into_iter();
    let (lower, _) = iter.size_hint();

    let mut buffer = buffer_new!(SIZE);
    let mut builder = builder_new!(buffer, lower);
    for ch in iter {
        builder.char_write(f(ch));
    }
    builder_into!(builder, buffer)
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, HEAP2>
    FromIterator<FlexStrWrapper<SIZE, PAD1, PAD2, HEAP2>> for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: for<'b> From<&'b str>,
    HEAP2: Deref<Target = str>,
{
    /// ```
    /// use flexstr::LocalStr;
    ///
    /// let v: Vec<LocalStr> = vec!["best".into(), "test".into()];
    /// let s: LocalStr = v.into_iter().map(|s| if s == "best" { "test".into() } else { s }).collect();
    /// assert!(s.is_inline());
    /// assert_eq!(s, "testtest");
    /// ```
    #[inline]
    fn from_iter<I: IntoIterator<Item = FlexStrWrapper<SIZE, PAD1, PAD2, HEAP2>>>(iter: I) -> Self {
        from_iter_str(iter)
    }
}

impl<'a, const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, HEAP2>
    FromIterator<&'a FlexStrWrapper<SIZE, PAD1, PAD2, HEAP2>>
    for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: for<'b> From<&'b str>,
    HEAP2: Deref<Target = str> + 'a,
{
    /// ```
    /// use flexstr::LocalStr;
    ///
    /// let v: Vec<LocalStr> = vec!["best".into(), "test".into()];
    /// let s: LocalStr = v.iter().filter(|s| *s == "best").collect();
    /// assert!(s.is_inline());
    /// assert_eq!(s, "best");
    /// ```
    #[inline]
    fn from_iter<I: IntoIterator<Item = &'a FlexStrWrapper<SIZE, PAD1, PAD2, HEAP2>>>(
        iter: I,
    ) -> Self {
        from_iter_str(iter)
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> FromIterator<String>
    for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: for<'b> From<&'b str>,
{
    /// ```
    /// use flexstr::LocalStr;
    ///
    /// let v = vec!["best".to_string(), "test".to_string()];
    /// let s: LocalStr = v.into_iter().map(|s| if s == "best" { "test".into() } else { s }).collect();
    /// assert!(s.is_inline());
    /// assert_eq!(s, "testtest");
    /// ```
    #[inline]
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        from_iter_str(iter)
    }
}

impl<'a, const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> FromIterator<&'a str>
    for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: for<'b> From<&'b str>,
{
    /// ```
    /// use flexstr::LocalStr;
    ///
    /// let v = vec!["best", "test"];
    /// let s: LocalStr = v.into_iter().map(|s| if s == "best" { "test" } else { s }).collect();
    /// assert!(s.is_inline());
    /// assert_eq!(s, "testtest");
    /// ```
    #[inline]
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        from_iter_str(iter)
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> FromIterator<char>
    for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: for<'b> From<&'b str>,
{
    /// ```
    /// use flexstr::LocalStr;
    ///
    /// let v = "besttest";
    /// let s: LocalStr = v.chars().map(|c| if c == 'b' { 't' } else { c }).collect();
    /// assert!(s.is_inline());
    /// assert_eq!(s, "testtest");
    /// ```
    #[inline]
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
        from_iter_char(iter, |ch| ch)
    }
}

impl<'a, const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> FromIterator<&'a char>
    for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: for<'b> From<&'b str>,
{
    /// ```
    /// use flexstr::LocalStr;
    ///
    /// let v = vec!['b', 'e', 's', 't', 't', 'e', 's', 't'];
    /// let s: LocalStr = v.iter().filter(|&ch| *ch != 'b').collect();
    /// assert!(s.is_inline());
    /// assert_eq!(s, "esttest");
    /// ```
    #[inline]
    fn from_iter<I: IntoIterator<Item = &'a char>>(iter: I) -> Self {
        from_iter_char(iter, |ch| *ch)
    }
}

// *** Optional serialization support ***

#[cfg(feature = "serde")]
impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> Serialize
    for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: Deref<Target = str>,
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
struct FlexStrVisitor<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP>(
    PhantomData<*const HEAP>,
);

#[cfg(feature = "serde")]
impl<'de, const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> Visitor<'de>
    for FlexStrVisitor<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: for<'a> From<&'a str>,
{
    type Value = FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>;

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
impl<'de, const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> Deserialize<'de>
    for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: for<'a> From<&'a str>,
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
/// `LocalStr::from_static("my_literal")`
/// ```
/// use flexstr::{local_str, LocalStr};
///
/// const STR: LocalStr = local_str!("This is a constant!");
/// assert!(STR.is_static())
/// ```
#[macro_export]
macro_rules! local_str {
    ($str:expr) => {
        <$crate::LocalStr>::from_static($str)
    };
}

/// Create compile time constant `SharedStr` (equivalent, but less typing than:
/// `<SharedStr>::from_static("my_literal")`
/// ```
/// use flexstr::{shared_str, SharedStr};
///
/// const STR: SharedStr = shared_str!("This is a constant!");
/// assert!(STR.is_static())
/// ```
#[macro_export]
macro_rules! shared_str {
    ($str:expr) => {
        <$crate::SharedStr>::from_static($str)
    };
}

/// `FlexStr` equivalent to `format` function from stdlib. Efficiently creates a native `FlexStr`
pub fn flex_fmt<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP>(
    args: Arguments<'_>,
) -> FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: for<'a> From<&'a str>,
{
    // NOTE: We have a disadvantage to `String` because we cannot call `estimated_capacity()` on args
    // As such, we cannot assume a given needed capacity - we start with a stack allocated buffer
    // and only promote to a heap buffer if a write won't fit
    let mut buffer = buffer_new!(SIZE);
    let mut builder = builder_new!(buffer);
    builder
        .write_fmt(args)
        .expect("a formatting trait implementation returned an error");
    builder_into!(builder, buffer)
}

/// Equivalent to `local_fmt` except that it uses `ufmt` which is much faster, but has limitations.
/// See [ufmt docs](https://docs.rs/ufmt/latest/ufmt/) for more details
/// ```
/// use flexstr::{local_str, local_ufmt};
///
/// let a = local_ufmt!("Is {}{}", local_str!("inlined"), "!");
/// assert!(a.is_inline());
/// assert_eq!(a, "Is inlined!");
/// ```
#[cfg(feature = "fast_format")]
#[macro_export(local_inner_macros)]
macro_rules! local_ufmt {
    ($($arg:tt)*) => {{
        let mut buffer = buffer_new!({ $crate::STRING_SIZED_INLINE });
        let mut builder = builder_new!(buffer);

        ufmt::uwrite!(&mut builder, $($arg)*).expect("a formatting trait implementation returned an error");
        let s: $crate::LocalStr = builder_into!(builder, buffer);
        s
    }}
}

/// Equivalent to `shared_fmt` except that it uses `ufmt` which is much faster, but has limitations.
/// See [ufmt docs](https://docs.rs/ufmt/latest/ufmt/) for more details
/// ```
/// use flexstr::{shared_str, shared_ufmt};
///
/// let a = shared_ufmt!("Is {}{}", shared_str!("inlined"), "!");
/// assert!(a.is_inline());
/// assert_eq!(a, "Is inlined!");
/// ```
#[cfg(feature = "fast_format")]
#[macro_export(local_inner_macros)]
macro_rules! shared_ufmt {
    ($($arg:tt)*) => {{
        let mut buffer = buffer_new!({ $crate::STRING_SIZED_INLINE });
        let mut builder = builder_new!(buffer);

        ufmt::uwrite!(&mut builder, $($arg)*).expect("a formatting trait implementation returned an error");
        let s: $crate::SharedStr = builder_into!(builder, buffer);
        s
    }}
}

/// `FlexStr` equivalent to `format!` macro from stdlib. Efficiently creates a native `FlexStr`
/// ```
/// use flexstr::local_fmt;
///
/// let a = local_fmt!("Is {}", "inlined");
/// assert!(a.is_inline());
/// assert_eq!(a, "Is inlined")
/// ```
#[macro_export]
macro_rules! local_fmt {
    ($($arg:tt)*) => {{
        let s: flexstr::LocalStr = flexstr::flex_fmt(format_args!($($arg)*));
        s
    }}
}

/// `SharedStr` equivalent to `format!` macro from stdlib. Efficiently creates a native `SharedStr`
/// ```
/// use flexstr::shared_fmt;
///
/// let a = shared_fmt!("Is {}", "inlined");
/// assert!(a.is_inline());
/// assert_eq!(a, "Is inlined")
/// ```

#[macro_export]
macro_rules! shared_fmt {
    ($($arg:tt)*) => {{
        let s: flexstr::SharedStr = flexstr::flex_fmt(format_args!($($arg)*));
        s
    }}
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "serde")]
    #[test]
    fn serialization() {
        use crate::{LocalStr, SharedStr};
        use alloc::string::ToString;
        use serde_json::json;

        #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq)]
        struct Test {
            a: LocalStr,
            b: SharedStr,
            c: LocalStr,
        }

        let a = "test";
        let b = "testing";
        let c = "testing testing testing testing testing testing testing testing testing";

        // Create our struct and values and verify storage
        let test = Test {
            a: local_str!(a),
            b: b.to_string().into(),
            c: c.to_string().into(),
        };
        assert!(test.a.is_static());
        assert!(test.b.is_inline());
        assert!(test.c.is_heap());

        // Serialize and ensure our JSON value actually matches
        let val = serde_json::to_value(test.clone()).unwrap();
        assert_eq!(json!({"a": a, "b": b, "c": c}), val);

        // Deserialize and validate storage and contents
        let test2: Test = serde_json::from_value(val).unwrap();
        assert!(test2.a.is_inline());
        assert!(test2.b.is_inline());
        assert!(test2.c.is_heap());

        assert_eq!(&test, &test2);
    }
}
