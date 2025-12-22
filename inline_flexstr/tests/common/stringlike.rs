#![allow(dead_code)]

use core::fmt;
use flexstr_support::{StringLike, StringToFromBytes};
use inline_flexstr::InlineFlexStr;

/// Test as_str() method for str types
#[cfg(feature = "str")]
pub fn test_as_str<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + AsRef<str> + PartialEq,
    InlineFlexStr<S>: StringLike<S>,
{
    // Input should be small enough to inline
    let inline_str =
        InlineFlexStr::try_from_type(s).expect("test input should be small enough to inline");
    let str_ref = StringLike::as_str(&inline_str);
    assert_eq!(str_ref, s.as_ref());
}

/// Test as_os_str() method for OsStr types
#[cfg(all(feature = "std", feature = "osstr"))]
pub fn test_as_os_str<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + AsRef<std::ffi::OsStr> + PartialEq,
    InlineFlexStr<S>: StringLike<S>,
{
    // Input should be small enough to inline
    let inline_str =
        InlineFlexStr::try_from_type(s).expect("test input should be small enough to inline");
    let os_str_ref = StringLike::as_os_str(&inline_str);
    assert_eq!(os_str_ref, s.as_ref());
}

/// Test as_path() method for Path types
#[cfg(all(feature = "std", feature = "path"))]
pub fn test_as_path<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + AsRef<std::path::Path> + PartialEq,
    InlineFlexStr<S>: StringLike<S>,
{
    // Input should be small enough to inline
    let inline_str =
        InlineFlexStr::try_from_type(s).expect("test input should be small enough to inline");
    let path_ref = StringLike::as_path(&inline_str);
    assert_eq!(path_ref, s.as_ref());
}

/// Test as_c_str() method for CStr types
#[cfg(feature = "cstr")]
pub fn test_as_c_str<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + AsRef<core::ffi::CStr> + PartialEq,
    InlineFlexStr<S>: StringLike<S>,
{
    // Input should be small enough to inline
    let inline_str =
        InlineFlexStr::try_from_type(s).expect("test input should be small enough to inline");
    let c_str_ref = StringLike::as_c_str(&inline_str);
    assert_eq!(c_str_ref, s.as_ref());
}

/// Test into_string() method
#[cfg(feature = "str")]
pub fn test_into_string<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    S::Owned: Into<String> + From<alloc::boxed::Box<S>> + AsRef<S>,
    InlineFlexStr<S>: StringLike<S>,
{
    // Input should be small enough to inline
    let inline_str =
        InlineFlexStr::try_from_type(s).expect("test input should be small enough to inline");
    let string = StringLike::into_string(inline_str);
    assert_eq!(string, s.to_owned().into());
}

/// Test to_string() method
#[cfg(feature = "str")]
pub fn test_to_string<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    S::Owned: Into<String>,
    InlineFlexStr<S>: StringLike<S>,
{
    // Input should be small enough to inline
    let inline_str =
        InlineFlexStr::try_from_type(s).expect("test input should be small enough to inline");
    let string = StringLike::to_string(&inline_str);
    assert_eq!(string, s.to_owned().into());
}

/// Test into_os_string() method
#[cfg(all(feature = "std", feature = "osstr"))]
pub fn test_into_os_string<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    S::Owned: Into<std::ffi::OsString> + From<alloc::boxed::Box<S>> + AsRef<S>,
    InlineFlexStr<S>: StringLike<S>,
{
    // Input should be small enough to inline
    let inline_str =
        InlineFlexStr::try_from_type(s).expect("test input should be small enough to inline");
    let os_string = StringLike::into_os_string(inline_str);
    assert_eq!(os_string, s.to_owned().into());
}

/// Test to_os_string() method
#[cfg(all(feature = "std", feature = "osstr"))]
pub fn test_to_os_string<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    S::Owned: Into<std::ffi::OsString>,
    InlineFlexStr<S>: StringLike<S>,
{
    // Input should be small enough to inline
    let inline_str =
        InlineFlexStr::try_from_type(s).expect("test input should be small enough to inline");
    let os_string = StringLike::to_os_string(&inline_str);
    assert_eq!(os_string, s.to_owned().into());
}

/// Test into_path_buf() method
#[cfg(all(feature = "std", feature = "path"))]
pub fn test_into_path_buf<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    S::Owned: Into<std::path::PathBuf> + From<alloc::boxed::Box<S>> + AsRef<S>,
    InlineFlexStr<S>: StringLike<S>,
{
    // Input should be small enough to inline
    let inline_str =
        InlineFlexStr::try_from_type(s).expect("test input should be small enough to inline");
    let path_buf = StringLike::into_path_buf(inline_str);
    assert_eq!(path_buf, s.to_owned().into());
}

/// Test to_path_buf() method
#[cfg(all(feature = "std", feature = "path"))]
pub fn test_to_path_buf<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    S::Owned: Into<std::path::PathBuf>,
    InlineFlexStr<S>: StringLike<S>,
{
    // Input should be small enough to inline
    let inline_str =
        InlineFlexStr::try_from_type(s).expect("test input should be small enough to inline");
    let path_buf = StringLike::to_path_buf(&inline_str);
    assert_eq!(path_buf, s.to_owned().into());
}

/// Test into_c_string() method
#[cfg(feature = "cstr")]
pub fn test_into_c_string<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    S::Owned: Into<alloc::ffi::CString> + From<alloc::boxed::Box<S>> + AsRef<S>,
    InlineFlexStr<S>: StringLike<S>,
{
    // Input should be small enough to inline
    let inline_str =
        InlineFlexStr::try_from_type(s).expect("test input should be small enough to inline");
    let c_string = StringLike::into_c_string(inline_str);
    assert_eq!(c_string.as_bytes_with_nul(), s.to_owned().into().as_bytes_with_nul());
}

/// Test to_c_string() method
#[cfg(feature = "cstr")]
pub fn test_to_c_string<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    S::Owned: Into<alloc::ffi::CString>,
    InlineFlexStr<S>: StringLike<S>,
{
    // Input should be small enough to inline
    let inline_str =
        InlineFlexStr::try_from_type(s).expect("test input should be small enough to inline");
    let c_string = StringLike::to_c_string(&inline_str);
    let expected: alloc::ffi::CString = s.to_owned().into();
    assert_eq!(c_string.as_bytes_with_nul(), expected.as_bytes_with_nul());
}

/// Test into_vec_bytes() method
#[cfg(feature = "bytes")]
pub fn test_into_vec_bytes<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    S::Owned: Into<alloc::vec::Vec<u8>> + From<alloc::boxed::Box<S>> + AsRef<S>,
    InlineFlexStr<S>: StringLike<S>,
{
    // Input should be small enough to inline
    let inline_str =
        InlineFlexStr::try_from_type(s).expect("test input should be small enough to inline");
    let vec_bytes = StringLike::into_vec_bytes(inline_str);
    assert_eq!(vec_bytes, s.to_owned().into());
}

/// Test to_vec_bytes() method
#[cfg(feature = "bytes")]
pub fn test_to_vec_bytes<S>(s: &'static S)
where
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    S::Owned: Into<alloc::vec::Vec<u8>>,
    InlineFlexStr<S>: StringLike<S>,
{
    // Input should be small enough to inline
    let inline_str =
        InlineFlexStr::try_from_type(s).expect("test input should be small enough to inline");
    let vec_bytes = StringLike::to_vec_bytes(&inline_str);
    assert_eq!(vec_bytes, s.to_owned().into());
}

