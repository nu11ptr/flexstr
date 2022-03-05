use alloc::string::String;
use core::fmt::Write;
use core::ops::Deref;
use core::{fmt, mem, ptr, str};

use crate::inline::MAX_INLINE;

// The size of internal buffer for formatting (if larger needed we punt and just use a heap allocated String)
pub(crate) const BUFFER_SIZE: usize = 1024;

// *** String Buffer ***

// Used to buffer formatting writes before turning into inline string or ref counter string
pub(crate) struct StringBuffer<const N: usize> {
    buffer: [mem::MaybeUninit<u8>; N],
    len: usize,
}

impl<const N: usize> StringBuffer<N> {
    pub fn new() -> Self {
        unsafe {
            // SAFETY: This should all be ok, because we will never read more then `len` which is
            // never larger than what has been written

            Self {
                buffer: mem::MaybeUninit::uninit().assume_init(),
                len: 0,
            }
        }
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        N
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn into_inner(self) -> [mem::MaybeUninit<u8>; N] {
        self.buffer
    }

    /// Create a new large string buffer copying the existing content
    pub fn to_large_buffer<const N2: usize>(&self) -> StringBuffer<N2> {
        let mut buffer = StringBuffer::new();

        if !self.is_empty() {
            unsafe {
                // SAFETY: This should be ok because we only copy what we've already written into
                // a brand new buffer. No way for it to overlap.
                // *** WE DO NEED TO BE CAREFUL TO ENSURE N2 >= self.len ALWAYS ***

                // Copy contents of &str to our data buffer
                ptr::copy_nonoverlapping(
                    self.buffer.as_ptr(),
                    buffer.buffer.as_mut_ptr(),
                    self.len(),
                );
            }

            buffer.len = self.len;
        }

        buffer
    }

    /// Create a new heap allocated string buffer copying the existing content
    pub fn to_string_buffer(&self, cap: usize) -> String {
        let mut buffer = String::with_capacity(cap);

        if !self.is_empty() {
            buffer.push_str(self);
        }

        buffer
    }

    /// Write the formatting `&str` into the buffer if possible. Returns true if write successful.
    pub fn write(&mut self, s: &str) -> bool {
        let len = self.len();

        if (self.capacity() - len) >= s.len() {
            let buffer = &mut self.buffer[len..];

            unsafe {
                // SAFETY: we've ensured enough space, moved up position, and no way s can overlap
                ptr::copy_nonoverlapping(s.as_ptr(), buffer.as_mut_ptr().cast(), s.len());
            }
            self.len += s.len();
            true
        } else {
            false
        }
    }
}

impl<const N: usize> Deref for StringBuffer<N> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        let buffer = &self.buffer[..self.len()];

        unsafe {
            // SAFETY: The contents are always obtained from a valid UTF8 str, so they must be valid
            // Additionally, we clamp the size of the slice passed to be no longer than our str length
            let buffer = &*(buffer as *const [mem::MaybeUninit<u8>] as *const [u8]);
            str::from_utf8_unchecked(buffer)
        }
    }
}

// *** FlexStr Builder ***

#[allow(clippy::large_enum_variant)]
pub(crate) enum FlexStrBuilder {
    Small(StringBuffer<MAX_INLINE>),
    Regular(StringBuffer<BUFFER_SIZE>),
    Large(String),
}

impl FlexStrBuilder {
    #[inline]
    pub fn new() -> Self {
        // TODO: Is it worth assuming inline size if we don't know the capacity needed???
        FlexStrBuilder::Small(StringBuffer::new())
    }

    #[inline]
    pub fn with_capacity(cap: usize) -> Self {
        if cap <= MAX_INLINE {
            FlexStrBuilder::Small(StringBuffer::new())
        } else if cap <= BUFFER_SIZE {
            FlexStrBuilder::Regular(StringBuffer::new())
        } else {
            FlexStrBuilder::Large(String::with_capacity(cap))
        }
    }

    fn create_string_and_write<const N: usize>(
        buffer: &mut StringBuffer<N>,
        s: &str,
    ) -> FlexStrBuilder {
        let required_cap = buffer.len() + s.len();
        // Start with a capacity twice the size of what is needed (to try and avoid future heap allocations)
        let mut buffer = buffer.to_string_buffer(required_cap * 2);
        // SAFETY: This always succeeds for String per stdlib
        unsafe {
            buffer.write_str(s).unwrap_unchecked();
        }
        FlexStrBuilder::Large(buffer)
    }

