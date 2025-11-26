use alloc::{boxed::Box, ffi::CString, rc::Rc, sync::Arc};
use core::ffi::CStr;

use crate::Flex;

pub type LocalCStr<'s> = Flex<'s, CStr, Rc<CStr>>;
pub type SharedCStr<'s> = Flex<'s, CStr, Arc<CStr>>;
pub type BoxCStr<'s> = Flex<'s, CStr, Box<CStr>>;

pub type LocalCString<'s> = Flex<'s, CString, Rc<CString>>;
pub type SharedCString<'s> = Flex<'s, CString, Arc<CString>>;
pub type BoxCString<'s> = Flex<'s, CString, Box<CString>>;
