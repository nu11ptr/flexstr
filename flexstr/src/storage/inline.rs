use core::marker::PhantomData;
use core::{mem, ptr};

use crate::storage::StorageType;
use crate::string::Str;

#[doc(hidden)]
#[cfg_attr(target_pointer_width = "64", repr(align(8)))]
#[cfg_attr(target_pointer_width = "32", repr(align(4)))]
#[repr(C)]
pub(crate) struct InlineStr<const SIZE: usize, STR>
where
    STR: ?Sized,
{
    data: [mem::MaybeUninit<u8>; SIZE],
    len: u8,
    pub marker: StorageType,
    // TODO: Do research on phantom type as relates to variance and auto traits
    phantom: PhantomData<fn(STR) -> STR>,
}

impl<const SIZE: usize, STR> Copy for InlineStr<SIZE, STR> where STR: ?Sized {}

impl<const SIZE: usize, STR> Clone for InlineStr<SIZE, STR>
where
    STR: ?Sized,
{
    fn clone(&self) -> Self {
        panic!("Clone should never be used on `InlineStr`");
    }
}

impl<const SIZE: usize, STR> InlineStr<SIZE, STR>
where
    STR: ?Sized,
{
    // If the SIZE param is larger than a u8
    const IS_VALID_SIZE: bool = Self::variant_size_is_valid();

    #[inline]
    const fn variant_size_is_valid() -> bool {
        mem::size_of::<InlineStr<SIZE, STR>>()
            <= (u8::MAX as usize) + mem::size_of::<StorageType>() + 1
    }
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

    #[inline(always)]
    unsafe fn new(s: &STR, len: usize) -> Self {
        if Self::IS_VALID_SIZE {
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
        } else {
            panic!("Oops! The max inline size cannot exceed 255 bytes");
        }
    }

    /// Returns the capacity of this inline string
    // NOTE: Cannot be const due to the trait bounds on Str
    #[inline(always)]
    pub fn capacity() -> usize {
        SIZE
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len as usize
    }

    #[inline]
    pub fn as_str_type(&self) -> &STR {
        let data = &self.data[..self.len()];

        unsafe {
            // SAFETY: The contents are always obtained from a valid UTF8 str, so they must be valid
            // Additionally, we clamp the size of the slice passed to be no longer than our str length
            let data = &*(data as *const [mem::MaybeUninit<u8>] as *const [u8]);
            STR::from_inline_data(data)
        }
    }
}
