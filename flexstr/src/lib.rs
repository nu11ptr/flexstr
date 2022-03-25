#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

//! A flexible, simple to use, immutable, clone-efficient [String] replacement for Rust
//!
//! ## String Creation from Literals
//!
//! String constants are easily wrapped into the unified string type. String contents are inlined
//! when possible otherwise allocated on the heap.
//!
//! ```
//! use flexstr::{local_str, LocalStr, ToLocalStr};
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
//! ```
//!
//! ## String Creation and Manipulation
//!
//! The stdlib [format] macro equivalent is used to create brand new strings. String operations like
//! changing case and concatenation are efficiently supported (inlining when possible).
//!
//! ```
//! use flexstr::{local_fmt, LocalStr, ToCase};
//!
//! // You can efficiently create a new `LocalStr` (without creating a `String`)
//! // This is equivalent to the stdlib `format!` macro
//! let inline_str = local_fmt!("in{}", "lined");
//! assert!(inline_str.is_inline());
//!
//! // We can upper/lowercase strings without converting to a `String` first
//! // This doesn't heap allocate since inlined
//! let inline_str2: LocalStr = "INLINED".to_ascii_lower();
//! assert!(inline_str2.is_inline());
//! assert_eq!(inline_str, inline_str2);
//!
//! // Concatenation doesn't even copy if we can fit it in the inline string
//! let inline_str3 = inline_str2 + "!!!";
//! assert!(inline_str3.is_inline());
//! assert_eq!(inline_str3, "inlined!!!");
//! ```
//!
//! ## Efficient, Universal String Type
//!
//! Clones never copy or allocate and are very fast. Regardless
//! of underlying storage type, all strings work together and resulting strings automatically
//! choose the best storage.
//!
//! ```
//! use flexstr::{local_str, LocalStr, ToLocalStr};
//!
//! // Clone is cheap, and never allocates
//! // (at most it is a ref count increment for heap allocated strings)
//! let rc_str = "This is too long to be inlined".to_local_str().clone();
//! assert!(rc_str.is_heap());
//!
//! // Regardless of storage type, these all operate seamlessly together
//! // and choose storage as required
//! const STATIC_STR: LocalStr = local_str!("This will eventually end up on the ");
//! let inline_str = "heap".to_local_str();
//!
//! let heap_str2 = STATIC_STR + &inline_str;
//! assert!(heap_str2.is_heap());
//! assert_eq!(heap_str2, "This will eventually end up on the heap");  
//! ```

extern crate alloc;

#[doc(hidden)]
#[macro_use]
pub mod builder;
#[doc(hidden)]
mod impls;
mod macros;
#[doc(hidden)]
pub mod storage;
#[doc(hidden)]
pub mod traits;

use alloc::rc::Rc;
use alloc::string::String;
use alloc::sync::Arc;
use core::fmt::{Arguments, Write};
use core::marker::PhantomData;
use core::mem;
use core::mem::ManuallyDrop;
use core::ops::Deref;

use static_assertions::{assert_eq_align, assert_eq_size, assert_impl_all, assert_not_impl_any};

use crate::storage::heap::HeapStr;
use crate::storage::inline::InlineFlexStr;
pub use crate::storage::inline::STRING_SIZED_INLINE;
use crate::storage::static_ref::StaticStr;
pub use crate::storage::{StorageType, WrongStorageType};
#[doc(inline)]
pub use crate::traits::*;

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

assert_eq_size!(LocalStr, String);
assert_eq_size!(SharedStr, String);
assert_not_impl_any!(LocalStr: Send, Sync);
assert_impl_all!(SharedStr: Send, Sync);

assert_eq_size!(HeapStr<PTR_SIZED_PAD, Rc<str>>, InlineFlexStr<STRING_SIZED_INLINE>);
assert_eq_size!(StaticStr<PTR_SIZED_PAD>, InlineFlexStr<STRING_SIZED_INLINE>);
assert_eq_align!(HeapStr<PTR_SIZED_PAD, Rc<str>>, InlineFlexStr<STRING_SIZED_INLINE>);
assert_eq_align!(StaticStr<PTR_SIZED_PAD>, InlineFlexStr<STRING_SIZED_INLINE>);

