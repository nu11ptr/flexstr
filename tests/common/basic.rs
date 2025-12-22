#![allow(dead_code)]

use core::fmt;
use flexstr::{FlexStr, RefCounted, StringLike};
use flexstr_support::StringToFromBytes;
use inline_flexstr::InlineFlexStr;

/// Test creation from borrowed string
pub fn test_creation_from_borrowed<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S>,
{
    let flex_str: FlexStr<'_, S, R> = FlexStr::from_borrowed(s);
    assert!(flex_str.is_borrowed());
    assert_eq!(flex_str.as_ref_type(), s);
}

/// Test creation from owned string
pub fn test_creation_from_owned<S, R>(owned: S::Owned)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S>,
    Box<S>: From<S::Owned>,
    S::Owned: AsRef<S>,
{
    let flex_str: FlexStr<'static, S, R> = FlexStr::from_owned(owned.as_ref().to_owned());
    assert!(flex_str.is_boxed());
    assert_eq!(flex_str.as_ref_type(), owned.as_ref());
}

/// Test creation from inline string
pub fn test_creation_from_inline<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S>,
{
    let inline_str = InlineFlexStr::try_from_type(s).unwrap();
    let flex_str: FlexStr<'_, S, R> = FlexStr::from_inline(inline_str);
    assert!(flex_str.is_inlined());
    assert_eq!(flex_str.as_ref_type(), s);
}

/// Test creation from reference counted string
pub fn test_creation_from_ref_counted<S, R>(s: R)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S>,
{
    let flex_str: FlexStr<'_, S, R> = FlexStr::from_ref_counted(s.clone());
    assert!(flex_str.is_ref_counted());
    assert_eq!(flex_str.as_ref_type(), &*s);
}

/// Test creation from boxed string
pub fn test_creation_from_boxed<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S>,
    Box<S>: From<S::Owned>,
    S::Owned: AsRef<S>,
{
    let boxed = Box::from(s.to_owned());
    let flex_str: FlexStr<'_, S, R> = FlexStr::from_boxed(boxed);
    assert!(flex_str.is_boxed());
    assert_eq!(flex_str.as_ref_type(), s);
}

/// Test empty string creation
pub fn test_empty<S, R>(empty: &'static S)
where
    S: ?Sized + StringToFromBytes,
    R: RefCounted<S>,
    FlexStr<'static, S, R>: StringLike<S>,
{
    let flex_str: FlexStr<'_, S, R> = FlexStr::from_borrowed(empty);
    assert!(flex_str.is_borrowed());
    assert!(StringLike::is_empty(&flex_str));
    assert_eq!(StringLike::len(&flex_str), 0);
}

/// Test accessor methods
pub fn test_accessors<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S>,
    FlexStr<'static, S, R>: StringLike<S>,
{
    let flex_str: FlexStr<'_, S, R> = FlexStr::from_borrowed(s);

    // Test as_ref_type
    assert_eq!(flex_str.as_ref_type(), s);

    // Test as_bytes
    let bytes = flex_str.as_bytes();
    assert_eq!(bytes, S::self_as_bytes(s));

    // Test as_raw_bytes
    let raw_bytes = flex_str.as_raw_bytes();
    assert_eq!(raw_bytes, S::self_as_raw_bytes(s));

    // Test len
    assert_eq!(StringLike::len(&flex_str), s.self_as_bytes().len());

    // Test is_empty
    assert_eq!(
        StringLike::is_empty(&flex_str),
        s.self_as_bytes().is_empty()
    );
}

/// Test cloning for all variants
pub fn test_clone_all_variants<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + PartialEq + fmt::Debug,
    R: RefCounted<S> + fmt::Debug,
    Box<S>: From<S::Owned>,
    S::Owned: AsRef<S>,
{
    // Test clone for borrowed
    let borrowed: FlexStr<'_, S, R> = FlexStr::from_borrowed(s);
    let cloned = borrowed.clone();
    assert_eq!(borrowed, cloned);

    // Test clone for inlined (input should be small enough to inline)
    let inline_str =
        InlineFlexStr::try_from_type(s).expect("test input should be small enough to inline");
    let inlined: FlexStr<'_, S, R> = FlexStr::from_inline(inline_str);
    let cloned = inlined.clone();
    assert_eq!(inlined, cloned);

    // Test clone for ref counted
    let rc: R = s.into();
    let ref_counted: FlexStr<'_, S, R> = FlexStr::from_ref_counted(rc.clone());
    let cloned = ref_counted.clone();
    assert_eq!(ref_counted, cloned);

    // Test clone for boxed
    let boxed: FlexStr<'_, S, R> = FlexStr::from_boxed(Box::from(s.to_owned()));
    let cloned = boxed.clone();
    assert_eq!(boxed, cloned);
}

/// Test Default implementation
/// Note: This test is only applicable for types where `&S: Default`
pub fn test_default<S, R>()
where
    S: ?Sized + StringToFromBytes,
    R: RefCounted<S>,
    for<'a> &'a S: Default,
    FlexStr<'static, S, R>: StringLike<S>,
{
    let flex_str = FlexStr::default();
    assert!(flex_str.is_borrowed());
    assert!(StringLike::is_empty(&flex_str));
}
