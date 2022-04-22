//
// WARNING: This file has been auto-generated using flexgen (https://github.com/nu11ptr/flexgen).
// Any manual modifications to this file will be overwritten the next time this file is generated.
//

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::sync::Arc;
use core::ops::Deref;
use std::ffi::OsStr;

use crate::custom::{PTR_SIZED_PAD, STRING_SIZED_INLINE};
use crate::inner::FlexStrInner;
use crate::storage::Storage;
use crate::traits::private::FlexStrCoreInner;
use crate::traits::{private, FlexStrCore};

// *** Regular Type ***

/// A flexible string type that transparently wraps a string literal, inline string, or an
/// [`Rc<OsStr>`](std::rc::Rc)
#[repr(transparent)]
pub struct FlexOsStr<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>(
    pub(crate) FlexStrInner<'str, SIZE, BPAD, HPAD, HEAP, OsStr>,
);

// ###  Clone ###

impl<'str, const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> Clone
    for FlexOsStr<'str, SIZE, PAD1, PAD2, HEAP>
where
    HEAP: Storage<OsStr> + Clone,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

// ### Deref ###

impl<'str, const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> Deref
    for FlexOsStr<'str, SIZE, PAD1, PAD2, HEAP>
where
    HEAP: Storage<OsStr>,
{
    type Target = OsStr;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.0.as_str_type()
    }
}

// ### FlexStrCoreInner ###

impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    private::FlexStrCoreInner<'str, SIZE, BPAD, HPAD, HEAP, OsStr>
    for FlexOsStr<'str, SIZE, BPAD, HPAD, HEAP>
where
    HEAP: Storage<OsStr>,
{
    type This = Self;
    #[inline(always)]
    fn wrap(inner: FlexStrInner<'str, SIZE, BPAD, HPAD, HEAP, OsStr>) -> Self::This {
        Self(inner)
    }
    #[inline(always)]
    fn inner(&self) -> &FlexStrInner<'str, SIZE, BPAD, HPAD, HEAP, OsStr> {
        &self.0
    }
}

// ### FlexStrCore ###

impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexStrCore<'str, SIZE, BPAD, HPAD, HEAP, OsStr> for FlexOsStr<'str, SIZE, BPAD, HPAD, HEAP>
where
    HEAP: Storage<OsStr>,
{
    #[inline(always)]
    fn as_str_type(&self) -> &OsStr {
        self.inner().as_str_type()
    }
}

impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
    FlexOsStr<'str, SIZE, BPAD, HPAD, HEAP>
{
    /// Creates a wrapped static string literal. This function is equivalent to using the macro and
    /// is `const fn` so it can be used to initialize a constant at compile time with zero runtime cost.
    /// ```
    /// use flexstr::{FlexStrCore, LocalOsStr};
    ///
    /// const S: LocalOsStr = LocalOsStr::from_static("test");
    /// assert!(S.is_static());
    /// ```
    #[inline(always)]
    pub const fn from_static(s: &'static OsStr) -> Self {
        Self(FlexStrInner::from_static(s))
    }
}

// *** Type Aliases ***

/// A flexible base string type that transparently wraps a string literal, inline string, or a custom `HEAP` type.
///
/// It is three machine words in size (3x usize) and can hold 22 bytes of inline string data on 64-bit platforms.
///
/// # Note
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexOsStr]
///
/// # Note 2
/// Custom concrete types need to specify a `HEAP` type with an exact size of two machine words (16 bytes
/// on 64-bit, and 8 bytes on 32-bit). Any other sized parameter will result in a runtime panic on string
/// creation.
pub type FlexOsStr3USize<'str, HEAP> =
    FlexOsStr<'str, STRING_SIZED_INLINE, PTR_SIZED_PAD, PTR_SIZED_PAD, HEAP>;

/// A flexible string type that transparently wraps a string literal, inline string, or
/// a/an [`Rc<OsStr>`](alloc::rc::Rc)
///
/// # Note
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexOsStr]
pub type LocalOsStr = FlexOsStr3USize<'static, Rc<OsStr>>;

/// A flexible string type that transparently wraps a string literal, inline string,
/// a/an [`Rc<OsStr>`](alloc::rc::Rc), or borrowed string (with appropriate lifetime)
///
/// # Note
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexOsStr]
pub type LocalOsStrRef<'str> = FlexOsStr3USize<'str, Rc<OsStr>>;

/// A flexible string type that transparently wraps a string literal, inline string, or
/// a/an [`Arc<OsStr>`](alloc::sync::Arc)
///
/// # Note
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexOsStr]
pub type SharedOsStr = FlexOsStr3USize<'static, Arc<OsStr>>;

/// A flexible string type that transparently wraps a string literal, inline string,
/// a/an [`Arc<OsStr>`](alloc::sync::Arc), or borrowed string (with appropriate lifetime)
///
/// # Note
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexOsStr]
pub type SharedOsStrRef<'str> = FlexOsStr3USize<'str, Arc<OsStr>>;

/// A flexible string type that transparently wraps a string literal, inline string, or
/// a/an [`Box<OsStr>`](alloc::boxed::Box)
///
/// # Note
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexOsStr]
///
/// # Note 2
/// This type is included for convenience for those who need wrapped [`Box<OsStr>`](alloc::boxed::Box)
/// support. Those who do not have this special use case are encouraged to use `Local` or `Shared`
/// variants for much better clone performance (without copy or additional allocation)
pub type BoxedOsStr = FlexOsStr3USize<'static, Box<OsStr>>;

/// A flexible string type that transparently wraps a string literal, inline string,
/// a/an [`Box<OsStr>`](alloc::boxed::Box), or borrowed string (with appropriate lifetime)
///
/// # Note
/// Since this is just a type alias for a generic type, full documentation can be found here: [FlexOsStr]
///
/// # Note 2
/// This type is included for convenience for those who need wrapped [`Box<OsStr>`](alloc::boxed::Box)
/// support. Those who do not have this special use case are encouraged to use `Local` or `Shared`
/// variants for much better clone performance (without copy or additional allocation)
pub type BoxedOsStrRef<'str> = FlexOsStr3USize<'str, Box<OsStr>>;