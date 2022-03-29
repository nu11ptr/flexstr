use crate::inner::FlexStrInner;
use crate::{Storage, Str};

pub(crate) mod private {
    use crate::inner::FlexStrInner;
    use crate::{Storage, Str};

    pub trait FlexStrCoreInner<
        'str,
        const SIZE: usize,
        const BPAD: usize,
        const HPAD: usize,
        HEAP,
        STR,
    >
    where
        HEAP: Storage<STR>,
        STR: Str + ?Sized,
    {
        type This;

        fn wrap(inner: FlexStrInner<'str, SIZE, BPAD, HPAD, HEAP, STR>) -> Self::This;

        fn inner(&self) -> &FlexStrInner<'str, SIZE, BPAD, HPAD, HEAP, STR>;
    }
}

/// Ths trait contains most of the core methods and is implemented by all FlexStr types
pub trait FlexStrCore<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP, STR>:
    private::FlexStrCoreInner<'str, SIZE, BPAD, HPAD, HEAP, STR>
where
    HEAP: Storage<STR>,
    STR: Str + ?Sized + 'static,
{
    /// Creates a new string from a `STR` reference. If the string is empty, an empty static string
    /// is returned. If at or under the inline length limit, an inline string will be returned.
    /// Otherwise, a heap based string will be allocated and returned. This is typically used to
    /// create strings from a non-static borrowed `STR` where you don't have ownership.
    /// ```
    /// use flexstr::{FlexStrCore, LocalStr};
    ///
    /// let s = LocalStr::from_ref("");
    /// assert!(s.is_static());
    ///
    /// let s = LocalStr::from_ref("test");
    /// assert!(s.is_inline());
    ///
    /// let s = LocalStr::from_ref("This is too long to be inlined!!!!!!!");
    /// assert!(s.is_heap());
    /// ```
    #[inline(always)]
    fn from_ref(s: impl AsRef<STR>) -> Self::This {
        Self::wrap(FlexStrInner::from_ref(s))
    }

    /// Attempts to create an inlined string. Returns a new inline string on success or the original
    /// source string if it will not fit. Since the to/into/[from_ref](FlexStr::from_ref) functions
    /// will automatically inline when possible, this function is really only for special use cases.
    /// ```
    /// use flexstr::{FlexStrCore, LocalStr};
    ///
    /// let s = LocalStr::try_inline("test").unwrap();
    /// assert!(s.is_inline());
    /// ```
    #[inline(always)]
    fn try_inline<S: AsRef<STR>>(s: S) -> Result<Self::This, S> {
        FlexStrInner::try_inline(s).map(Self::wrap)
    }

    /// Force the creation of a heap allocated string. Unlike to/into/[from_ref](FlexStr::from_ref)
    /// functions, this will not attempt to inline first even if the string is a candidate for inlining.
    /// Using this is generally only recommended when using the associated [to_heap](FlexStr::to_heap)
    /// and [try_to_heap](FlexStr::try_to_heap) functions.
    /// ```
    /// use flexstr::{FlexStrCore, LocalStr};
    ///
    /// let s = LocalStr::from_ref_heap("test");
    /// assert!(s.is_heap());
    /// ```
    #[inline(always)]
    fn from_ref_heap(s: impl AsRef<STR>) -> Self::This {
        Self::wrap(FlexStrInner::from_ref_heap(s))
    }

    /// Create a new heap based string by wrapping the existing user provided heap string type (T).
    /// For [LocalStr] this will be an [Rc\<str\>](std::rc::Rc) and for [SharedStr] it will be an
    /// [Arc\<str\>](std::sync::Arc). This would typically only be used if efficient unwrapping of heap
    /// based data is needed at a later time.
    /// ```
    /// use flexstr::{FlexStrCore, LocalStr};
    ///
    /// let s = LocalStr::from_heap(b"test"[..].into());
    /// assert!(s.is_heap());
    /// ```
    #[inline(always)]
    fn from_heap(t: HEAP) -> Self::This {
        Self::wrap(FlexStrInner::from_heap(t))
    }

    /// Returns true if this is a wrapped string literal (`&'static str`)
    /// ```
    /// use flexstr::{FlexStrCore, LocalStr};
    ///
    /// let s = LocalStr::from_static("test");
    /// assert!(s.is_static());
    /// ```
    #[inline(always)]
    fn is_static(&self) -> bool {
        self.inner().is_static()
    }

    /// Returns true if this is an inlined string
    /// ```
    /// use flexstr::{FlexStrCore, LocalStr};
    ///
    /// let s = LocalStr::try_inline("test").unwrap();
    /// assert!(s.is_inline());
    /// ```
    #[inline(always)]
    fn is_inline(&self) -> bool {
        self.inner().is_inline()
    }

    /// Returns true if this is a wrapped string using heap storage
    /// ```
    /// use flexstr::{FlexStrCore, LocalStr};
    ///
    /// let s = LocalStr::from_ref_heap("test");
    /// assert!(s.is_heap());
    /// ```
    #[inline(always)]
    fn is_heap(&self) -> bool {
        self.inner().is_heap()
    }
}

/// This trait contains only borrowing related methods and is therefore implemented only by `Ref` types
pub trait FlexStrCoreRef<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP, STR>:
    private::FlexStrCoreInner<'str, SIZE, BPAD, HPAD, HEAP, STR>
where
    HEAP: Storage<STR>,
    STR: Str + ?Sized + 'static,
{
    /// Creates a wrapped borrowed string literal. The string is not copied but the reference is
    /// simply wrapped and tied to the lifetime of the source string.
    /// ```
    /// use flexstr::{FlexStrCoreRef, LocalStrRef};
    ///
    /// let abc = format!("{}", "abc");
    /// let s = LocalStrRef::from_borrow(&abc);
    /// assert!(s.is_borrow());
    /// ```
    #[inline(always)]
    fn from_borrow(s: &'str STR) -> Self::This {
        Self::wrap(FlexStrInner::from_borrow(s))
    }

    /// Returns true if this is a wrapped string using borrowed storage
    #[inline(always)]
    fn is_borrow(&self) -> bool {
        self.inner().is_borrow()
    }
}
