#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

//! A flexible, simple to use, immutable, clone-efficient [String] replacement for Rust

extern crate alloc;

pub mod custom;
mod storage;
mod string;

pub use crate::string::std_str::{
    BoxedStr, BoxedStrRef, FlexStr, LocalStr, LocalStrRef, SharedStr, SharedStrRef, EMPTY,
};

/// Provides support for [BStr](bstr::BStr)-based [FlexStrBase] strings
#[cfg(feature = "bstr")]
pub mod b_str {
    pub use crate::string::b_str::{
        BoxedBStr, BoxedBStrRef, FlexBStr, LocalBStr, LocalBStrRef, SharedBStr, SharedBStrRef,
    };
}

/// Provides support for [CStr](std::ffi::CStr)-based [FlexStrBase] strings
#[cfg(feature = "std")]
pub mod c_str {
    pub use crate::string::c_str::{
        BoxedCStr, BoxedCStrRef, CStrNullError, FlexCStr, LocalCStr, LocalCStrRef, SharedCStr,
        SharedCStrRef, EMPTY,
    };
}

/// Provides support for [OsStr](std::ffi::OsStr)-based [FlexStrBase] strings
#[cfg(feature = "std")]
pub mod os_str {
    pub use crate::string::os_str::{
        BoxedOsStr, BoxedOsStrRef, FlexOsStr, LocalOsStr, LocalOsStrRef, SharedOsStr,
        SharedOsStrRef,
    };
}

/// Provides support for raw [`[u8]`](slice)-based [FlexStrBase] strings
pub mod raw_str {
    pub use crate::string::raw_str::{
        BoxedRawStr, BoxedRawStrRef, FlexRawStr, LocalRawStr, LocalRawStrRef, SharedRawStr,
        SharedRawStrRef, EMPTY,
    };
}

use core::mem;

use crate::custom::BAD_SIZE_OR_ALIGNMENT;
use crate::storage::{BorrowStr, HeapStr, InlineStr, Storage, StorageType};
use crate::string::Str;

/// This serves as the base type for the whole crate. Most methods are listed here.
///
/// A flexible string base type that transparently wraps a string literal, inline string, a heap
/// allocated type, or a borrowed string (with appropriate lifetime).
///
/// # Note
/// It is not generally recommended to try and create direct custom concrete types of from this type as it
/// is complicated to calculate the correct sizes of all the generic type parameters. However, be aware
/// that a runtime panic will be issued on creation if incorrect, so if you are able to create a string
/// of your custom type, your parameters were of correct size/alignment.

// Cannot yet reference associated types from a generic param (impl trait) for const generic params,
// so we are forced to work with raw const generics for now. Also, cannot call const fn functions
// with a trait that has bounds other than `Size` atm.
pub union FlexStrBase<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP, STR>
where
    STR: ?Sized + 'static,
{
    static_str: mem::ManuallyDrop<BorrowStr<BPAD, &'static STR>>,
    inline_str: mem::ManuallyDrop<InlineStr<SIZE, STR>>,
    heap_str: mem::ManuallyDrop<HeapStr<HPAD, HEAP, STR>>,
    borrow_str: mem::ManuallyDrop<BorrowStr<BPAD, &'str STR>>,
}

// *** Clone ***

impl<'str, const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR> Clone
    for FlexStrBase<'str, SIZE, PAD1, PAD2, HEAP, STR>
where
    HEAP: Storage<STR> + Clone,
    STR: Str + ?Sized,
{
    #[inline]
    fn clone(&self) -> Self {
        // SAFETY: Marker check is aligned to correct accessed field
        unsafe {
            // TODO: Replace raw union construction with inline calls to special `from_` functions?
            // (while watching benchmarks closely!)
            match self.static_str.marker {
                StorageType::Static => Self {
                    static_str: self.static_str,
                },
                StorageType::Inline => Self {
                    inline_str: self.inline_str,
                },
                StorageType::Heap => Self {
                    // Recreating vs. calling clone at the top is 30% faster in benchmarks
                    heap_str: mem::ManuallyDrop::new(HeapStr::from_heap(
                        self.heap_str.heap.clone(),
                    )),
                },
                StorageType::Borrow => Self {
                    borrow_str: self.borrow_str,
                },
            }
        }
    }
}

// *** Drop ***

impl<'str, const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR> Drop
    for FlexStrBase<'str, SIZE, PAD1, PAD2, HEAP, STR>
where
    STR: ?Sized,
{
    #[inline]
    fn drop(&mut self) {
        // SAFETY: Marker check is aligned to correct accessed field
        unsafe {
            if let StorageType::Heap = self.heap_str.marker {
                mem::ManuallyDrop::drop(&mut self.heap_str);
            }
        }
    }
}

impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP, STR>
    FlexStrBase<'str, SIZE, BPAD, HPAD, HEAP, STR>
