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
    /// Extracts a string slice containing the entire [FlexStr] in the final string type
    /// ```
    /// use flexstr::{FlexStrCore, LocalStr};
    ///
    /// let s = LocalStr::from_ref("abc");
    /// assert_eq!(s.as_str_type(), "abc");
    /// ```
    fn as_str_type(&self) -> &STR;

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

    /// Returns true if this is a wrapped string using borrowed storage
    #[inline(always)]
    fn is_borrow(&self) -> bool {
        self.inner().is_borrow()
    }
}
