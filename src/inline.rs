use core::marker::PhantomData;

pub struct InlineBytes<T: ?Sized> {
    inline: [u8; 22],
    len: u8,
    marker: PhantomData<T>,
}

impl<T: ?Sized> Clone for InlineBytes<T> {
    fn clone(&self) -> Self {
        Self {
            inline: self.inline,
            len: self.len,
            marker: self.marker,
        }
    }
}

impl<T: ?Sized> AsRef<[u8]> for InlineBytes<T> {
    fn as_ref(&self) -> &[u8] {
        &self.inline[..self.len as usize]
    }
}
