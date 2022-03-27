pub(crate) mod b_str;
pub(crate) mod c_str;
pub(crate) mod os_str;
pub(crate) mod raw_str;
pub(crate) mod std_str;

/// Trait used for implementing a custom inner string type ([str], [OsStr](std::ffi::OsStr), [Cstr](std::ffi::CStr), etc.)
pub trait Str {
    /// Regular (typically [Vec]-based) heap allocate string type
    type StringType;
    /// Type of the individual element of the underlying storage array
    type StoredType: Copy;
    /// Error returned when a conversion from raw type to representative type fails
    type ConvertError;

    /// Transforms a slice of the stored type into the final string type. This can't fail so it only
    /// is called when the data is already vetted to be valid
    fn from_stored_data(bytes: &[Self::StoredType]) -> &Self;

    /// Tries to transform raw data that has not yet been vetted to the final string type. If it is not
    /// possible, a [Self::ConvertError] is returned
    fn try_from_raw_data(bytes: &[u8]) -> Result<&Self, Self::ConvertError>;

    /// If self is_empty return a static empty string. If not supported by this string type, None is returned
    fn empty(&self) -> Option<&'static Self>;

    /// Returns the storage length for this particular string in bytes (not the # of chars)
    fn length(&self) -> usize;

    /// Returns a representation of the inline type as a pointer
    fn as_pointer(&self) -> *const Self::StoredType;
}

