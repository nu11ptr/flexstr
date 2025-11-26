use alloc::{boxed::Box, rc::Rc, sync::Arc, vec::Vec};

use crate::Flex;

pub type LocalBytes<'s> = Flex<'s, [u8], Rc<[u8]>>;
pub type SharedBytes<'s> = Flex<'s, [u8], Arc<[u8]>>;
pub type BoxBytes<'s> = Flex<'s, [u8], Box<[u8]>>;

pub type LocalVec<'s> = Flex<'s, Vec<u8>, Rc<Vec<u8>>>;
pub type SharedVec<'s> = Flex<'s, Vec<u8>, Arc<Vec<u8>>>;
pub type BoxVec<'s> = Flex<'s, Vec<u8>, Box<Vec<u8>>>;
