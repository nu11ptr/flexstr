use core::marker::PhantomData;

use flexstr_support::StringToFromBytes;

// *** OwnedToFromBoxedInner ***

pub trait OwnedToFromBoxed<S: ?Sized + StringToFromBytes>
where
    S::Owned: OwnedToFromBoxed<S>,
{
    type BoxType;

    fn into_boxed(self) -> Self::BoxType;

    fn from_boxed(boxed: &mut Self::BoxType) -> Self;

    fn clone_boxed(boxed: &Self::BoxType) -> Self::BoxType;
}

// *** OwnedOpsMut ***

pub trait OwnedOpsMut<S: ?Sized + StringToFromBytes> {
    fn push_str(&mut self, s: &S);
}

// *** BoxedFlexStr ***

#[cfg(feature = "safe")]
pub type BoxedFlexStr<S> = Boxed<S, Option<alloc::boxed::Box<S>>>;

#[cfg(not(feature = "safe"))]
pub type BoxedFlexStr<S> = Boxed<S, crate::small_box::SmallBox<S>>;

pub struct Boxed<S: ?Sized + StringToFromBytes, B>
where
    S::Owned: OwnedToFromBoxed<S, BoxType = B>,
{
    inner: B,
    _marker: PhantomData<S>,
}

impl<S: ?Sized + StringToFromBytes, B> Boxed<S, B>
where
    S::Owned: OwnedToFromBoxed<S, BoxType = B>,
{
    pub fn new(s: S::Owned) -> Self {
        Self {
            inner: s.into_boxed(),
            _marker: PhantomData,
        }
    }
}

impl<S: ?Sized + StringToFromBytes, B> Boxed<S, B>
where
    S::Owned: OwnedToFromBoxed<S, BoxType = B> + OwnedOpsMut<S>,
{
    pub fn with_mut_str<F>(&mut self, f: F)
    where
        F: FnOnce(&mut S::Owned),
    {
        let mut str: S::Owned = S::Owned::from_boxed(&mut self.inner);
        f(&mut str);
        self.inner = str.into_boxed();
    }

    pub fn push_str(&mut self, s: &S) {
        self.with_mut_str(|str| str.push_str(s));
    }
}

impl<S: ?Sized + StringToFromBytes, B> Clone for Boxed<S, B>
where
    S::Owned: OwnedToFromBoxed<S, BoxType = B>,
{
    fn clone(&self) -> Self {
        Self {
            inner: S::Owned::clone_boxed(&self.inner),
            _marker: PhantomData,
        }
    }
}
