use alloc::{rc::Rc, sync::Arc};
use core::ffi::CStr;

use crate::Flex;

pub type LocalCStr<'s> = Flex<'s, CStr, Rc<CStr>>;
pub type SharedCStr<'s> = Flex<'s, CStr, Arc<CStr>>;
