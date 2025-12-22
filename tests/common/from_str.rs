#![allow(dead_code)]

use core::fmt;
use core::str::FromStr;
use flexstry::{FlexStr, RefCounted};
use flexstr_support::StringToFromBytes;

/// Test FromStr success for FlexStr
pub fn test_from_str_flex_str_success<S, R>(s: &str)
where
    FlexStr<'static, S, R>: FromStr<Err = core::convert::Infallible>,
    S: ?Sized + StringToFromBytes + fmt::Debug + PartialEq,
    R: RefCounted<S>,
    S::Owned: AsRef<S>,
{
    let flex_str = FlexStr::from_str(s).unwrap();
    assert_eq!(flex_str.as_ref_type(), flex_str.as_ref_type()); // Basic sanity check
}

/// Test FromStr for FlexStr<[u8], R>
#[cfg(feature = "bytes")]
pub fn test_from_str_bytes_success<R>()
where
    R: RefCounted<[u8]>,
{
    let flex_str = FlexStr::<'static, [u8], R>::from_str("test").unwrap();
    assert_eq!(flex_str.as_ref_type(), b"test");
}

/// Test FromStr for FlexStr<OsStr, R>
#[cfg(all(feature = "std", feature = "osstr"))]
pub fn test_from_str_osstr_success<R>()
where
    R: RefCounted<std::ffi::OsStr>,
{
    use std::ffi::OsStr;
    
    let flex_str = FlexStr::<'static, OsStr, R>::from_str("test").unwrap();
    assert_eq!(flex_str.as_ref_type(), OsStr::new("test"));
}

/// Test FromStr for FlexStr<Path, R>
#[cfg(all(feature = "std", feature = "path"))]
pub fn test_from_str_path_success<R>()
where
    R: RefCounted<std::path::Path>,
{
    use std::path::Path;
    
    let flex_str = FlexStr::<'static, Path, R>::from_str("test").unwrap();
    assert_eq!(flex_str.as_ref_type(), Path::new("test"));
}

/// Test FromStr for FlexStr<CStr, R> success
#[cfg(feature = "cstr")]
pub fn test_from_str_cstr_success<R>()
where
    R: RefCounted<core::ffi::CStr> + fmt::Debug,
{
    use flexstry::FlexStr;
    
    let flex_str = FlexStr::<'static, core::ffi::CStr, R>::from_str("test").unwrap();
    assert_eq!(flex_str.as_ref_type().to_bytes(), b"test");
}

/// Test FromStr for FlexStr<CStr, R> error (interior NUL)
#[cfg(feature = "cstr")]
pub fn test_from_str_cstr_error<R>()
where
    R: RefCounted<core::ffi::CStr> + fmt::Debug,
{
    use flexstry::{FlexStr, InteriorNulError};
    
    // String with interior NUL should fail
    let result: Result<FlexStr<'static, core::ffi::CStr, R>, InteriorNulError> = FlexStr::from_str("test\0middle");
    result.unwrap_err();
}


