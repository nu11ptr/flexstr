use alloc::{rc::Rc, sync::Arc};

use crate::Flex;

pub type LocalBytes<'s> = Flex<'s, [u8], Rc<[u8]>>;
pub type SharedBytes<'s> = Flex<'s, [u8], Arc<[u8]>>;
