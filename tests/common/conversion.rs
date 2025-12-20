#![allow(dead_code)]

use core::fmt;
use flexstry::{FlexStr, InlineFlexStr, RefCounted, StringLike, StringToFromBytes};

/// Test to_owned conversion
pub fn test_to_owned<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S>,
    Box<S>: From<S::Owned>,
    S::Owned: AsRef<S>,
{
    let borrowed: FlexStr<'_, S, R> = FlexStr::from_borrowed(s);
    let owned = borrowed.to_owned();
    assert_eq!(owned.as_ref_type(), s);
    assert!(matches!(
        owned,
        FlexStr::Inlined(_) | FlexStr::RefCounted(_)
    ));
}

/// Test into_owned conversion
pub fn test_into_owned<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S>,
    Box<S>: From<S::Owned>,
    S::Owned: AsRef<S>,
{
    let borrowed: FlexStr<'_, S, R> = FlexStr::from_borrowed(s);
    let owned = borrowed.into_owned();
    assert_eq!(owned.as_ref_type(), s);
    assert!(!owned.is_borrowed());
}

/// Test to_owned_type conversion
pub fn test_to_owned_type<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S>,
    S::Owned: PartialEq + AsRef<S>,
    FlexStr<'static, S, R>: StringLike<S>,
{
    let flex_str: FlexStr<'_, S, R> = FlexStr::from_borrowed(s);
    let owned = StringLike::to_owned_type(&flex_str);
    assert_eq!(owned.as_ref(), s);
}

/// Test into_owned_type conversion
pub fn test_into_owned_type<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + PartialEq + fmt::Debug,
    R: RefCounted<S>,
    Box<S>: From<S::Owned>,
    S::Owned: PartialEq + AsRef<S> + From<Box<S>>,
    FlexStr<'static, S, R>: StringLike<S>,
{
    let flex_str: FlexStr<'_, S, R> = FlexStr::from_borrowed(s);
    let owned = StringLike::into_owned_type(flex_str);
    assert_eq!(owned.as_ref(), s);
}

/// Test to_local conversion (from Shared to Local)
pub fn test_to_local<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    alloc::sync::Arc<S>: for<'a> From<&'a S>,
    alloc::rc::Rc<S>: for<'a> From<&'a S>,
{
    use alloc::sync::Arc;

    let shared: FlexStr<'_, S, Arc<S>> = FlexStr::from_borrowed(s);
    let local = shared.to_local();
    assert_eq!(local.as_ref_type(), s);
    assert!(local.is_borrowed() || local.is_inlined() || local.is_ref_counted());
}

/// Test into_local conversion (from Shared to Local)
pub fn test_into_local<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    alloc::sync::Arc<S>: for<'a> From<&'a S>,
    alloc::rc::Rc<S>: for<'a> From<&'a S>,
{
    use alloc::sync::Arc;

    let shared: FlexStr<'_, S, Arc<S>> = FlexStr::from_borrowed(s);
    let local = shared.into_local();
    assert_eq!(local.as_ref_type(), s);
}

/// Test to_shared conversion (from Local to Shared)
pub fn test_to_shared<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    alloc::sync::Arc<S>: for<'a> From<&'a S>,
    alloc::rc::Rc<S>: for<'a> From<&'a S>,
{
    use alloc::rc::Rc;

    let local: FlexStr<'_, S, Rc<S>> = FlexStr::from_borrowed(s);
    let shared = local.to_shared();
    assert_eq!(shared.as_ref_type(), s);
    assert!(shared.is_borrowed() || shared.is_inlined() || shared.is_ref_counted());
}

/// Test into_shared conversion (from Local to Shared)
pub fn test_into_shared<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    alloc::sync::Arc<S>: for<'a> From<&'a S>,
    alloc::rc::Rc<S>: for<'a> From<&'a S>,
{
    use alloc::rc::Rc;

    let local: FlexStr<'_, S, Rc<S>> = FlexStr::from_borrowed(s);
    let shared = local.into_shared();
    assert_eq!(shared.as_ref_type(), s);
}

/// Test optimize method
pub fn test_optimize<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S>,
    Box<S>: From<S::Owned>,
    S::Owned: AsRef<S>,
{
    // Test optimize on boxed (should convert to inlined or ref_counted)
    let boxed: FlexStr<'_, S, R> = FlexStr::from_boxed(Box::from(s.to_owned()));
    let optimized = boxed.optimize();
    assert_eq!(optimized.as_ref_type(), s);
    assert!(matches!(
        optimized,
        FlexStr::Inlined(_) | FlexStr::RefCounted(_)
    ));

    // Test optimize on borrowed (should stay borrowed)
    let borrowed: FlexStr<'_, S, R> = FlexStr::from_borrowed(s);
    let optimized = borrowed.optimize();
    assert_eq!(optimized.as_ref_type(), s);
    assert!(optimized.is_borrowed());
}

/// Test From<&S> implementation
pub fn test_from_borrowed_ref<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S>,
{
    let flex_str: FlexStr<'_, S, R> = s.into();
    assert!(flex_str.is_borrowed());
    assert_eq!(flex_str.as_ref_type(), s);
}

/// Test From<Box<S>> implementation
pub fn test_from_box<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S>,
    Box<S>: From<S::Owned>,
    S::Owned: AsRef<S>,
{
    let boxed = Box::from(s.to_owned());
    let flex_str: FlexStr<'_, S, R> = boxed.into();
    assert!(flex_str.is_boxed());
    assert_eq!(flex_str.as_ref_type(), s);
}

/// Test From<InlineFlexStr> implementation
pub fn test_from_inline_flex_str<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S>,
{
    // Input should be small enough to inline
    let inline_str =
        InlineFlexStr::try_from_type(s).expect("test input should be small enough to inline");
    let flex_str: FlexStr<'_, S, R> = inline_str.into();
    assert!(flex_str.is_inlined());
    assert_eq!(flex_str.as_ref_type(), s);
}

/// Test From<Cow> implementation
pub fn test_from_cow<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S>,
    Box<S>: From<S::Owned>,
    S::Owned: AsRef<S>,
{
    use alloc::borrow::Cow;

    // Test Cow::Borrowed
    let cow: Cow<'_, S> = Cow::Borrowed(s);
    let flex_str: FlexStr<'_, S, R> = cow.into();
    assert!(flex_str.is_borrowed());
    assert_eq!(flex_str.as_ref_type(), s);

    // Test Cow::Owned
    let cow: Cow<'_, S> = Cow::Owned(s.to_owned());
    let flex_str: FlexStr<'_, S, R> = cow.into();
    assert!(flex_str.is_boxed());
    assert_eq!(flex_str.as_ref_type(), s);
}