const BAD_SIZE_OR_ALIGNMENT: &str = "OOPS! It seems you are trying to create a custom `FlexStr` but have \
violated the invariants on size and alignment. It is recommended to only try and use `FlexStrBase` \
and pick a storage type with a size of exactly two machine words (16 bytes on 64-bit, 8 bytes on 32-bit). \
Creating a custom type based directly on the `FlexStr` union is possible, but it is difficult to calculate \
all the type parameters correctly and is therefore not recommended.";

/// Padding the size of a pointer for this platform minus one
pub const PTR_SIZED_PAD: usize = mem::size_of::<*const ()>() - 1;

/// A flexible string type that transparently wraps a string literal, inline string, or a heap allocated type
///
/// # Note
/// It is not generally recommended to try and create direct custom concrete types of `FlexStr` as it
/// is complicated to calculate the correct sizes of all the generic type parameters. However, be aware
/// that a runtime panic will be issued on creation if incorrect, so if you are able to create a string
/// of your custom type, your parameters were of correct size/alignment.
pub union FlexStr<
    const SIZE: usize,
    const PAD1: usize,
    const PAD2: usize,
    HEAP,
    STR: ?Sized,
    STRING,
> {
    static_str: StaticStr<PAD1>,
    inline_str: InlineFlexStr<SIZE>,
    heap_str: mem::ManuallyDrop<HeapStr<PAD2, HEAP>>,
    phantom: PhantomData<(STRING, STR)>,
}

/// A flexible base string type that transparently wraps a string literal, inline string, or a custom `HEAP` type
///
/// # Note
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexStr]
///
/// # Note 2
/// Custom concrete types need to specify a `HEAP` type with an exact size of two machine words (16 bytes
/// on 64-bit, and 8 bytes on 32-bit). Any other sized parameter will result in a runtime panic on string
/// creation.
pub type FlexStrBase<HEAP> =
    FlexStr<STRING_SIZED_INLINE, PTR_SIZED_PAD, PTR_SIZED_PAD, HEAP, str, String>;

/// A flexible string type that transparently wraps a string literal, inline string, or an [`Rc<str>`]
///
/// # Note
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexStr]
pub type LocalStr = FlexStrBase<Rc<str>>;

/// A flexible string type that transparently wraps a string literal, inline string, or an [`Arc<str>`]
///
/// # Note
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexStr]
pub type SharedStr = FlexStrBase<Arc<str>>;

// *** Clone ***

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR, STRING> Clone
    for FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    HEAP: Clone,
    STR: ?Sized,
{
    #[inline]
    fn clone(&self) -> Self {
        // SAFETY: Marker check is aligned to correct accessed field
        unsafe {
            // TODO: Replace raw union construction with inline calls to special `from_` functions? (while watching benchmarks closely!)
            match self.static_str.marker {
                StorageType::Static => FlexStr {
                    static_str: self.static_str,
                },
                StorageType::Inline => FlexStr {
                    inline_str: self.inline_str,
                },
                StorageType::Heap => FlexStr {
                    // Recreating vs. calling clone at the top is 30% faster in benchmarks
                    heap_str: ManuallyDrop::new(HeapStr::from_heap(self.heap_str.heap.clone())),
                },
            }
        }
    }
}

