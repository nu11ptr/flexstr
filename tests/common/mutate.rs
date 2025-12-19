use core::fmt;

use flexstry::{FlexStr, RefCountedMut, StringFromBytesMut};

#[allow(dead_code)]
pub fn test_mutation_borrowed<S, R>(s: &'static S)
where
    S: ?Sized + StringFromBytesMut + fmt::Debug + PartialEq,
    R: RefCountedMut<S>,
{
    let mut flex_str: FlexStr<S, R> = s.into();
    assert!(flex_str.is_borrowed());

    let s2 = flex_str.to_mut_type();
    assert_eq!(s2, s);

    assert!(flex_str.is_inlined());
    let s2 = flex_str.to_mut_type();
    assert_eq!(s2, s);
}

#[allow(dead_code)]
pub fn test_mutation_inlined<S, R>(s: &'static S)
where
    S: ?Sized + StringFromBytesMut + fmt::Debug + PartialEq,
    R: RefCountedMut<S>,
{
    let mut flex_str: FlexStr<S, R> = s.into();
    flex_str = flex_str.into_owned();
    assert!(flex_str.is_inlined());

    let s2 = flex_str.to_mut_type();
    assert_eq!(s2, s);
}

#[allow(dead_code)]
pub fn test_mutation_ref_counted<S, R>(s: R)
where
    S: ?Sized + StringFromBytesMut + fmt::Debug + PartialEq,
    R: RefCountedMut<S> + for<'a> Into<FlexStr<'a, S, R>>,
{
    let mut flex_str: FlexStr<S, R> = s.clone().into();
    assert!(flex_str.is_ref_counted());

    let s2 = flex_str.to_mut_type();
    assert_eq!(s2, &*s);
}

#[allow(dead_code)]
pub fn test_mutation_boxed<S, R>(s: S::Owned)
where
    S: ?Sized + StringFromBytesMut + fmt::Debug + PartialEq,
    R: RefCountedMut<S>,
    S::Owned: for<'a> Into<FlexStr<'a, S, R>> + Clone + AsRef<S>,
{
    let mut flex_str: FlexStr<S, R> = s.clone().into();
    assert!(flex_str.is_boxed());

    let s2 = flex_str.to_mut_type();
    assert_eq!(s2, s.as_ref());
}
