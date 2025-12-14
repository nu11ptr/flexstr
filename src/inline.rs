#[cfg(not(feature = "std"))]
use alloc::string::String;
use core::ops::Deref;

// This must be the size of the String type minus 2 bytes for the length and discriminator
pub(crate) const INLINE_CAPACITY: usize = size_of::<String>() - 2;

pub struct InlineBytes {
    inline: [u8; INLINE_CAPACITY],
    len: u8,
}

impl InlineBytes {
    pub(crate) fn from_bytes(s: &[u8]) -> Self {
        let mut inline = [0u8; INLINE_CAPACITY];
        let len = s.len();

        // PANIC SAFETY: Caller responsible for ensuring the slice is not too long
        inline[..len].copy_from_slice(&s[..len]);

        Self {
            inline,
            len: len as u8,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.inline[..self.len as usize]
    }
}

// TODO: We can probably derive this, but that might change if we add a MaybeUninit<u8> as the unsafe option
impl Clone for InlineBytes {
    fn clone(&self) -> Self {
        Self {
            inline: self.inline,
            len: self.len,
        }
    }
}

impl AsRef<[u8]> for InlineBytes {
    #[inline(always)]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Deref for InlineBytes {
    type Target = [u8];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_bytes()
    }
}
