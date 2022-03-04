use alloc::string::String;
use core::fmt::{Debug, Formatter};
use core::ops::Deref;
use core::{fmt, mem, ptr, str};

/// The max capacity of an inline string (in bytes)
pub(crate) const MAX_INLINE: usize = mem::size_of::<String>() - 2;

/// This is the custom inline string type - it is not typically used directly, but instead is used
/// transparently by `FlexStr` and `AFlexStr`
#[derive(Clone, Copy)]
pub(crate) struct InlineFlexStr {
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

    #[inline]
    unsafe fn new(s: &str) -> Self {
        // SAFETY: This is safe because while uninitialized to start, we copy the the str contents
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

    #[inline]
    pub(crate) fn from_array(data: [mem::MaybeUninit<u8>; MAX_INLINE], len: u8) -> Self {
        Self { data, len }
    }

    /// Returns the length of this `InlineFlexStr` in bytes
    #[inline]
    pub fn len(&self) -> usize {
        self.len as usize
    }

    /// Attempts to concatenate the `&str` if there is room. It returns true if it is able to do so.
    pub fn try_concat(&mut self, s: &str) -> bool {
        if self.len() + s.len() > MAX_INLINE {
            false
        } else {
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
        }
    }
}

impl Debug for InlineFlexStr {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <str as Debug>::fmt(self, f)
    }
}

impl Deref for InlineFlexStr {
    type Target = str;

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
        let s = InlineFlexStr::try_new(lit).unwrap_err();
        assert_eq!(s, lit);
        assert_eq!(s.len(), lit.len())
    }

    #[test]
    fn good_concat() {
        let lit = "Inline";
        let lit2 = " me";
        let mut s = InlineFlexStr::try_new(lit).expect("bad inline str");
        assert!(s.try_concat(lit2));
        assert_eq!(&*s, lit.to_string() + lit2);
    }

    #[test]
    fn bad_concat() {
        let lit = "This is";
        let lit2 = " way too long to be an inline string!!!";
        let mut s = InlineFlexStr::try_new(lit).expect("bad inline str");
        assert!(!s.try_concat(lit2));
        assert_eq!(&*s, lit);
    }
}
