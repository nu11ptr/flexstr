use core::marker::PhantomData;
use core::{mem, ptr};

use crate::storage::StorageType;
use crate::string::Str;

/// Type representing the inline storage including its size and string type
type InlineStorage<const N: usize> = [mem::MaybeUninit<u8>; N];

#[doc(hidden)]
#[derive(Clone, Copy)]
#[cfg_attr(target_pointer_width = "64", repr(align(8)))]
#[cfg_attr(target_pointer_width = "32", repr(align(4)))]
#[repr(C)]
pub(crate) struct InlineStr<const SIZE: usize, STR>
where
    STR: ?Sized,
{
    data: InlineStorage<SIZE>,
    len: u8,
    pub marker: StorageType,
    phantom: PhantomData<STR>,
}

impl<const SIZE: usize, STR> InlineStr<SIZE, STR>
where
    STR: Str + ?Sized,
{
    /// Attempts to return a new [InlineStr] if the source string is short enough to be copied.
    /// If not, the source is returned as the error.
    #[inline(always)]
    pub fn try_new<T: AsRef<STR>>(s: T) -> Result<Self, T> {
        let s_ref = s.as_ref();

        // There is no guarantee this will be O(1) or constant
        let len = s_ref.length();

        if len <= Self::capacity() {
            unsafe { Ok(Self::new(s_ref, len)) }
        } else {
            Err(s)
        }
    }

    #[inline]
    unsafe fn new(s: &STR, len: usize) -> Self {
        // SAFETY: This is safe because while uninitialized to start, we copy the the str contents
        // over the top. We check to ensure it is not too long in `try_new` and don't call this
        // function directly. The copy is restrained to the length of the str.

        // Declare array, but keep uninitialized (we will overwrite momentarily)
        let mut data: [mem::MaybeUninit<u8>; SIZE] = mem::MaybeUninit::uninit().assume_init();
        // Copy contents of &str to our data buffer
        ptr::copy_nonoverlapping(s.as_inline_ptr(), data.as_mut_ptr().cast::<u8>(), len);

        Self {
            data,
            len: len as u8,
            marker: StorageType::Inline,
            phantom: PhantomData,
        }
    }

    /// Returns the capacity of this inline string
    // NOTE: Cannot be const due to the trait bounds on Str
    #[inline]
    pub fn capacity() -> usize {
        SIZE
    }
}
