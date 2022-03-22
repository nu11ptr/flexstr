use alloc::rc::Rc;
use alloc::sync::Arc;
use core::ops::Deref;

use crate::STRING_SIZED_INLINE;

pub trait Writer {
    fn str_write(&mut self, s: impl AsRef<str>);

    fn char_write(&mut self, ch: char);
}

// pub struct FakeMaybeUninit;
//
// impl Writer for FakeMaybeUninit {
//     fn str_write(&mut self, _: &str) {
//         unimplemented!("'write_str' called on 'FakeMaybeUninit'");
//     }
//
//     fn char_write(&mut self, _: char) {
//         unimplemented!("'write_char' called on 'FakeMaybeUninit'");
//     }
// }

pub trait ExactSizedCreate: Clone + Deref<Target = str> + for<'a> From<&'a str> {
    // #[inline]
    // fn try_new_uninit(_len: usize) -> Result<T, ()> {
    //     Err(())
    // }
    //
    // #[inline]
    // unsafe fn assume_init(_uninit: T) -> Self {
    //     unimplemented!("This shouldn't be called unless overridden");
    // }

    #[inline]
    fn create_exact_sized<F, S, W>(s: S, capacity: usize, f: F) -> Self
    where
        F: Fn(S, &W),
        S: AsRef<str>,
        W: Writer,
    {
        // TODO: Find a way to inherit SIZE instead of using this
        let mut buffer = buffer_new!(STRING_SIZED_INLINE);
        let mut builder = builder_new!(buffer, capacity);

        f(s, &builder);
        Self::from(&*buffer)
    }
}

impl ExactSizedCreate for Rc<str> {}

impl ExactSizedCreate for Arc<str> {}