    #[inline]
    pub fn str_write(&mut self, s: &str) {
        // SAFETY: This always succeeds - buffer will be promoted until it eventually becomes a
        // `String` which cannot fail per stdlib docs
        unsafe {
            self.write_str(s).unwrap_unchecked();
        }
    }

    #[inline]
    pub fn char_write(&mut self, c: char) {
        // SAFETY: Wraps `write_str` which always succeeds per above
        unsafe { self.write_char(c).unwrap_unchecked() }
    }
}

impl Write for FlexStrBuilder {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        match self {
            FlexStrBuilder::Small(buffer) => {
                if buffer.write(s) {
                    Ok(())
                } else if s.len() <= BUFFER_SIZE {
                    let mut buffer = buffer.to_large_buffer();
                    buffer.write(s);
                    *self = FlexStrBuilder::Regular(buffer);
                    Ok(())
                } else {
                    *self = Self::create_string_and_write(buffer, s);
                    Ok(())
                }
            }
            FlexStrBuilder::Regular(buffer) => {
                if buffer.write(s) {
                    Ok(())
                } else {
                    *self = Self::create_string_and_write(buffer, s);
                    Ok(())
                }
            }
            // This always succeeds per stdlib docs
            FlexStrBuilder::Large(buffer) => buffer.write_str(s),
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;
    use core::fmt::Write;

    use crate::builder::{FlexStrBuilder, StringBuffer, BUFFER_SIZE};
    use crate::inline::MAX_INLINE;
    use crate::FlexStr;

    #[test]
    fn string_buffer() {
        // Write 1
        let write1 = "test";
        let mut buffer: StringBuffer<MAX_INLINE> = StringBuffer::new();
        assert!(buffer.is_empty());
        assert!(buffer.write(write1));
        assert_eq!(buffer.len(), write1.len());
        assert_eq!(&*buffer, write1);

        // Try write 2 - not large enough
        let write2 = "This is far too long for the inline buffer!!!!!!!!!";
        assert!(!buffer.write(write2));
        assert_eq!(buffer.len(), write1.len());
        assert_eq!(&*buffer, write1);

        // Promote to larger buffer and ensure contents copied
        let mut buffer: StringBuffer<BUFFER_SIZE> = buffer.to_large_buffer();
        assert_eq!(buffer.len(), write1.len());
        assert_eq!(&*buffer, write1);

        // Retry write 2
        assert!(buffer.write(write2));
        assert_eq!(buffer.len(), write1.len() + write2.len());
        assert_eq!(&*buffer, write1.to_string() + write2);

        // Try write 3 - not large enough
        let write3 = "x".repeat(BUFFER_SIZE);
        assert!(!buffer.write(&write3));
        assert_eq!(buffer.len(), write1.len() + write2.len());
        assert_eq!(&*buffer, write1.to_string() + write2);

        // Promote to string buffer and ensure contents are copied
        let mut buffer = buffer.to_string_buffer(write1.len() + write2.len() + write3.len());
        assert_eq!(buffer.len(), write1.len() + write2.len());
        assert_eq!(&*buffer, write1.to_string() + write2);

        // Retry write 3
        assert!(buffer.write_str(&write3).is_ok());
        assert_eq!(buffer.len(), write1.len() + write2.len() + write3.len());
        assert_eq!(&*buffer, write1.to_string() + write2 + &write3);
    }

    #[test]
    fn flex_str_builder_promotion() {
        // Write 1 - verify inline buffer size
        let write1 = "test";
        let mut builder = FlexStrBuilder::new();
        assert!(matches!(builder, FlexStrBuilder::Small(_)));
        assert!(builder.write_str(write1).is_ok());
        assert!(matches!(builder, FlexStrBuilder::Small(_)));

        // Write 2
        let write2 = "This is far too long for the inline buffer!!!!!!!!!";
        assert!(builder.write_str(write2).is_ok());
        assert!(matches!(builder, FlexStrBuilder::Regular(_)));

        // Write 3
        let write3 = "x".repeat(BUFFER_SIZE);
        assert!(builder.write_str(&write3).is_ok());
        assert!(matches!(builder, FlexStrBuilder::Large(_)));
        let s: FlexStr = builder.into();
        assert_eq!(s, write1.to_string() + write2 + &write3);
    }
}
