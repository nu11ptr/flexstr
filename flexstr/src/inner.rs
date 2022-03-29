use core::mem;

use crate::{BorrowStr, HeapStr, InlineStr, Storage, StorageType, Str, BAD_SIZE_OR_ALIGNMENT};

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
pub union FlexStrInner<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP, STR>
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
    for FlexStrInner<'str, SIZE, PAD1, PAD2, HEAP, STR>
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
    for FlexStrInner<'str, SIZE, PAD1, PAD2, HEAP, STR>
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
    FlexStrInner<'str, SIZE, BPAD, HPAD, HEAP, STR>
where
    STR: ?Sized + 'static,
{
    // If the union variants aren't the precise right size bad things will happen - we protect against that
    pub(crate) const IS_VALID_SIZE: bool = Self::variant_sizes_are_valid();

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
    FlexStrInner<'str, SIZE, BPAD, HPAD, HEAP, STR>
where
    HEAP: Storage<STR>,
    STR: Str + ?Sized,
{
    #[inline]
    pub fn from_ref(s: impl AsRef<STR>) -> Self {
        match s.as_ref().empty() {
            // TODO: Benchmark empty strings to see if I need to specialize this
            Some(empty) => Self::from_static(empty),
            None => match Self::try_inline(s) {
                Ok(s) => s,
                Err(s) => Self::from_ref_heap(s),
            },
        }
    }

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

    #[inline]
    pub fn try_inline<S: AsRef<STR>>(s: S) -> Result<Self, S> {
        match InlineStr::try_new(s) {
            Ok(s) => Ok(Self::from_inline(s)),
            Err(s) => Err(s),
        }
    }

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

    #[inline]
    pub fn is_static(&self) -> bool {
        // SAFETY: Marker is identical in all union fields
        unsafe { matches!(self.static_str.marker, StorageType::Static) }
    }

    #[inline]
    pub fn is_inline(&self) -> bool {
        // SAFETY: Marker is identical in all union fields
        unsafe { matches!(self.static_str.marker, StorageType::Inline) }
    }

    #[inline]
    pub fn is_heap(&self) -> bool {
        // SAFETY: Marker is identical in all union fields
        unsafe { matches!(self.static_str.marker, StorageType::Heap) }
    }

    #[inline]
    pub fn is_borrow(&self) -> bool {
        // SAFETY: Marker is identical in all union fields
        unsafe { matches!(self.static_str.marker, StorageType::Borrow) }
    }
}