// *** Drop ***

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR, STRING> Drop
    for FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    STR: ?Sized,
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

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR, STRING> Deref
    for FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    HEAP: Deref<Target = str>,
    STR: ?Sized,
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

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR, STRING>
    FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    STR: ?Sized,
{
    /// An empty ("") static constant string
    pub const EMPTY: Self = if Self::IS_VALID_SIZE {
        FlexStr {
            static_str: StaticStr::EMPTY,
        }
    } else {
        panic!("{}", BAD_SIZE_OR_ALIGNMENT);
    };

    // If the union variants aren't the precise right size bad things will happen - we protect against that
    const IS_VALID_SIZE: bool = Self::variant_sizes_are_valid();

    #[inline]
    const fn variant_sizes_are_valid() -> bool {
        mem::size_of::<HeapStr<PAD2, HEAP>>() == mem::size_of::<InlineFlexStr<SIZE>>()
            && mem::size_of::<StaticStr<PAD1>>() == mem::size_of::<InlineFlexStr<SIZE>>()
            && mem::align_of::<HeapStr<PAD2, HEAP>>() == mem::align_of::<InlineFlexStr<SIZE>>()
            && mem::align_of::<StaticStr<PAD1>>() == mem::align_of::<InlineFlexStr<SIZE>>()
    }

    /// Creates a wrapped static string literal. This function is equivalent to using the macro and
    /// is `const fn` so it can be used to initialize a constant at compile time with zero runtime cost.
    /// ```
    /// use flexstr::LocalStr;
    ///
    /// const S: LocalStr = LocalStr::from_static("test");
    /// assert!(S.is_static());
    /// ```
    #[inline]
    pub const fn from_static(s: &'static str) -> Self {
        if Self::IS_VALID_SIZE {
            FlexStr {
                static_str: StaticStr::from_static(s),
            }
        } else {
            panic!("{}", BAD_SIZE_OR_ALIGNMENT);
        }
    }

    /// Creates a new string from a [str] reference. If the string is empty, an empty static string
    /// is returned. If at or under the inline length limit, an inline string will be returned.
    /// Otherwise, a heap based string will be allocated and returned. This is typically used to
    /// create strings from a non-static borrowed [str] where you don't have ownership.
    /// ```
    /// use flexstr::LocalStr;
    ///
    /// let s: LocalStr = LocalStr::from_ref("");
    /// assert!(s.is_static());
    ///
    /// let s: LocalStr = LocalStr::from_ref("test");
    /// assert!(s.is_inline());
    ///
    /// let s: LocalStr = LocalStr::from_ref("This is too long to be inlined!!!!!!!");
    /// assert!(s.is_heap());
    /// ```
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

    #[doc(hidden)]
    #[inline]
    pub fn from_inline(s: InlineFlexStr<SIZE>) -> Self {
        if Self::IS_VALID_SIZE {
            FlexStr { inline_str: s }
        } else {
            panic!("{}", BAD_SIZE_OR_ALIGNMENT);
        }
    }

    /// Attempts to create an inlined string. Returns a new inline string on success or the original
    /// source string if it will not fit. Since the to/into/[from_ref](FlexStr::from_ref) functions
    /// will automatically inline when possible, this function is really only for special use cases.
    /// ```
    /// use flexstr::LocalStr;
    ///
    /// let s = LocalStr::try_inline("test").unwrap();
    /// assert!(s.is_inline());
    /// ```
    #[inline]
    pub fn try_inline<S: AsRef<str>>(s: S) -> Result<Self, S> {
        match InlineFlexStr::try_new(s) {
            Ok(s) => Ok(Self::from_inline(s)),
            Err(s) => Err(s),
        }
    }

    /// Force the creation of a heap allocated string. Unlike to/into/[from_ref](FlexStr::from_ref)
    /// functions, this will not attempt to inline first even if the string is a candidate for inlining.
    /// Using this is generally only recommended when using the associated [to_heap](FlexStr::to_heap)
    /// and [try_to_heap](FlexStr::try_to_heap) functions.
    /// ```
    /// use flexstr::LocalStr;
    ///
    /// let s = LocalStr::from_ref_heap("test");
    /// assert!(s.is_heap());
    /// ```
    #[inline]
    pub fn from_ref_heap(s: impl AsRef<str>) -> Self
    where
        HEAP: for<'a> From<&'a str>,
    {
        if Self::IS_VALID_SIZE {
            FlexStr {
                heap_str: ManuallyDrop::new(HeapStr::from_ref(s)),
            }
        } else {
            panic!("{}", BAD_SIZE_OR_ALIGNMENT);
        }
    }

    /// Create a new heap based string by wrapping the existing user provided heap string type (T).
    /// For [LocalStr] this will be an [`Rc<str>`] and for [SharedStr] it will be an [`Arc<str>`].
    /// This would typically only be used if efficient unwrapping of heap based data is needed at
    /// a later time.
    /// ```
    /// use flexstr::LocalStr;
    ///
    /// let s = LocalStr::from_heap("test".into());
    /// assert!(s.is_heap());
    /// ```
    #[inline]
    pub fn from_heap(t: HEAP) -> Self {
        if Self::IS_VALID_SIZE {
            FlexStr {
                heap_str: ManuallyDrop::new(HeapStr::from_heap(t)),
            }
        } else {
            panic!("{}", BAD_SIZE_OR_ALIGNMENT);
        }
    }

    #[inline]
    fn from_char(ch: char) -> Self {
        // SAFETY: Regardless of architecture, 4 bytes will always fit in an inline string
        unsafe { Self::try_inline(ch.encode_utf8(&mut [0; 4])).unwrap_unchecked() }
    }

    /// Returns the size of the maximum possible inline length for this type
    /// ```
    /// use flexstr::{LocalStr, STRING_SIZED_INLINE};
    ///
    /// assert_eq!(LocalStr::inline_capacity(), STRING_SIZED_INLINE);
    /// ```
    #[inline]
    pub fn inline_capacity() -> usize {
        SIZE
    }

    /// Attempts to extract a static inline string literal if one is stored inside this [LocalStr].
    /// Returns [WrongStorageType] if this is not a static string literal.
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

    /// Attempts to extract a copy of the heap value (for [LocalStr] this will be an [`Rc<str>`] and
    /// for [SharedStr] an [`Arc<str>`]) via cloning. If this is not a heap based string, a
    /// [WrongStorageType] error will be returned.
    /// ```
    /// use flexstr::LocalStr;
    ///
    /// let s = LocalStr::from_heap("test".into());
    /// assert!(s.is_heap());
    /// assert_eq!(s.try_to_heap().unwrap(), "test".into());
    /// ```
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

    /// Returns a copy of the heap value (for [FlexStr] this will be an [`Rc<str>`] and
    /// for [SharedStr] an [`Arc<str>`]). If this is not a heap based string, a new value will be allocated
    /// and returned
    /// ```
    /// use flexstr::{local_str, LocalStr};
    ///
    /// const S: LocalStr = local_str!("static");
    /// assert!(S.is_static());
    /// assert_eq!(S.to_heap(), "static".into());
    /// ```
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

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR, STRING>
    FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    HEAP: Deref<Target = str>,
    STR: ?Sized,
{
    /// Returns true if this [FlexStr] is empty
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

    /// Returns the length of this [FlexStr] in bytes (not chars or graphemes)
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

    /// Extracts a string slice containing the entire [FlexStr]
    /// ```
    /// use flexstr::ToLocalStr;
    ///
    /// let s = "abc".to_local_str();
    /// assert_eq!(s.as_str(), "abc");
    /// ```
    #[inline]
    pub fn as_str(&self) -> &str {
        self
    }

    /// Converts this [FlexStr] into a [String]. This should be more efficient than using the [ToString]
    /// trait (which we cannot implement due to a blanket stdlib implementation) as this avoids the
    /// [Display](alloc::fmt::Display)-based implementation.
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

    #[inline]
    fn add(mut self, rhs: &str) -> Self
    where
        HEAP: for<'a> From<&'a str>,
    {
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

#[inline]
fn concat<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR, STRING>(
    s1: &str,
    s2: &str,
) -> FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    HEAP: for<'a> From<&'a str>,
    STR: ?Sized,
{
    let mut buffer = buffer_new!(SIZE);
    let mut builder = builder_new!(buffer, s1.len() + s2.len());
    builder.str_write(s1);
    builder.str_write(s2);
    builder_into!(builder, buffer)
}

#[inline]
fn from_iter_str<const SIZE: usize, const PAD1: usize, const PAD2: usize, I, HEAP, U, STR, STRING>(
    iter: I,
) -> FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    I: IntoIterator<Item = U>,
    HEAP: for<'b> From<&'b str>,
    U: AsRef<str>,
    STR: ?Sized,
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
fn from_iter_char<
    const SIZE: usize,
    const PAD1: usize,
    const PAD2: usize,
    I,
    F,
    HEAP,
    U,
    STR,
    STRING,
>(
    iter: I,
    f: F,
) -> FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    I: IntoIterator<Item = U>,
    F: Fn(U) -> char,
    HEAP: for<'b> From<&'b str>,
    STR: ?Sized,
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

/// Equivalent to the [format](std::fmt::format) function from stdlib. Efficiently creates a native [FlexStr]
pub fn flex_fmt<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR, STRING>(
    args: Arguments<'_>,
) -> FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    HEAP: for<'a> From<&'a str>,
    STR: ?Sized,
{
    // NOTE: We have a disadvantage to [String] because we cannot call `estimated_capacity()` on args
    // As such, we cannot assume a given needed capacity - we start with a stack allocated buffer
    // and only promote to a heap buffer if a write won't fit
    let mut buffer = buffer_new!(SIZE);
    let mut builder = builder_new!(buffer);
    builder
        .write_fmt(args)
        .expect("a formatting trait implementation returned an error");
    builder_into!(builder, buffer)
}
