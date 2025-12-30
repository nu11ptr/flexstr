use core::marker::PhantomData;
use core::ptr::NonNull;

use flexstr_support::StringToFromBytes;

use crate::boxed::OwnedToFromBoxed;

// *** LengthCapacity - 32-bit (16-bit length/capacity) ***

#[cfg(all(not(feature = "large_strings"), target_pointer_width = "32"))]
#[derive(Clone, Copy)]
struct LengthCapacity {
    len: u16,
    cap: u16,
}

#[cfg(all(not(feature = "large_strings"), target_pointer_width = "32"))]
impl LengthCapacity {
    const MAX_CAPACITY: usize = u16::MAX as usize;

    #[inline]
    pub fn new(len: usize, cap: usize) -> Self {
        if cap > Self::MAX_CAPACITY {
            panic!("String is too large after mutation");
        }
        Self {
            len: len as u16,
            cap: cap as u16,
        }
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.len as usize
    }

    #[inline]
    pub fn cap(&self) -> usize {
        self.cap as usize
    }
}

// *** LengthCapacity - 32-bit (24-bit length/capacity) ***

#[cfg(all(feature = "large_strings", target_pointer_width = "32"))]
#[derive(Clone, Copy)]
struct LengthCapacity {
    len: u16,
    cap: u16,
    len_extra: u8,
    cap_extra: u8,
}

#[cfg(all(feature = "large_strings", target_pointer_width = "32"))]
impl LengthCapacity {
    const MAX_CAPACITY: usize = u16::MAX as usize + ((u8::MAX as usize) << 16);

    #[inline]
    pub fn new(len: usize, cap: usize) -> Self {
        if cap > Self::MAX_CAPACITY {
            panic!("String is too large after mutation");
        }
        Self {
            len: len as u16,
            cap: cap as u16,
            len_extra: (len >> 16) as u8,
            cap_extra: (cap >> 16) as u8,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len as usize + ((self.len_extra as usize) << 16)
    }

    #[inline]
    pub fn cap(&self) -> usize {
        self.cap as usize + ((self.cap_extra as usize) << 16)
    }
}

// *** LengthCapacity - 64-bit (32-bit length/capacity) ***

#[cfg(all(not(feature = "large_strings"), target_pointer_width = "64"))]
#[derive(Clone, Copy)]
struct LengthCapacity {
    len: u32,
    cap: u32,
}

#[cfg(all(not(feature = "large_strings"), target_pointer_width = "64"))]
impl LengthCapacity {
    const MAX_CAPACITY: usize = u32::MAX as usize;

    #[inline]
    pub fn new(len: usize, cap: usize) -> Self {
        if cap > Self::MAX_CAPACITY {
            panic!("String is too large after mutation");
        }
        Self {
            len: len as u32,
            cap: cap as u32,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len as usize
    }

    #[inline]
    pub fn cap(&self) -> usize {
        self.cap as usize
    }
}

// *** LengthCapacity - 64-bit (48-bit length/capacity) ***

#[cfg(all(feature = "large_strings", target_pointer_width = "64"))]
#[derive(Clone, Copy)]
struct LengthCapacity {
    len: u32,
    cap: u32,
    len_extra: u16,
    cap_extra: u16,
}

#[cfg(all(feature = "large_strings", target_pointer_width = "64"))]
impl LengthCapacity {
    const MAX_CAPACITY: usize = u32::MAX as usize + ((u16::MAX as usize) << 32);

    #[inline]
    pub fn new(len: usize, cap: usize) -> Self {
        if cap > Self::MAX_CAPACITY {
            panic!("String is too large after mutation");
        }
        Self {
            len: len as u32,
            cap: cap as u32,
            len_extra: (len >> 32) as u16,
            cap_extra: (cap >> 32) as u16,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len as usize + ((self.len_extra as usize) << 32)
    }

    #[inline]
    pub fn cap(&self) -> usize {
        self.cap as usize + ((self.cap_extra as usize) << 32)
    }
}

// *** SmallBox ***

pub struct SmallBox<S: ?Sized + StringToFromBytes>
where
    S::Owned: OwnedToFromBoxed<S, BoxType = SmallBox<S>>,
{
    ptr: NonNull<u8>,
    len_cap: LengthCapacity,
    _marker: PhantomData<S>,
}

impl<S: ?Sized + StringToFromBytes> SmallBox<S>
where
    S::Owned: OwnedToFromBoxed<S, BoxType = SmallBox<S>>,
{
    pub(crate) unsafe fn new(ptr: *const u8, len: usize, cap: usize) -> Self {
        Self {
            // SAFETY: The caller is responsible for ensuring the pointer is valid
            ptr: unsafe { NonNull::new_unchecked(ptr as *mut u8) },
            len_cap: LengthCapacity::new(len, cap),
            _marker: PhantomData,
        }
    }

    pub fn ptr(&self) -> *const u8 {
        self.ptr.as_ptr()
    }

    pub fn len(&self) -> usize {
        self.len_cap.len()
    }

    pub fn cap(&self) -> usize {
        self.len_cap.cap()
    }
}

impl<S: ?Sized + StringToFromBytes> Drop for SmallBox<S>
where
    S::Owned: OwnedToFromBoxed<S, BoxType = SmallBox<S>>,
{
    fn drop(&mut self) {
        S::Owned::from_boxed(self);
    }
}
