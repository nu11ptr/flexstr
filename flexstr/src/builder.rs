use alloc::string::String;
use core::fmt::Write;
use core::ops::Deref;
use core::{fmt, mem, ptr, str};

use crate::inline::STRING_SIZED_INLINE;

// The size of internal buffer for formatting (if larger needed we punt and just use a heap allocated String)
#[doc(hidden)]
pub const BUFFER_SIZE: usize = 1024;

// *** String Buffer ***

// This is used so that if the writes are small enough we can move 'inline' directly out without a copy
#[doc(hidden)]
#[cfg_attr(target_pointer_width = "64", repr(align(8)))]
#[cfg_attr(target_pointer_width = "32", repr(align(4)))]
pub union Buffer<const N: usize = STRING_SIZED_INLINE, const N2: usize = BUFFER_SIZE> {
    inline: [mem::MaybeUninit<u8>; N],
    pub heap: [mem::MaybeUninit<u8>; N2],
}

// NOTE: This is a macro as the inliner was copying our stack buffer
#[doc(hidden)]
#[macro_export]
macro_rules! buffer_new {
    ($inline_size:expr) => {
        unsafe {
            const BUF_SIZE: usize = $crate::builder::BUFFER_SIZE;

            // SAFETY: Just an uninitialized buffer - we will ensure not to read past `len` below
            // Since N2 > N we will always treat this as the 'heap' variant except when extracting
            $crate::builder::StringBuffer::<$inline_size, BUF_SIZE> {
                len: 0,
                buffer: $crate::builder::Buffer {
                    heap: ::core::mem::MaybeUninit::uninit().assume_init(),
                },
            }
        }
    };
}

// Used to buffer formatting writes before turning into inline string or ref counter string
#[doc(hidden)]
pub struct StringBuffer<const N: usize = STRING_SIZED_INLINE, const N2: usize = BUFFER_SIZE> {
    pub buffer: Buffer<N, N2>,
    pub len: usize,
}

impl<const N: usize, const N2: usize> StringBuffer<N, N2> {
    #[inline]
    pub fn capacity() -> usize {
        N2
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
    pub fn is_inline_candidate(&self) -> bool {
        self.len <= N
    }

    #[inline]
    pub fn into_inner(self) -> [mem::MaybeUninit<u8>; N] {
        // SAFETY: 'inline' is always smaller (N) than 'heap' (N2) and we only return it when we
        // are sure the len <= N
        unsafe { self.buffer.inline }
    }

    /// Create a new heap allocated string buffer copying the existing content
    #[inline]
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

        if (Self::capacity() - len) >= s.len() {
            unsafe {
                // SAFETY: We do everything based on 'heap' (N2) which is always larger than 'inline' (N)
                // and is also interchangeable with inline (inline is just shorter)
                let buffer = &mut self.buffer.heap[len..];

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

impl<const N: usize, const N2: usize> Deref for StringBuffer<N, N2> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe {
            // SAFETY: We do everything based on 'heap' (N2) which is always larger than 'inline' (N)
            // and is also interchangeable with inline (inline is just shorter)
            let buffer = &self.buffer.heap[..self.len()];

            // SAFETY: The contents are always obtained from a valid UTF8 str, so they must be valid
            // Additionally, we clamp the size of the slice passed to be no longer than our str length
            let buffer = &*(buffer as *const [mem::MaybeUninit<u8>] as *const [u8]);
            str::from_utf8_unchecked(buffer)
        }
    }
}

// *** LocalStr Builder ***

#[doc(hidden)]
pub enum FlexStrBuilder<
    'buffer,
    const N: usize = STRING_SIZED_INLINE,
    const N2: usize = BUFFER_SIZE,
> {
    InlineBuffer(&'buffer mut StringBuffer<N, N2>),
    StringBuffer(String),
}

impl<const N: usize, const N2: usize> FlexStrBuilder<'_, N, N2> {
    #[inline]
    pub fn str_write(&mut self, s: impl AsRef<str>) {
        // SAFETY: This always succeeds - buffer will be promoted until it eventually becomes a
        // `String` which cannot fail per stdlib docs
        unsafe {
            <Self as Write>::write_str(self, s.as_ref()).unwrap_unchecked();
        }
    }

    #[inline]
    pub fn char_write(&mut self, c: char) {
        // SAFETY: Wraps `write_str` which always succeeds per above
        unsafe { <Self as Write>::write_char(self, c).unwrap_unchecked() }
    }
}

impl<const N: usize, const N2: usize> Write for FlexStrBuilder<'_, N, N2> {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        match self {
            FlexStrBuilder::InlineBuffer(buffer) => {
                if buffer.write(s) {
                    Ok(())
                } else {
                    let required_cap = buffer.len() + s.len();
                    // Start with a capacity twice the size of what is needed (to try and avoid future heap allocations)
                    let mut buffer = buffer.to_string_buffer(required_cap * 2);
                    // SAFETY: This always succeeds for String per stdlib
                    unsafe {
                        buffer.write_str(s).unwrap_unchecked();
                    }

                    *self = FlexStrBuilder::StringBuffer(buffer);
                    Ok(())
                }
            }
            // This always succeeds per stdlib docs
            FlexStrBuilder::StringBuffer(buffer) => buffer.write_str(s),
        }
    }
}

