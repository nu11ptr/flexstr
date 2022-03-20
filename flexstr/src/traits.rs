use alloc::string::String;
use core::mem::ManuallyDrop;
use core::ops::Deref;

use crate::{
    FlexStrWrapper, HeapStr, LocalStr, SharedStr, StorageType, PTR_SIZED_PAD, STRING_SIZED_INLINE,
};

// *** Repeat custom trait ***

/// Trait that can repeat a given `LocalStr` "n" times efficiently
pub trait Repeat<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> {
    /// Repeats a given `LocalStr` "n" times efficiently and returns a new `LocalStr`
    fn repeat_n(&self, n: usize) -> FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>;
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> Repeat<SIZE, PAD1, PAD2, HEAP>
    for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: Deref<Target = str> + for<'a> From<&'a str>,
{
    /// ```
    /// use flexstr::{local_str, IntoLocalStr, Repeat};
    ///
    /// let s = local_str!("a").repeat_n(10);
    /// assert!(s.is_inline());
    /// assert_eq!(s, "a".repeat(10));
    /// ```
    #[inline]
    fn repeat_n(&self, n: usize) -> FlexStrWrapper<SIZE, PAD1, PAD2, HEAP> {
        str::repeat_n(self, n)
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> Repeat<SIZE, PAD1, PAD2, HEAP>
    for str
where
    HEAP: for<'a> From<&'a str>,
{
    #[inline]
    fn repeat_n(&self, n: usize) -> FlexStrWrapper<SIZE, PAD1, PAD2, HEAP> {
        let cap = self.len() * n;
        let mut buffer = buffer_new!(SIZE);
        let mut builder = builder_new!(buffer, cap);

        for _ in 0..n {
            builder.str_write(self);
        }

        builder_into!(builder, buffer)
    }
}

// *** ToCase custom trait ***

/// Trait that provides uppercase/lowercase conversion functions for `LocalStr`
pub trait ToCase<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> {
    /// Converts string to uppercase and returns a `LocalStr`
    fn to_upper(&self) -> FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>;

    /// Converts string to lowercase and returns a `LocalStr`
    fn to_lower(&self) -> FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>;

    /// Converts string to ASCII uppercase and returns a `LocalStr`
    fn to_ascii_upper(&self) -> FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>;

    /// Converts string to ASCII lowercase and returns a `LocalStr`
    fn to_ascii_lower(&self) -> FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>;
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> ToCase<SIZE, PAD1, PAD2, HEAP>
    for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: Deref<Target = str> + for<'a> From<&'a str>,
{
    /// ```
    /// use flexstr::{local_str, LocalStr, ToCase};
    ///
    /// let a: LocalStr = local_str!("test").to_upper();
    /// assert_eq!(a, "TEST");
    /// ```
    #[inline]
    fn to_upper(&self) -> FlexStrWrapper<SIZE, PAD1, PAD2, HEAP> {
        str::to_upper(self)
    }

    /// ```
    /// use flexstr::{local_str, LocalStr, IntoLocalStr, ToCase};
    ///
    /// let a: LocalStr = local_str!("TEST").to_lower();
    /// assert_eq!(a, "test");
    /// ```
    #[inline]
    fn to_lower(&self) -> FlexStrWrapper<SIZE, PAD1, PAD2, HEAP> {
        str::to_lower(self)
    }

    /// ```
    /// use flexstr::{local_str, LocalStr, IntoLocalStr, ToCase};
    ///
    /// let a: LocalStr = local_str!("test").to_ascii_upper();
    /// assert_eq!(a, "TEST");
    /// ```
    #[inline]
    fn to_ascii_upper(&self) -> FlexStrWrapper<SIZE, PAD1, PAD2, HEAP> {
        str::to_ascii_upper(self)
    }

    /// ```
    /// use flexstr::{local_str, LocalStr, IntoLocalStr, ToCase};
    ///
    /// let a: LocalStr = local_str!("TEST").to_ascii_lower();
    /// assert_eq!(a, "test");
    /// ```
    #[inline]
    fn to_ascii_lower(&self) -> FlexStrWrapper<SIZE, PAD1, PAD2, HEAP> {
        str::to_ascii_lower(self)
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> ToCase<SIZE, PAD1, PAD2, HEAP>
    for str
where
    HEAP: for<'a> From<&'a str>,
{
    /// ```
    /// use flexstr::{LocalStr, ToCase};
    ///
    /// let a: LocalStr = "test".to_upper();
    /// assert_eq!(a, "TEST");
    /// ```
    fn to_upper(&self) -> FlexStrWrapper<SIZE, PAD1, PAD2, HEAP> {
        // We estimate capacity based on previous string, but if not ASCII this might be wrong
        let mut buffer = buffer_new!(SIZE);
        let mut builder = builder_new!(buffer, self.len());

        for ch in self.chars() {
            let upper_chars = ch.to_uppercase();
            for ch in upper_chars {
                builder.char_write(ch);
            }
        }

        builder_into!(builder, buffer)
    }

    /// ```
    /// use flexstr::{LocalStr, ToCase};
    ///
    /// let a: LocalStr = "TEST".to_lower();
    /// assert_eq!(a, "test");
    /// ```
    fn to_lower(&self) -> FlexStrWrapper<SIZE, PAD1, PAD2, HEAP> {
        // We estimate capacity based on previous string, but if not ASCII this might be wrong
        let mut buffer = buffer_new!(SIZE);
        let mut builder = builder_new!(buffer, self.len());

        for ch in self.chars() {
            let lower_chars = ch.to_lowercase();
            for ch in lower_chars {
                builder.char_write(ch);
            }
        }

        builder_into!(builder, buffer)
    }

    /// ```
    /// use flexstr::{LocalStr, ToCase};
    ///
    /// let a: LocalStr = "test".to_ascii_upper();
    /// assert_eq!(a, "TEST");
    /// ```
    fn to_ascii_upper(&self) -> FlexStrWrapper<SIZE, PAD1, PAD2, HEAP> {
        let mut buffer = buffer_new!(SIZE);
        let mut builder = builder_new!(buffer, self.len());

        for mut ch in self.chars() {
            char::make_ascii_uppercase(&mut ch);
            builder.char_write(ch);
        }

        builder_into!(builder, buffer)
    }

    /// ```
    /// use flexstr::{LocalStr, ToCase};
    ///
    /// let a: LocalStr = "TEST".to_ascii_lower();
    /// assert_eq!(a, "test");
    /// ```
    fn to_ascii_lower(&self) -> FlexStrWrapper<SIZE, PAD1, PAD2, HEAP> {
        let mut buffer = buffer_new!(SIZE);
        let mut builder = builder_new!(buffer, self.len());

        for mut ch in self.chars() {
            char::make_ascii_lowercase(&mut ch);
            builder.char_write(ch);
        }

        builder_into!(builder, buffer)
    }
}

// *** Generic `To` trait ***

/// A trait that converts the source to a `Flex<SIZE, PAD1, PAD2, HEAP>` without consuming it
/// ```
/// use flexstr::{LocalStr, ToFlex};
///
/// let a: LocalStr = "This is a heap allocated string!!!!!".to_string().to_flex();
/// assert!(a.is_heap());
/// ```
pub trait ToFlex<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> {
    /// Converts the source to a `Flex<SIZE, PAD1, PAD2, HEAP>` without consuming it
    fn to_flex(&self) -> FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>;
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, HEAP2>
    ToFlex<SIZE, PAD1, PAD2, HEAP> for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP2>
where
    HEAP: for<'a> From<&'a str>,
    HEAP2: Clone + Deref<Target = str>,
{
    /// ```
    /// use flexstr::{SharedStr, LocalStr, ToFlex};
    ///
    /// let a: SharedStr = "test".into();
    /// let b: LocalStr = a.to_flex();
    /// assert_eq!(a, b);
    /// ```
    #[inline]
    fn to_flex(&self) -> FlexStrWrapper<SIZE, PAD1, PAD2, HEAP> {
        self.clone().into_flex()
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> ToFlex<SIZE, PAD1, PAD2, HEAP>
    for str
where
    HEAP: for<'a> From<&'a str>,
{
    /// ```
    /// use flexstr::{SharedStr, LocalStr, ToFlex};
    ///
    /// // *** Don't do this - use `into_flex` on literals ***
    /// let a: SharedStr = "inlined".to_flex();
    /// assert!(a.is_inline());
    ///
    /// let b: LocalStr = "This is too long to be inlined!!!!!!".to_flex();
    /// assert!(b.is_heap())
    /// ```
    #[inline]
    fn to_flex(&self) -> FlexStrWrapper<SIZE, PAD1, PAD2, HEAP> {
        self.into()
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> ToFlex<SIZE, PAD1, PAD2, HEAP>
    for bool
where
    HEAP: for<'a> From<&'a str>,
{
    /// ```
    /// use flexstr::{LocalStr, ToFlex};
    ///
    /// let s: LocalStr = false.to_flex();
    /// assert!(s.is_static());
    /// assert_eq!(s, "false");
    /// ```
    #[inline]
    fn to_flex(&self) -> FlexStrWrapper<SIZE, PAD1, PAD2, HEAP> {
        FlexStrWrapper::from_static(if *self { "true" } else { "false" })
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> ToFlex<SIZE, PAD1, PAD2, HEAP>
    for char
where
    HEAP: for<'a> From<&'a str> + Deref<Target = str>,
{
    /// ```
    /// use flexstr::{LocalStr, ToFlex};
    ///
    /// let s: LocalStr = '☺'.to_flex();
    /// assert!(s.is_inline());
    /// assert_eq!(s, "☺");
    /// ```
    #[inline]
    fn to_flex(&self) -> FlexStrWrapper<SIZE, PAD1, PAD2, HEAP> {
        (*self).into()
    }
}

#[cfg(feature = "int_convert")]
macro_rules! impl_int_flex {
    ($($type:ty),+) => {
        $(impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> ToFlex<SIZE, PAD1, PAD2, HEAP> for $type
        where
            HEAP: for<'a> From<&'a str>,
        {
            /// ```
            /// use flexstr::{LocalStr, ToFlex};
            ///
            #[doc = concat!("let s: LocalStr = 123", stringify!($type), ".to_flex();")]
            /// assert!(s.is_inline());
            /// assert_eq!(s, "123");
            /// ```
            #[inline]
            fn to_flex(&self) -> FlexStrWrapper<SIZE, PAD1, PAD2, HEAP> {
                let mut buffer = itoa::Buffer::new();
                buffer.format(*self).to_flex()
            }
        })+
    };
}

#[cfg(feature = "int_convert")]
impl_int_flex!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize);

#[cfg(feature = "fp_convert")]
macro_rules! impl_float_flex {
    ($($type:ty),+) => {
        $(impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> ToFlex<SIZE, PAD1, PAD2, HEAP> for $type
        where
            HEAP: for<'a> From<&'a str>,
        {
            /// ```
            /// use flexstr::{LocalStr, ToFlex};
            ///
            #[doc = concat!("let s: LocalStr = 123.456", stringify!($type), ".to_flex();")]
            /// assert!(s.is_inline());
            /// assert_eq!(s, "123.456");
            /// ```
            #[inline]
            fn to_flex(&self) -> FlexStrWrapper<SIZE, PAD1, PAD2, HEAP> {
                let mut buffer = ryu::Buffer::new();
                buffer.format(*self).to_flex()
            }
        })+
    };
}

#[cfg(feature = "fp_convert")]
impl_float_flex!(f32, f64);

// *** Generic `Into` Custom Traits ***

/// A trait that converts the source to a `FlexStr<SIZE, PAD1, PAD2, HEAP>` while consuming the original
/// ```
/// use flexstr::{local_str, LocalStr, IntoFlex};
///
/// let a: LocalStr = local_str!("This is a wrapped static string literal no matter how long it is!!!!!");
/// assert!(a.is_static());
/// ```
pub trait IntoFlex<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> {
    /// Converts the source to a `FlexStr<SIZE, PAD1, PAD2, HEAP>` while consuming the original
    fn into_flex(self) -> FlexStrWrapper<SIZE, PAD1, PAD2, HEAP>;
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, HEAP2>
    IntoFlex<SIZE, PAD1, PAD2, HEAP> for FlexStrWrapper<SIZE, PAD1, PAD2, HEAP2>
where
    HEAP: for<'a> From<&'a str>,
    HEAP2: Deref<Target = str>,
{
    /// ```
    /// use flexstr::{shared_str, SharedStr, LocalStr, IntoFlex};
    ///
    /// const a: SharedStr = shared_str!("This can be just wrapped as a static string literal!");
    /// assert!(a.is_static());
    /// let b: LocalStr = a.clone().into_flex();
    /// assert!(b.is_static());
    /// assert_eq!(a, b);
    ///
    /// let c: LocalStr = "Inlined!".to_string().into_flex();
    /// assert!(c.is_inline());
    /// let d: SharedStr = c.clone().into_flex();
    /// assert!(d.is_inline());
    /// assert_eq!(c, d);
    ///
    /// let e: LocalStr = "This will be a wrapped heap allocated `String`!".to_string().into_flex();
    /// assert!(e.is_heap());
    /// let f: SharedStr = e.clone().into_flex();
    /// assert!(f.is_heap());
    /// assert_eq!(e, f);
    /// ```
    #[inline]
    fn into_flex(self) -> FlexStrWrapper<SIZE, PAD1, PAD2, HEAP> {
        // SAFETY: Marker check is aligned to correct accessed field
        unsafe {
            match self.static_str.marker {
                StorageType::Static => FlexStrWrapper {
                    static_str: self.static_str,
                },
                StorageType::Inline => FlexStrWrapper {
                    inline_str: self.inline_str,
                },
                StorageType::Heap => {
                    // TODO: Any more efficient way to do this?
                    // Would like to use `from_raw` and `into_raw`, but need to ensure
                    // exclusive ownership for this to be safe. For `Rc` that might be possible,
                    // but `Arc` could be multi-threaded so needs to be atomic
                    FlexStrWrapper {
                        heap_str: ManuallyDrop::new(HeapStr::from_heap(HEAP::from(
                            &self.heap_str.heap,
                        ))),
                    }
                }
            }
        }
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> IntoFlex<SIZE, PAD1, PAD2, HEAP>
    for String
where
    HEAP: for<'a> From<&'a str>,
{
    /// ```
    /// use flexstr::{SharedStr, IntoFlex};
    ///
    /// let a = "Inlined!".to_string();
    /// let b: SharedStr = a.clone().into_flex();
    /// assert!(b.is_inline());
    /// assert_eq!(b, a);
    /// ```
    #[inline]
    fn into_flex(self) -> FlexStrWrapper<SIZE, PAD1, PAD2, HEAP> {
        <FlexStrWrapper<SIZE, PAD1, PAD2, HEAP> as From<&str>>::from(&self)
    }
}

// *** FlexStr `To` Traits ***

/// A trait that converts the source to a `FlexStr` without consuming it
/// ```
/// use flexstr::ToLocalStr;
///
/// let a = "This is a heap allocated string!!!!!".to_string().to_local_str();
/// assert!(a.is_heap());
/// ```
pub trait ToLocalStr {
    /// Converts the source to a `FlexStr` without consuming it
    fn to_local_str(&self) -> LocalStr;
}

impl<HEAP> ToLocalStr for FlexStrWrapper<STRING_SIZED_INLINE, PTR_SIZED_PAD, PTR_SIZED_PAD, HEAP>
where
    HEAP: Clone + Deref<Target = str>,
{
    /// ```
    /// use flexstr::{SharedStr, ToLocalStr};
    ///
    /// let a: SharedStr = "test".into();
    /// let b = a.to_local_str();
    /// assert_eq!(a, b);
    /// ```
    #[inline]
    fn to_local_str(&self) -> LocalStr {
        self.to_flex()
    }
}

impl ToLocalStr for str {
    /// ```
    /// use flexstr::ToLocalStr;
    ///
    /// // Don't use for literals - use `into_local_str` instead
    /// let a = "test".to_local_str();
    /// assert!(a.is_inline());
    /// ```
    #[inline]
    fn to_local_str(&self) -> LocalStr {
        self.to_flex()
    }
}

impl ToLocalStr for bool {
    /// ```
    /// use flexstr::ToLocalStr;
    ///
    /// let s = false.to_local_str();
    /// assert!(s.is_static());
    /// assert_eq!(s, "false");
    /// ```
    #[inline]
    fn to_local_str(&self) -> LocalStr {
        self.to_flex()
    }
}

impl ToLocalStr for char {
    /// ```
    /// use flexstr::ToLocalStr;
    ///
    /// let s = '☺'.to_local_str();
    /// assert!(s.is_inline());
    /// assert_eq!(s, "☺");
    /// ```
    #[inline]
    fn to_local_str(&self) -> LocalStr {
        self.to_flex()
    }
}

#[cfg(feature = "int_convert")]
macro_rules! impl_int_local_str {
    ($($type:ty),+) => {
        $(impl ToLocalStr for $type
        {
            /// ```
            /// use flexstr::ToLocalStr;
            ///
            #[doc = concat!("let s = 123", stringify!($type), ".to_local_str();")]
            /// assert!(s.is_inline());
            /// assert_eq!(s, "123");
            /// ```
            #[inline]
            fn to_local_str(&self) -> LocalStr {
                self.to_flex()
            }
        })+
    };
}

#[cfg(feature = "int_convert")]
impl_int_local_str!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize);

#[cfg(feature = "fp_convert")]
macro_rules! impl_float_local_str {
    ($($type:ty),+) => {
        $(impl ToLocalStr for $type
        {
            /// ```
            /// use flexstr::ToLocalStr;
            ///
            #[doc = concat!("let s = 123.456", stringify!($type), ".to_local_str();")]
            /// assert!(s.is_inline());
            /// assert_eq!(s, "123.456");
            /// ```
            #[inline]
            fn to_local_str(&self) -> LocalStr {
                self.to_flex()
            }
        })+
    };
}

#[cfg(feature = "fp_convert")]
impl_float_local_str!(f32, f64);

// *** SharedStr `To` Traits ***

/// A trait that converts the source to an `SharedStr` without consuming it
/// ```
/// use flexstr::ToSharedStr;
///
/// let a = "This is a heap allocated string!!!!!".to_shared_str();
/// assert!(a.is_heap());
/// ```
pub trait ToSharedStr {
    /// Converts the source to a `SharedStr` without consuming it
    fn to_shared_str(&self) -> SharedStr;
}

impl<HEAP> ToSharedStr for FlexStrWrapper<STRING_SIZED_INLINE, PTR_SIZED_PAD, PTR_SIZED_PAD, HEAP>
where
    HEAP: Clone + Deref<Target = str>,
{
    /// ```
    /// use flexstr::{LocalStr, ToSharedStr};
    ///
    /// let a: LocalStr = "test".into();
    /// let b = a.to_shared_str();
    /// assert_eq!(a, b);
    /// ```
    #[inline]
    fn to_shared_str(&self) -> SharedStr {
        self.to_flex()
    }
}

impl ToSharedStr for str {
    /// ```
    /// use flexstr::ToSharedStr;
    ///
    /// // Don't use for literals - use `into_local_str` instead
    /// let a = "test".to_shared_str();
    /// assert!(a.is_inline());
    /// ```
    #[inline]
    fn to_shared_str(&self) -> SharedStr {
        self.to_flex()
    }
}

impl ToSharedStr for bool {
    /// ```
    /// use flexstr::ToSharedStr;
    ///
    /// let s = false.to_shared_str();
    /// assert!(s.is_static());
    /// assert_eq!(s, "false");
    /// ```
    #[inline]
    fn to_shared_str(&self) -> SharedStr {
        self.to_flex()
    }
}

impl ToSharedStr for char {
    /// ```
    /// use flexstr::ToSharedStr;
    ///
    /// let s = '☺'.to_shared_str();
    /// assert!(s.is_inline());
    /// assert_eq!(s, "☺");
    /// ```
    #[inline]
    fn to_shared_str(&self) -> SharedStr {
        self.to_flex()
    }
}

#[cfg(feature = "int_convert")]
macro_rules! impl_int_shared_str {
    ($($type:ty),+) => {
        $(impl ToSharedStr for $type
        {
            /// ```
            /// use flexstr::ToSharedStr;
            ///
            #[doc = concat!("let s = 123", stringify!($type), ".to_shared_str();")]
            /// assert!(s.is_inline());
            /// assert_eq!(s, "123");
            /// ```
            #[inline]
            fn to_shared_str(&self) -> SharedStr {
                self.to_flex()
            }
        })+
    };
}

#[cfg(feature = "int_convert")]
impl_int_shared_str!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize);

#[cfg(feature = "fp_convert")]
macro_rules! impl_float_shared_str {
    ($($type:ty),+) => {
        $(impl ToSharedStr for $type
        {
            /// ```
            /// use flexstr::ToSharedStr;
            ///
            #[doc = concat!("let s = 123.456", stringify!($type), ".to_shared_str();")]
            /// assert!(s.is_inline());
            /// assert_eq!(s, "123.456");
            /// ```
            #[inline]
            fn to_shared_str(&self) -> SharedStr {
                self.to_flex()
            }
        })+
    };
}

#[cfg(feature = "fp_convert")]
impl_float_shared_str!(f32, f64);

// *** FlexStr `Into` Traits ***

/// A trait that converts the source to a `FlexStr` while consuming the original
/// ```
/// use flexstr::local_str;
///
/// let a = local_str!("This is a wrapped static string literal no matter how long it is!!!!!");
/// assert!(a.is_static());
/// ```
pub trait IntoLocalStr {
    /// Converts the source to a `FlexStr` while consuming the original
    fn into_local_str(self) -> LocalStr;
}

impl<HEAP> IntoLocalStr for FlexStrWrapper<STRING_SIZED_INLINE, PTR_SIZED_PAD, PTR_SIZED_PAD, HEAP>
where
    HEAP: Deref<Target = str>,
{
    /// ```
    /// use flexstr::{shared_str, SharedStr, IntoFlex, IntoLocalStr};
    ///
    /// let a: SharedStr = shared_str!("This can be just wrapped as a static string literal!");
    /// assert!(a.is_static());
    /// let b = a.clone().into_local_str();
    /// assert!(b.is_static());
    /// assert_eq!(a, b);
    /// ```
    #[inline]
    fn into_local_str(self) -> LocalStr {
        self.into_flex()
    }
}

impl IntoLocalStr for String {
    /// ```
    /// use flexstr::IntoLocalStr;
    ///
    /// let a = "This is a heap allocated string since it is a `String`".to_string().into_local_str();
    /// assert!(a.is_heap());
    /// ```
    #[inline]
    fn into_local_str(self) -> LocalStr {
        self.into_flex()
    }
}

// *** SharedStr `Into` Traits ***

/// A trait that converts the source to a `SharedStr` while consuming the original
/// ```
/// use flexstr::shared_str;
///
/// let a = shared_str!("This is a wrapped static string literal no matter how long it is!!!!!");
/// assert!(a.is_static());
/// ```
pub trait IntoSharedStr {
    /// Converts the source to an `SharedStr` while consuming the original
    fn into_shared_str(self) -> SharedStr;
}

impl<HEAP> IntoSharedStr for FlexStrWrapper<STRING_SIZED_INLINE, PTR_SIZED_PAD, PTR_SIZED_PAD, HEAP>
where
    HEAP: Deref<Target = str>,
{
    /// ```
    /// use flexstr::{SharedStr, IntoSharedStr, shared_str};
    ///
    /// let a: SharedStr = shared_str!("This can be just wrapped as a static string literal!");
    /// assert!(a.is_static());
    /// let b = a.clone().into_shared_str();
    /// assert!(b.is_static());
    /// assert_eq!(a, b);
    /// ```
    #[inline]
    fn into_shared_str(self) -> SharedStr {
        self.into_flex()
    }
}

impl IntoSharedStr for String {
    /// ```
    /// use flexstr::IntoSharedStr;
    ///
    /// let a = "This is a heap allocated string since it is a `String`".to_string().into_shared_str();
    /// assert!(a.is_heap());
    /// ```
    #[inline]
    fn into_shared_str(self) -> SharedStr {
        self.into_flex()
    }
}