where
    STR: ?Sized + 'static,
{
    // If the union variants aren't the precise right size bad things will happen - we protect against that
    const IS_VALID_SIZE: bool = Self::variant_sizes_are_valid();

    #[inline]
    const fn variant_sizes_are_valid() -> bool {
        mem::size_of::<HeapStr<HPAD, HEAP, STR>>() == mem::size_of::<InlineStr<SIZE, STR>>()
            && mem::size_of::<BorrowStr<BPAD, &'static STR>>()
                == mem::size_of::<InlineStr<SIZE, STR>>()
            && mem::align_of::<HeapStr<HPAD, HEAP, STR>>()
                == mem::align_of::<InlineStr<SIZE, STR>>()
            && mem::align_of::<BorrowStr<BPAD, &'static STR>>()
                == mem::align_of::<InlineStr<SIZE, STR>>()
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
    pub const fn from_static(s: &'static STR) -> Self {
        if Self::IS_VALID_SIZE {
            Self {
                static_str: mem::ManuallyDrop::new(BorrowStr::from_static(s)),
            }
        } else {
            panic!("{}", BAD_SIZE_OR_ALIGNMENT);
        }
    }
}

impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP, STR>
    FlexStrBase<'str, SIZE, BPAD, HPAD, HEAP, STR>
where
    HEAP: Storage<STR>,
    STR: Str + ?Sized,
{
    /// Creates a new string from a `STR` reference. If the string is empty, an empty static string
    /// is returned. If at or under the inline length limit, an inline string will be returned.
    /// Otherwise, a heap based string will be allocated and returned. This is typically used to
    /// create strings from a non-static borrowed `STR` where you don't have ownership.
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
    pub fn from_ref(s: impl AsRef<STR>) -> Self
    where
        STR: AsRef<STR>,
    {
        let s = s.as_ref();

        match s.empty() {
            // TODO: Benchmark empty strings to see if I need to specialize this
            Some(empty) => Self::from_static(empty),
            None => match Self::try_inline(s) {
                Ok(s) => s,
                Err(_) => Self::from_ref_heap(s),
            },
        }
    }

    #[inline]
    fn from_inline(s: InlineStr<SIZE, STR>) -> Self {
        if Self::IS_VALID_SIZE {
            Self {
                inline_str: mem::ManuallyDrop::new(s),
            }
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
    pub fn try_inline<S: AsRef<STR>>(s: S) -> Result<Self, S> {
        match InlineStr::try_new(s) {
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
    pub fn from_ref_heap(s: impl AsRef<STR>) -> Self {
        if Self::IS_VALID_SIZE {
            Self {
                heap_str: mem::ManuallyDrop::new(HeapStr::from_ref(s)),
            }
        } else {
            panic!("{}", BAD_SIZE_OR_ALIGNMENT);
        }
    }

    /// Create a new heap based string by wrapping the existing user provided heap string type (T).
    /// For [LocalStr] this will be an [Rc\<str\>](std::rc::Rc) and for [SharedStr] it will be an
    /// [Arc\<str\>](std::sync::Arc). This would typically only be used if efficient unwrapping of heap
    /// based data is needed at a later time.
    /// ```
    /// use flexstr::LocalStr;
    ///
    /// let s = LocalStr::from_heap(b"test"[..].into());
    /// assert!(s.is_heap());
    /// ```
    #[inline]
    pub fn from_heap(t: HEAP) -> Self {
        if Self::IS_VALID_SIZE {
            Self {
                heap_str: mem::ManuallyDrop::new(HeapStr::from_heap(t)),
            }
        } else {
            panic!("{}", BAD_SIZE_OR_ALIGNMENT);
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

/// This serves as the base for borrowed (Ref) types. Only the borrow related methods are listed here.
///
/// A flexible string type that transparently wraps a string literal, inline string, a heap allocated type,
/// or a borrowed string (with appropriate lifetime).
///
/// # Note
/// It is not generally recommended to try and create direct custom concrete types of this type as it
/// is complicated to calculate the correct sizes of all the generic type parameters. However, be aware
/// that a runtime panic will be issued on creation if incorrect, so if you are able to create a string
/// of your custom type, your parameters were of correct size/alignment.
pub type FlexStrRefBase<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP, STR> =
    FlexStrBase<'str, SIZE, BPAD, HPAD, HEAP, STR>;

impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP, STR>
    FlexStrRefBase<'str, SIZE, BPAD, HPAD, HEAP, STR>
where
    HEAP: Storage<STR>,
    STR: Str + ?Sized,
{
    /// Creates a wrapped borrowed string literal. The string is not copied but the reference is
    /// simply wrapped and tied to the lifetime of the source string.
    /// ```
    /// use flexstr::LocalStrRef;
    ///
    /// let abc = format!("{}", "abc");
    /// let s = LocalStrRef::from_borrow(&abc);
    /// assert!(s.is_borrow());
    /// ```
    #[inline]
    pub fn from_borrow(s: &'str STR) -> Self {
        if Self::IS_VALID_SIZE {
            Self {
                borrow_str: mem::ManuallyDrop::new(BorrowStr::from_borrow(s)),
            }
        } else {
            panic!("{}", BAD_SIZE_OR_ALIGNMENT);
        }
    }

    /// Returns true if this is a wrapped string using borrowed storage
    #[inline]
    pub fn is_borrow(&self) -> bool {
        // SAFETY: Marker is identical in all union fields
        unsafe { matches!(self.static_str.marker, StorageType::Borrow) }
    }
}
