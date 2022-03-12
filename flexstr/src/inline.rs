use alloc::string::String;
use core::fmt::{Debug, Formatter};
use core::ops::Deref;
use core::{fmt, mem, ptr, str};

/// Using this inline capacity will result in a type with the same memory size as a builtin `String`
pub const STRING_SIZED_INLINE: usize = mem::size_of::<String>() - 2;

/// This is the custom inline string type - it is not typically used directly, but instead is used
/// transparently by `FlexStr` and `AFlexStr`
#[derive(Clone, Copy)]
pub(crate) struct InlineFlexStr<const N: usize = STRING_SIZED_INLINE> {
    data: [mem::MaybeUninit<u8>; N],
    len: u8,
}

impl<const N: usize> InlineFlexStr<N> {
    /// Attempts to return a new `InlineFlexStr` if the source string is short enough to be copied.
    /// If not, the source is returned as the error.
    #[inline]
    pub fn try_new<T: AsRef<str>>(s: T) -> Result<Self, T> {
        let s_ref = s.as_ref();

        if s_ref.len() <= Self::capacity() {
            unsafe { Ok(Self::new(s_ref)) }
        } else {
            Err(s)
        }
    }

    #[inline]
    unsafe fn new(s: &str) -> Self {
        // SAFETY: This is safe because while uninitialized to start, we copy the the str contents
        // over the top. We check to ensure it is not too long in `try_new` and don't call this
        // function directly. The copy is restrained to the length of the str.

        // Declare array, but keep uninitialized (we will overwrite momentarily)
        let mut data: [mem::MaybeUninit<u8>; N] = mem::MaybeUninit::uninit().assume_init();
        // Copy contents of &str to our data buffer
        ptr::copy_nonoverlapping(s.as_ptr(), data.as_mut_ptr().cast::<u8>(), s.len());

        Self {
            len: s.len() as u8,
            data,
        }
    }

    #[inline]
    pub(crate) fn from_array(data: [mem::MaybeUninit<u8>; N], len: u8) -> Self {
        Self { data, len }
    }

    /// Returns the capacity of this inline string
    #[inline]
    pub fn capacity() -> usize {
        N
    }

    /// Returns the length of this `InlineFlexStr` in bytes
    #[inline]
    pub fn len(&self) -> usize {
        self.len as usize
    }

    /// Return true if the inline string is empty else false
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Attempts to concatenate the `&str` if there is room. It returns true if it is able to do so.
    #[inline]
    pub fn try_concat(&mut self, s: &str) -> bool {
        if self.len() + s.len() <= Self::capacity() {
            // Point to the location directly after our string
            let data = self.data[self.len as usize..].as_mut_ptr().cast::<u8>();

            unsafe {
                // SAFETY: We know the buffer is large enough and that the location is not overlapping
                // this one (we know that because we have ownership of one of them)
                // Copy contents of &str to our data buffer
                ptr::copy_nonoverlapping(s.as_ptr(), data, s.len());
            }
            self.len += s.len() as u8;
            true
        } else {
            false
        }
    }
}

impl<const N: usize> Debug for InlineFlexStr<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <str as Debug>::fmt(self, f)
    }
}

impl<const N: usize> Deref for InlineFlexStr<N> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        let data = &self.data[..self.len()];

        unsafe {
            // SAFETY: The contents are always obtained from a valid UTF8 str, so they must be valid
            // Additionally, we clamp the size of the slice passed to be no longer than our str length
            let data = &*(data as *const [mem::MaybeUninit<u8>] as *const [u8]);
            str::from_utf8_unchecked(data)
        }
    }
}

impl<const N: usize> TryFrom<String> for InlineFlexStr<N> {
    type Error = String;

    #[inline]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl<'s, const N: usize> TryFrom<&'s String> for InlineFlexStr<N> {
    type Error = &'s String;

    #[inline]
    fn try_from(value: &'s String) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl<'s, const N: usize> TryFrom<&'s str> for InlineFlexStr<N> {
    type Error = &'s str;

    #[inline]
    fn try_from(value: &'s str) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::inline::InlineFlexStr;
    use alloc::string::ToString;

    #[test]
    fn empty() {
        let lit = "";
        let s: InlineFlexStr = lit.try_into().expect("bad inline str");
        assert_eq!(&*s, lit);
        assert_eq!(s.len(), lit.len())
    }

    #[test]
    fn good_init() {
        let lit = "inline";
        let s: InlineFlexStr = lit.try_into().expect("bad inline str");
        assert_eq!(&*s, lit);
        assert_eq!(s.len(), lit.len())
    }

    #[test]
    fn bad_init() {
        let lit = "This is way too long to be an inline string!!!";
        let s = <InlineFlexStr>::try_new(lit).unwrap_err();
        assert_eq!(s, lit);
        assert_eq!(s.len(), lit.len())
    }

    #[test]
    fn good_concat() {
        let lit = "Inline";
        let lit2 = " me";
        let mut s = <InlineFlexStr>::try_new(lit).expect("bad inline str");
        assert!(s.try_concat(lit2));
        assert_eq!(&*s, lit.to_string() + lit2);
    }

    #[test]
    fn bad_concat() {
        let lit = "This is";
        let lit2 = " way too long to be an inline string!!!";
        let mut s = <InlineFlexStr>::try_new(lit).expect("bad inline str");
        assert!(!s.try_concat(lit2));
        assert_eq!(&*s, lit);
    }
}