#[cfg(feature = "fast_format")]
impl<const N: usize, const N2: usize> ufmt_write::uWrite for FlexStrBuilder<'_, N, N2> {
    type Error = core::fmt::Error;

    #[inline]
    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        <Self as Write>::write_str(self, s)
    }
}

// NOTE: This is a macro as the inliner was copying our stack buffer
#[doc(hidden)]
#[macro_export]
macro_rules! builder_new {
    ($buffer:ident) => {
        $crate::builder::FlexStrBuilder::InlineBuffer(&mut $buffer)
    };
    ($buffer:ident, $cap:expr) => {
        if $cap <= $crate::builder::BUFFER_SIZE {
            $crate::builder::FlexStrBuilder::InlineBuffer(&mut $buffer)
        } else {
            $crate::builder::FlexStrBuilder::StringBuffer(String::with_capacity($cap))
        }
    };
}

// NOTE: This is a macro as the inliner was copying our stack buffer
#[doc(hidden)]
#[macro_export]
macro_rules! builder_into {
    ($builder:ident, $buffer: ident) => {
        match $builder {
            $crate::builder::FlexStrBuilder::InlineBuffer(_) => {
                if $buffer.is_inline_candidate() {
                    let len = $buffer.len() as u8;
                    $crate::FlexStrWrapper {
                        inline_str: $crate::inline::InlineFlexStr::from_array(
                            $buffer.into_inner(),
                            len,
                        ),
                    }
                } else {
                    $crate::traits::ToFlex::to_flex(&*$buffer)
                }
            }
            $crate::builder::FlexStrBuilder::StringBuffer(s) => s.into(),
        }
    };
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;
    use core::fmt::Write;

    use crate::builder::{FlexStrBuilder, BUFFER_SIZE};
    use crate::inline::STRING_SIZED_INLINE;
    use crate::LocalStr;

    #[test]
    fn string_buffer() {
        // Write 1
        let write1 = "test";
        let mut buffer = buffer_new!(STRING_SIZED_INLINE);
        assert!(buffer.is_empty());
        assert!(buffer.write(write1));
        assert_eq!(buffer.len(), write1.len());
        assert_eq!(&*buffer, write1);

        // Try write 2 - not large enough
        let write2 = "x".repeat(BUFFER_SIZE);
        assert!(!buffer.write(&write2));
        assert_eq!(buffer.len(), write1.len());
        assert_eq!(&*buffer, write1.to_string());

        // Promote to string buffer and ensure contents are copied
        let mut buffer = buffer.to_string_buffer(write1.len() + write2.len());
        assert_eq!(buffer.len(), write1.len());
        assert_eq!(&*buffer, write1.to_string());

        // Retry write 2
        assert!(buffer.write_str(&write2).is_ok());
        assert_eq!(buffer.len(), write1.len() + write2.len());
        assert_eq!(&*buffer, write1.to_string() + &write2);
    }

    #[test]
    fn flex_str_builder_promotion() {
        // Write 1 - verify inline buffer size
        let mut buffer = buffer_new!(STRING_SIZED_INLINE);
        let mut builder = builder_new!(buffer);
        assert!(matches!(builder, FlexStrBuilder::InlineBuffer(_)));
        let write = "This should fit easily in our buffer";
        assert!(builder.write_str(write).is_ok());
        assert!(matches!(builder, FlexStrBuilder::InlineBuffer(_)));

        // Write 2
        let write2 = "x".repeat(BUFFER_SIZE);
        assert!(builder.write_str(&write2).is_ok());
        assert!(matches!(builder, FlexStrBuilder::StringBuffer(_)));
        let s: LocalStr = builder_into!(builder, buffer);
        assert_eq!(s, write.to_string() + &write2);
    }
}
