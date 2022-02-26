use alloc::string::{String, ToString};
use core::fmt::{Debug, Display, Formatter};
use core::ops::Deref;
use core::{fmt, mem, ptr, str};

/// The max capacity of an inline string (in bytes)
pub const MAX_INLINE: usize = mem::size_of::<String>() - 2;

/// This is the custom inline string type - it is not typically used directly, but instead is used
/// transparently by `FlexStr` and `AFlexStr`
#[derive(Clone, Copy, Debug)]
pub struct InlineFlexStr {
    data: [mem::MaybeUninit<u8>; MAX_INLINE],
    len: u8,
}

impl InlineFlexStr {
    /// Attempts to return a new `InlineFlexStr` if the source string is short enough to be copied.
    /// If not, the source is returned as the error.
    #[inline]
    pub fn try_new<T: AsRef<str>>(s: T) -> Result<Self, T> {
        let s_ref = s.as_ref();

        if s_ref.len() > MAX_INLINE {
            Err(s)
        } else {
            unsafe { Ok(Self::new(s_ref)) }
        }
    }

    unsafe fn new(s: &str) -> Self {
        // Safety: This is safe because while uninitialized to start, we copy the the str contents
        // over the top. We check to ensure it is not too long in `try_new` and don't call this
        // function directly. The copy is restrained to the length of the str.

        // Declare array, but keep uninitialized (we will overwrite momentarily)
        let mut data: [mem::MaybeUninit<u8>; MAX_INLINE] = mem::MaybeUninit::uninit().assume_init();
        // Copy contents of &str to our data buffer
        ptr::copy_nonoverlapping(s.as_ptr(), data.as_mut_ptr().cast::<u8>(), s.len());

        Self {
            len: s.len() as u8,
            data,
        }
    }

    /// Returns the length of this `InlineFlexStr` in bytes
    #[inline]
    pub fn len(&self) -> usize {
        self.len as usize
    }

    /// Returns true if this `InlineFlexStr` is an empty string
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn try_concat(&mut self, s: &str) -> bool {
        if self.len() + s.len() > MAX_INLINE {
            false
        } else {
            // Point to the location directly after our string
            let data = self.data[self.len as usize..].as_mut_ptr().cast::<u8>();

            unsafe {
                // Safety: We know the buffer is large enough and that the location is not overlapping
                // this one (we know that because we have ownership of one of them)
                // Copy contents of &str to our data buffer
                ptr::copy_nonoverlapping(s.as_ptr(), data, s.len());
            }
            self.len += s.len() as u8;
            true
        }
    }
}

impl Display for InlineFlexStr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.deref(), f)
    }
}

impl Deref for InlineFlexStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        let data = &self.data[..self.len()];

        unsafe {
            // Safety: The contents are always obtained from a valid UTF8 str, so they must be valid
            // Additionally, we clamp the size of the slice passed to be no longer than our str length
            let data = &*(data as *const [mem::MaybeUninit<u8>] as *const [u8]);
            str::from_utf8_unchecked(data)
        }
    }
}

impl TryFrom<String> for InlineFlexStr {
    type Error = String;

    #[inline]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl<'s> TryFrom<&'s String> for InlineFlexStr {
    type Error = &'s String;

    #[inline]
    fn try_from(value: &'s String) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl<'s> TryFrom<&'s str> for InlineFlexStr {
    type Error = &'s str;

    #[inline]
    fn try_from(value: &'s str) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl From<InlineFlexStr> for String {
    #[inline]
    fn from(s: InlineFlexStr) -> Self {
        s.to_string()
    }
}