#[doc(hidden)]
#[macro_export]
macro_rules! define_flex_types {
    ($ident:literal, $type:ty) => {
        use $crate::custom::{BAD_SIZE_OR_ALIGNMENT, PTR_SIZED_PAD, STRING_SIZED_INLINE};

        paste! {
            /// A flexible base string type that transparently wraps a string literal, inline string, or a custom `HEAP` type
            ///
            /// # Note
            /// Since this is just a type alias for a generic type, full documentation can be found here: [FlexStr](crate::FlexStr)
            ///
            /// # Note 2
            /// Custom concrete types need to specify a `HEAP` type with an exact size of two machine words (16 bytes
            /// on 64-bit, and 8 bytes on 32-bit). Any other sized parameter will result in a runtime panic on string
            /// creation.
            pub type [<Flex $ident StrBase>]<HEAP> =
                [<Flex $ident Str>]<'static, STRING_SIZED_INLINE, PTR_SIZED_PAD, PTR_SIZED_PAD, HEAP>;

            /// A flexible base string type that transparently wraps a string literal, inline string, a custom `HEAP` type, or
            /// a borrowed string (with appropriate lifetime specified).
            ///
            /// # Note
            /// Since this is just a type alias for a generic type, full documentation can be found here: [FlexStr](crate::FlexStr)
            ///
            /// # Note 2
            /// Custom concrete types need to specify a `HEAP` type with an exact size of two machine words (16 bytes
            /// on 64-bit, and 8 bytes on 32-bit). Any other sized parameter will result in a runtime panic on string
            /// creation.
            pub type [<Flex $ident StrRefBase>]<'str, HEAP> =
                [<Flex $ident Str>]<'str, STRING_SIZED_INLINE, PTR_SIZED_PAD, PTR_SIZED_PAD, HEAP>;

            #[doc = concat!("A flexible string type that transparently wraps a string literal, inline string, or an [`Rc<",
            stringify!($type), ">`](std::rc::Rc)")]
            ///
            /// # Note
            #[doc = "Since this is just a type alias for a generic type, full documentation can be found here: ["
            [<Flex $ident Str>] "]"]
            pub type [<Local $ident Str>] = [<Flex $ident StrBase>]<Rc<$type>>;

            #[doc = concat!("A flexible string type that transparently wraps a string literal, inline string, or an [`Arc<",
            stringify!($type), ">`](std::sync::Arc)")]
            ///
            /// # Note
            #[doc = "Since this is just a type alias for a generic type, full documentation can be found here: ["
            [<Flex $ident Str>] "]"]
            pub type [<Shared $ident Str>] = [<Flex $ident StrBase>]<Arc<$type>>;

            #[doc = concat!("A flexible string type that transparently wraps a string literal, inline string, an [`Rc<",
            stringify!($type), ">`](std::rc::Rc), or borrowed string (with appropriate lifetime)")]
            ///
            /// # Note
            #[doc = "Since this is just a type alias for a generic type, full documentation can be found here: ["
            [<Flex $ident Str>] "]"]
            pub type [<Local $ident StrRef>]<'str> = [<Flex $ident StrRefBase>]<'str, Rc<$type>>;

            #[doc = concat!("A flexible string type that transparently wraps a string literal, inline string, an [`Arc<",
            stringify!($type), ">`](std::sync::Arc), or borrowed string (with appropriate lifetime)")]
            ///
            /// # Note
            #[doc = "Since this is just a type alias for a generic type, full documentation can be found here: ["
            [<Flex $ident Str>] "]"]
            pub type [<Shared $ident StrRef>]<'str> = [<Flex $ident StrRefBase>]<'str, Arc<$type>>;

            #[doc = concat!("A flexible string type that transparently wraps a string literal, inline string, or a [`Box<",
            stringify!($type), ">`](std::boxed::Box)")]
            ///
            /// # Note
            #[doc = concat!("This type is included for convenience for those who need wrapped [`Box<", stringify!($type),
            ">`](std::boxed::Box)")]
            #[doc = "support. Those who do not have this special use case are encouraged to use [Local" $ident "Str] or [Shared"
            $ident "Str] for much better clone performance (without copy or additional allocation)"]
            ///
            /// # Note 2
            #[doc = "Since this is just a type alias for a generic type, full documentation can be found here: ["
            [<Flex $ident Str>] "]"]
            pub type [<Boxed $ident Str>] = [<Flex $ident StrBase>]<Box<$type>>;

            #[doc = concat!("A flexible string type that transparently wraps a string literal, inline string, an [`Box<",
            stringify!($type), ">`](std::boxed::Box), or borrowed string (with appropriate lifetime)")]
            ///
            /// # Note
            #[doc = concat!("This type is included for convenience for those who need wrapped [`Box<", stringify!($type),
            ">`](std::boxed::Box)")]
            #[doc = "support. Those who do not have this special use case are encouraged to use [Local" $ident "StrRef] or [Shared"
            $ident "StrRef] for much better clone performance (without copy or additional allocation)"]
            ///
            /// # Note 2
            #[doc = "Since this is just a type alias for a generic type, full documentation can be found here: ["
            [<Flex $ident Str>] "]"]
            pub type [<Boxed $ident StrRef>]<'str> = [<Flex $ident StrRefBase>]<'str, Box<$type>>;
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_flex_str {
    ($flex_str:ident, $str:ty) => {
        /// A flexible string type that transparently wraps a string literal, inline string, a heap allocated type,
        /// or a borrowed string (with appropriate lifetime)
        ///
        /// # Note
        #[doc = concat!("It is not generally recommended to try and create direct custom concrete types of [", stringify!($flex_str), "] as it")]
        /// is complicated to calculate the correct sizes of all the generic type parameters. However, be aware
        /// that a runtime panic will be issued on creation if incorrect, so if you are able to create a string
        /// of your custom type, your parameters were of correct size/alignment.
        #[repr(transparent)]
        pub struct $flex_str<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>(
            $crate::FlexStrInner<'str, SIZE, BPAD, HPAD, HEAP, $str>,
        );

        impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
            $flex_str<'str, SIZE, BPAD, HPAD, HEAP>
        {
            // If the union variants aren't the precise right size bad things will happen - we protect against that
            const IS_VALID_SIZE: bool = Self::variant_sizes_are_valid();

            #[inline]
            const fn variant_sizes_are_valid() -> bool {
                use core::mem;

                use crate::{BorrowStr, HeapStr, InlineStr};

                mem::size_of::<HeapStr<HPAD, HEAP, $str>>() == mem::size_of::<InlineStr<SIZE, $str>>()
                    && mem::size_of::<BorrowStr<BPAD, &'static $str>>()
                        == mem::size_of::<InlineStr<SIZE, $str>>()
                    && mem::align_of::<HeapStr<HPAD, HEAP, $str>>()
                        == mem::align_of::<InlineStr<SIZE, $str>>()
                    && mem::align_of::<BorrowStr<BPAD, &'static $str>>()
                        == mem::align_of::<InlineStr<SIZE, $str>>()
            }

            /// Returns true if this is a wrapped string literal (`&'static str`)
            /// ```
            /// use flexstr::LocalStr;
            ///
            /// let s = LocalStr::from_static("test");
            /// assert!(s.is_static());
            /// ```
            #[inline(always)]
            pub fn is_static(&self) -> bool {
                self.0.is_static()
            }

            /// Returns true if this is an inlined string
            /// ```
            /// use flexstr::LocalStr;
            ///
            /// let s = LocalStr::try_inline("test").unwrap();
            /// assert!(s.is_inline());
            /// ```
            #[inline(always)]
            pub fn is_inline(&self) -> bool {
                self.0.is_inline()
            }

            /// Returns true if this is a wrapped string using heap storage
            /// ```
            /// use flexstr::LocalStr;
            ///
            /// let s = LocalStr::from_ref_heap("test");
            /// assert!(s.is_heap());
            /// ```
            #[inline(always)]
            pub fn is_heap(&self) -> bool {
                self.0.is_heap()
            }

            /// Returns true if this is a wrapped string using borrowed storage
            #[inline(always)]
            pub fn is_borrow(&self) -> bool {
                self.0.is_borrow()
            }
        }
    };
}
