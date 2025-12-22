#![allow(dead_code)]

use core::fmt;
use flexstry::{FlexStr, RefCounted, StringLike};
use flexstr_support::StringToFromBytes;

/// Test as_str() method for str types
pub fn test_as_str<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + AsRef<str> + PartialEq,
    R: RefCounted<S>,
    FlexStr<'static, S, R>: StringLike<S>,
{
    let flex_str: FlexStr<'_, S, R> = FlexStr::from_borrowed(s);
    let str_ref = StringLike::as_str(&flex_str);
    assert_eq!(str_ref, s.as_ref());
}

/// Test as_os_str() method for OsStr types
#[cfg(all(feature = "std", feature = "osstr"))]
pub fn test_as_os_str<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + AsRef<std::ffi::OsStr> + PartialEq,
    R: RefCounted<S>,
    FlexStr<'static, S, R>: StringLike<S>,
{
    let flex_str: FlexStr<'_, S, R> = FlexStr::from_borrowed(s);
    let os_str_ref = StringLike::as_os_str(&flex_str);
    assert_eq!(os_str_ref, s.as_ref());
}

/// Test as_path() method for Path types
#[cfg(all(feature = "std", feature = "path"))]
pub fn test_as_path<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + AsRef<std::path::Path> + PartialEq,
    R: RefCounted<S>,
    FlexStr<'static, S, R>: StringLike<S>,
{
    let flex_str: FlexStr<'_, S, R> = FlexStr::from_borrowed(s);
    let path_ref = StringLike::as_path(&flex_str);
    assert_eq!(path_ref, s.as_ref());
}

/// Test as_c_str() method for CStr types
#[cfg(feature = "cstr")]
pub fn test_as_c_str<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + AsRef<core::ffi::CStr> + PartialEq,
    R: RefCounted<S>,
    FlexStr<'static, S, R>: StringLike<S>,
{
    let flex_str: FlexStr<'_, S, R> = FlexStr::from_borrowed(s);
    let c_str_ref = StringLike::as_c_str(&flex_str);
    assert_eq!(c_str_ref, s.as_ref());
}

/// Test into_string() method
pub fn test_into_string<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S>,
    S::Owned: Into<String> + From<alloc::boxed::Box<S>> + AsRef<S>,
    FlexStr<'static, S, R>: StringLike<S>,
{
    let flex_str: FlexStr<'_, S, R> = FlexStr::from_borrowed(s);
    let string = StringLike::into_string(flex_str);
    assert_eq!(string, s.to_owned().into());
}

/// Test to_string() method
pub fn test_to_string<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S>,
    S::Owned: Into<String>,
    FlexStr<'static, S, R>: StringLike<S>,
{
    let flex_str: FlexStr<'_, S, R> = FlexStr::from_borrowed(s);
    let string = StringLike::to_string(&flex_str);
    assert_eq!(string, s.to_owned().into());
}

/// Test into_os_string() method
#[cfg(all(feature = "std", feature = "osstr"))]
pub fn test_into_os_string<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S>,
    S::Owned: Into<std::ffi::OsString> + From<alloc::boxed::Box<S>> + AsRef<S>,
    FlexStr<'static, S, R>: StringLike<S>,
{
    let flex_str: FlexStr<'_, S, R> = FlexStr::from_borrowed(s);
    let os_string = StringLike::into_os_string(flex_str);
    assert_eq!(os_string, s.to_owned().into());
}

/// Test to_os_string() method
#[cfg(all(feature = "std", feature = "osstr"))]
pub fn test_to_os_string<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S>,
    S::Owned: Into<std::ffi::OsString>,
    FlexStr<'static, S, R>: StringLike<S>,
{
    let flex_str: FlexStr<'_, S, R> = FlexStr::from_borrowed(s);
    let os_string = StringLike::to_os_string(&flex_str);
    assert_eq!(os_string, s.to_owned().into());
}

/// Test into_path_buf() method
#[cfg(all(feature = "std", feature = "path"))]
pub fn test_into_path_buf<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S>,
    S::Owned: Into<std::path::PathBuf> + From<alloc::boxed::Box<S>> + AsRef<S>,
    FlexStr<'static, S, R>: StringLike<S>,
{
    let flex_str: FlexStr<'_, S, R> = FlexStr::from_borrowed(s);
    let path_buf = StringLike::into_path_buf(flex_str);
    assert_eq!(path_buf, s.to_owned().into());
}

/// Test to_path_buf() method
#[cfg(all(feature = "std", feature = "path"))]
pub fn test_to_path_buf<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S>,
    S::Owned: Into<std::path::PathBuf>,
    FlexStr<'static, S, R>: StringLike<S>,
{
    let flex_str: FlexStr<'_, S, R> = FlexStr::from_borrowed(s);
    let path_buf = StringLike::to_path_buf(&flex_str);
    assert_eq!(path_buf, s.to_owned().into());
}

/// Test into_c_string() method
#[cfg(feature = "cstr")]
pub fn test_into_c_string<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S> + fmt::Debug,
    S::Owned: Into<alloc::ffi::CString> + From<alloc::boxed::Box<S>> + AsRef<S>,
    FlexStr<'static, S, R>: StringLike<S>,
{
    let flex_str: FlexStr<'_, S, R> = FlexStr::from_borrowed(s);
    let c_string = StringLike::into_c_string(flex_str);
    assert_eq!(c_string.as_bytes_with_nul(), s.to_owned().into().as_bytes_with_nul());
}

/// Test to_c_string() method
#[cfg(feature = "cstr")]
pub fn test_to_c_string<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S> + fmt::Debug,
    S::Owned: Into<alloc::ffi::CString>,
    FlexStr<'static, S, R>: StringLike<S>,
{
    let flex_str: FlexStr<'_, S, R> = FlexStr::from_borrowed(s);
    let c_string = StringLike::to_c_string(&flex_str);
    let expected: alloc::ffi::CString = s.to_owned().into();
    assert_eq!(c_string.as_bytes_with_nul(), expected.as_bytes_with_nul());
}

/// Test into_vec_bytes() method
#[cfg(feature = "bytes")]
pub fn test_into_vec_bytes<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S>,
    S::Owned: Into<alloc::vec::Vec<u8>> + From<alloc::boxed::Box<S>> + AsRef<S>,
    FlexStr<'static, S, R>: StringLike<S>,
{
    let flex_str: FlexStr<'_, S, R> = FlexStr::from_borrowed(s);
    let vec_bytes = StringLike::into_vec_bytes(flex_str);
    assert_eq!(vec_bytes, s.to_owned().into());
}

/// Test to_vec_bytes() method
#[cfg(feature = "bytes")]
pub fn test_to_vec_bytes<S, R>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S>,
    S::Owned: Into<alloc::vec::Vec<u8>>,
    FlexStr<'static, S, R>: StringLike<S>,
{
    let flex_str: FlexStr<'_, S, R> = FlexStr::from_borrowed(s);
    let vec_bytes = StringLike::to_vec_bytes(&flex_str);
    assert_eq!(vec_bytes, s.to_owned().into());
}

