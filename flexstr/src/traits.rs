use alloc::string::String;
use core::mem::ManuallyDrop;
use core::ops::Deref;

use crate::{AFlexStr, Flex, FlexMarker, FlexStr, HeapStr, PTR_SIZED_PAD, STRING_SIZED_INLINE};

// *** Repeat custom trait ***

/// Trait that can repeat a given `FlexStr` "n" times efficiently
pub trait Repeat<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> {
    /// Repeats a given `FlexStr` "n" times efficiently and returns a new `FlexStr`
    fn repeat_n(&self, n: usize) -> Flex<SIZE, PAD1, PAD2, HEAP>;
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> Repeat<SIZE, PAD1, PAD2, HEAP>
    for Flex<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: Deref<Target = str> + for<'a> From<&'a str>,
{
    /// ```
    /// use flexstr::{flex_str, IntoFlexStr, Repeat};
    ///
    /// let s = flex_str!("a").repeat_n(10);
    /// assert!(s.is_inline());
    /// assert_eq!(s, "a".repeat(10));
    /// ```
    #[inline]
    fn repeat_n(&self, n: usize) -> Flex<SIZE, PAD1, PAD2, HEAP> {
        str::repeat_n(self, n)
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> Repeat<SIZE, PAD1, PAD2, HEAP>
    for str
where
    HEAP: for<'a> From<&'a str>,
{
    #[inline]
    fn repeat_n(&self, n: usize) -> Flex<SIZE, PAD1, PAD2, HEAP> {
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

/// Trait that provides uppercase/lowercase conversion functions for `FlexStr`
pub trait ToCase<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> {
    /// Converts string to uppercase and returns a `FlexStr`
    fn to_upper(&self) -> Flex<SIZE, PAD1, PAD2, HEAP>;

    /// Converts string to lowercase and returns a `FlexStr`
    fn to_lower(&self) -> Flex<SIZE, PAD1, PAD2, HEAP>;

    /// Converts string to ASCII uppercase and returns a `FlexStr`
    fn to_ascii_upper(&self) -> Flex<SIZE, PAD1, PAD2, HEAP>;

    /// Converts string to ASCII lowercase and returns a `FlexStr`
    fn to_ascii_lower(&self) -> Flex<SIZE, PAD1, PAD2, HEAP>;
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> ToCase<SIZE, PAD1, PAD2, HEAP>
    for Flex<SIZE, PAD1, PAD2, HEAP>
where
    HEAP: Deref<Target = str> + for<'a> From<&'a str>,
{
    /// ```
    /// use flexstr::{flex_str, FlexStr, ToCase};
    ///
    /// let a: FlexStr = flex_str!("test").to_upper();
    /// assert_eq!(a, "TEST");
    /// ```
    #[inline]
    fn to_upper(&self) -> Flex<SIZE, PAD1, PAD2, HEAP> {
        str::to_upper(self)
    }

    /// ```
    /// use flexstr::{flex_str, FlexStr, IntoFlexStr, ToCase};
    ///
    /// let a: FlexStr = flex_str!("TEST").to_lower();
    /// assert_eq!(a, "test");
    /// ```
    #[inline]
    fn to_lower(&self) -> Flex<SIZE, PAD1, PAD2, HEAP> {
        str::to_lower(self)
    }

    /// ```
    /// use flexstr::{flex_str, FlexStr, IntoFlexStr, ToCase};
    ///
    /// let a: FlexStr = flex_str!("test").to_ascii_upper();
    /// assert_eq!(a, "TEST");
    /// ```
    #[inline]
    fn to_ascii_upper(&self) -> Flex<SIZE, PAD1, PAD2, HEAP> {
        str::to_ascii_upper(self)
    }

    /// ```
    /// use flexstr::{flex_str, FlexStr, IntoFlexStr, ToCase};
    ///
    /// let a: FlexStr = flex_str!("TEST").to_ascii_lower();
    /// assert_eq!(a, "test");
    /// ```
    #[inline]
    fn to_ascii_lower(&self) -> Flex<SIZE, PAD1, PAD2, HEAP> {
        str::to_ascii_lower(self)
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> ToCase<SIZE, PAD1, PAD2, HEAP>
    for str
where
    HEAP: for<'a> From<&'a str>,
{
    /// ```
    /// use flexstr::{FlexStr, ToCase};
    ///
    /// let a: FlexStr = "test".to_upper();
    /// assert_eq!(a, "TEST");
    /// ```
    fn to_upper(&self) -> Flex<SIZE, PAD1, PAD2, HEAP> {
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
    /// use flexstr::{FlexStr, ToCase};
    ///
    /// let a: FlexStr = "TEST".to_lower();
    /// assert_eq!(a, "test");
    /// ```
    fn to_lower(&self) -> Flex<SIZE, PAD1, PAD2, HEAP> {
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
    /// use flexstr::{FlexStr, ToCase};
    ///
    /// let a: FlexStr = "test".to_ascii_upper();
    /// assert_eq!(a, "TEST");
    /// ```
    fn to_ascii_upper(&self) -> Flex<SIZE, PAD1, PAD2, HEAP> {
        let mut buffer = buffer_new!(SIZE);
        let mut builder = builder_new!(buffer, self.len());

        for mut ch in self.chars() {
            char::make_ascii_uppercase(&mut ch);
            builder.char_write(ch);
        }

        builder_into!(builder, buffer)
    }

    /// ```
    /// use flexstr::{FlexStr, ToCase};
    ///
    /// let a: FlexStr = "TEST".to_ascii_lower();
    /// assert_eq!(a, "test");
    /// ```
    fn to_ascii_lower(&self) -> Flex<SIZE, PAD1, PAD2, HEAP> {
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
/// use flexstr::{FlexStr, ToFlex};
///
/// let a: FlexStr = "This is a heap allocated string!!!!!".to_string().to_flex();
/// assert!(a.is_heap());
/// ```
pub trait ToFlex<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> {
    /// Converts the source to a `Flex<SIZE, PAD1, PAD2, HEAP>` without consuming it
    fn to_flex(&self) -> Flex<SIZE, PAD1, PAD2, HEAP>;
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, HEAP2>
    ToFlex<SIZE, PAD1, PAD2, HEAP> for Flex<SIZE, PAD1, PAD2, HEAP2>
where
    HEAP: for<'a> From<&'a str>,
    HEAP2: Clone + Deref<Target = str>,
{
    /// ```
    /// use flexstr::{AFlexStr, FlexStr, ToFlex};
    ///
    /// let a: AFlexStr = "test".into();
    /// let b: FlexStr = a.to_flex();
    /// assert_eq!(a, b);
    /// ```
    #[inline]
    fn to_flex(&self) -> Flex<SIZE, PAD1, PAD2, HEAP> {
        self.clone().into_flex()
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> ToFlex<SIZE, PAD1, PAD2, HEAP>
    for str
where
    HEAP: for<'a> From<&'a str>,
{
    /// ```
    /// use flexstr::{AFlexStr, FlexStr, ToFlex};
    ///
    /// // *** Don't do this - use `into_flex` on literals ***
    /// let a: AFlexStr = "inlined".to_flex();
    /// assert!(a.is_inline());
    ///
    /// let b: FlexStr = "This is too long to be inlined!!!!!!".to_flex();
    /// assert!(b.is_heap())
    /// ```
    #[inline]
    fn to_flex(&self) -> Flex<SIZE, PAD1, PAD2, HEAP> {
        self.into()
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> ToFlex<SIZE, PAD1, PAD2, HEAP>
    for bool
where
    HEAP: for<'a> From<&'a str>,
{
    /// ```
    /// use flexstr::{FlexStr, ToFlex};
    ///
    /// let s: FlexStr = false.to_flex();
    /// assert!(s.is_static());
    /// assert_eq!(s, "false");
    /// ```
    #[inline]
    fn to_flex(&self) -> Flex<SIZE, PAD1, PAD2, HEAP> {
        Flex::from_static(if *self { "true" } else { "false" })
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> ToFlex<SIZE, PAD1, PAD2, HEAP>
    for char
where
    HEAP: for<'a> From<&'a str> + Deref<Target = str>,
{
    /// ```
    /// use flexstr::{FlexStr, ToFlex};
    ///
    /// let s: FlexStr = '☺'.to_flex();
    /// assert!(s.is_inline());
    /// assert_eq!(s, "☺");
    /// ```
    #[inline]
    fn to_flex(&self) -> Flex<SIZE, PAD1, PAD2, HEAP> {
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
            /// use flexstr::{FlexStr, ToFlex};
            ///
            #[doc = concat!("let s: FlexStr = 123", stringify!($type), ".to_flex();")]
            /// assert!(s.is_inline());
            /// assert_eq!(s, "123");
            /// ```
            #[inline]
            fn to_flex(&self) -> Flex<SIZE, PAD1, PAD2, HEAP> {
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
            /// use flexstr::{FlexStr, ToFlex};
            ///
            #[doc = concat!("let s: FlexStr = 123.456", stringify!($type), ".to_flex();")]
            /// assert!(s.is_inline());
            /// assert_eq!(s, "123.456");
            /// ```
            #[inline]
            fn to_flex(&self) -> Flex<SIZE, PAD1, PAD2, HEAP> {
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
/// use flexstr::{flex_str, FlexStr, IntoFlex};
///
/// let a: FlexStr = flex_str!("This is a wrapped static string literal no matter how long it is!!!!!");
/// assert!(a.is_static());
/// ```
pub trait IntoFlex<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> {
    /// Converts the source to a `FlexStr<SIZE, PAD1, PAD2, HEAP>` while consuming the original
    fn into_flex(self) -> Flex<SIZE, PAD1, PAD2, HEAP>;
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, HEAP2>
    IntoFlex<SIZE, PAD1, PAD2, HEAP> for Flex<SIZE, PAD1, PAD2, HEAP2>
where
    HEAP: for<'a> From<&'a str>,
    HEAP2: Deref<Target = str>,
{
    /// ```
    /// use flexstr::{a_flex_str, AFlexStr, FlexStr, IntoFlex};
    ///
    /// const a: AFlexStr = a_flex_str!("This can be just wrapped as a static string literal!");
    /// assert!(a.is_static());
    /// let b: FlexStr = a.clone().into_flex();
    /// assert!(b.is_static());
    /// assert_eq!(a, b);
    ///
    /// let c: FlexStr = "Inlined!".to_string().into_flex();
    /// assert!(c.is_inline());
    /// let d: AFlexStr = c.clone().into_flex();
    /// assert!(d.is_inline());
    /// assert_eq!(c, d);
    ///
    /// let e: FlexStr = "This will be a wrapped heap allocated `String`!".to_string().into_flex();
    /// assert!(e.is_heap());
    /// let f: AFlexStr = e.clone().into_flex();
    /// assert!(f.is_heap());
    /// assert_eq!(e, f);
    /// ```
    #[inline]
    fn into_flex(self) -> Flex<SIZE, PAD1, PAD2, HEAP> {
        // SAFETY: Marker check is aligned to correct accessed field
        unsafe {
            match self.static_str.marker {
                FlexMarker::Static => Flex {
                    static_str: self.static_str,
                },
                FlexMarker::Inline => Flex {
                    inline_str: self.inline_str,
                },
                FlexMarker::Heap => {
                    // TODO: Any more efficient way to do this?
                    // Would like to use `from_raw` and `into_raw`, but need to ensure
                    // exclusive ownership for this to be safe. For `Rc` that might be possible,
                    // but `Arc` could be multi-threaded so needs to be atomic
                    Flex {
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
    /// use flexstr::{AFlexStr, IntoFlex};
    ///
    /// let a = "Inlined!".to_string();
    /// let b: AFlexStr = a.clone().into_flex();
    /// assert!(b.is_inline());
    /// assert_eq!(b, a);
    /// ```
    #[inline]
    fn into_flex(self) -> Flex<SIZE, PAD1, PAD2, HEAP> {
        <Flex<SIZE, PAD1, PAD2, HEAP> as From<&str>>::from(&self)
    }
}

// *** FlexStr `To` Traits ***

/// A trait that converts the source to a `FlexStr` without consuming it
/// ```
/// use flexstr::ToFlexStr;
///
/// let a = "This is a heap allocated string!!!!!".to_string().to_flex_str();
/// assert!(a.is_heap());
/// ```
pub trait ToFlexStr {
    /// Converts the source to a `FlexStr` without consuming it
    fn to_flex_str(&self) -> FlexStr;
}

impl<HEAP> ToFlexStr for Flex<STRING_SIZED_INLINE, PTR_SIZED_PAD, PTR_SIZED_PAD, HEAP>
where
    HEAP: Clone + Deref<Target = str>,
{
    /// ```
    /// use flexstr::{AFlexStr, ToFlexStr};
    ///
    /// let a: AFlexStr = "test".into();
    /// let b = a.to_flex_str();
    /// assert_eq!(a, b);
    /// ```
    #[inline]
    fn to_flex_str(&self) -> FlexStr {
        self.to_flex()
    }
}

impl ToFlexStr for str {
    /// ```
    /// use flexstr::ToFlexStr;
    ///
    /// // Don't use for literals - use `into_flex_str` instead
    /// let a = "test".to_flex_str();
    /// assert!(a.is_inline());
    /// ```
    #[inline]
    fn to_flex_str(&self) -> FlexStr {
        self.to_flex()
    }
}

impl ToFlexStr for bool {
    /// ```
    /// use flexstr::ToFlexStr;
    ///
    /// let s = false.to_flex_str();
    /// assert!(s.is_static());
    /// assert_eq!(s, "false");
    /// ```
    #[inline]
    fn to_flex_str(&self) -> FlexStr {
        self.to_flex()
    }
}

impl ToFlexStr for char {
    /// ```
    /// use flexstr::ToFlexStr;
    ///
    /// let s = '☺'.to_flex_str();
    /// assert!(s.is_inline());
    /// assert_eq!(s, "☺");
    /// ```
    #[inline]
    fn to_flex_str(&self) -> FlexStr {
        self.to_flex()
    }
}

#[cfg(feature = "int_convert")]
macro_rules! impl_int_flex_str {
    ($($type:ty),+) => {
        $(impl ToFlexStr for $type
        {
            /// ```
            /// use flexstr::ToFlexStr;
            ///
            #[doc = concat!("let s = 123", stringify!($type), ".to_flex_str();")]
            /// assert!(s.is_inline());
            /// assert_eq!(s, "123");
            /// ```
            #[inline]
            fn to_flex_str(&self) -> FlexStr {
                self.to_flex()
            }
        })+
    };
}

#[cfg(feature = "int_convert")]
impl_int_flex_str!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize);

#[cfg(feature = "fp_convert")]
macro_rules! impl_float_flex_str {
    ($($type:ty),+) => {
        $(impl ToFlexStr for $type
        {
            /// ```
            /// use flexstr::ToFlexStr;
            ///
            #[doc = concat!("let s = 123.456", stringify!($type), ".to_flex_str();")]
            /// assert!(s.is_inline());
            /// assert_eq!(s, "123.456");
            /// ```
            #[inline]
            fn to_flex_str(&self) -> FlexStr {
                self.to_flex()
            }
        })+
    };
}

#[cfg(feature = "fp_convert")]
impl_float_flex_str!(f32, f64);

// *** AFlexStr `To` Traits ***

/// A trait that converts the source to an `AFlexStr` without consuming it
/// ```
/// use flexstr::ToAFlexStr;
///
/// let a = "This is a heap allocated string!!!!!".to_a_flex_str();
/// assert!(a.is_heap());
/// ```
pub trait ToAFlexStr {
    /// Converts the source to a `AFlexStr` without consuming it
    fn to_a_flex_str(&self) -> AFlexStr;
}

impl<HEAP> ToAFlexStr for Flex<STRING_SIZED_INLINE, PTR_SIZED_PAD, PTR_SIZED_PAD, HEAP>
where
    HEAP: Clone + Deref<Target = str>,
{
    /// ```
    /// use flexstr::{FlexStr, ToAFlexStr};
    ///
    /// let a: FlexStr = "test".into();
    /// let b = a.to_a_flex_str();
    /// assert_eq!(a, b);
    /// ```
    #[inline]
    fn to_a_flex_str(&self) -> AFlexStr {
        self.to_flex()
    }
}

impl ToAFlexStr for str {
    /// ```
    /// use flexstr::ToAFlexStr;
    ///
    /// // Don't use for literals - use `into_flex_str` instead
    /// let a = "test".to_a_flex_str();
    /// assert!(a.is_inline());
    /// ```
    #[inline]
    fn to_a_flex_str(&self) -> AFlexStr {
        self.to_flex()
    }
}

impl ToAFlexStr for bool {
    /// ```
    /// use flexstr::ToAFlexStr;
    ///
    /// let s = false.to_a_flex_str();
    /// assert!(s.is_static());
    /// assert_eq!(s, "false");
    /// ```
    #[inline]
    fn to_a_flex_str(&self) -> AFlexStr {
        self.to_flex()
    }
}

impl ToAFlexStr for char {
    /// ```
    /// use flexstr::ToAFlexStr;
    ///
    /// let s = '☺'.to_a_flex_str();
    /// assert!(s.is_inline());
    /// assert_eq!(s, "☺");
    /// ```
    #[inline]
    fn to_a_flex_str(&self) -> AFlexStr {
        self.to_flex()
    }
}

#[cfg(feature = "int_convert")]
macro_rules! impl_int_a_flex_str {
    ($($type:ty),+) => {
        $(impl ToAFlexStr for $type
        {
            /// ```
            /// use flexstr::ToAFlexStr;
            ///
            #[doc = concat!("let s = 123", stringify!($type), ".to_a_flex_str();")]
            /// assert!(s.is_inline());
            /// assert_eq!(s, "123");
            /// ```
            #[inline]
            fn to_a_flex_str(&self) -> AFlexStr {
                self.to_flex()
            }
        })+
    };
}

#[cfg(feature = "int_convert")]
impl_int_a_flex_str!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize);

#[cfg(feature = "fp_convert")]
macro_rules! impl_float_a_flex_str {
    ($($type:ty),+) => {
        $(impl ToAFlexStr for $type
        {
            /// ```
            /// use flexstr::ToAFlexStr;
            ///
            #[doc = concat!("let s = 123.456", stringify!($type), ".to_a_flex_str();")]
            /// assert!(s.is_inline());
            /// assert_eq!(s, "123.456");
            /// ```
            #[inline]
            fn to_a_flex_str(&self) -> AFlexStr {
                self.to_flex()
            }
        })+
    };
}

#[cfg(feature = "fp_convert")]
impl_float_a_flex_str!(f32, f64);

// *** FlexStr `Into` Traits ***

/// A trait that converts the source to a `FlexStr` while consuming the original
/// ```
/// use flexstr::flex_str;
///
/// let a = flex_str!("This is a wrapped static string literal no matter how long it is!!!!!");
/// assert!(a.is_static());
/// ```
pub trait IntoFlexStr {
    /// Converts the source to a `FlexStr` while consuming the original
    fn into_flex_str(self) -> FlexStr;
}

impl<HEAP> IntoFlexStr for Flex<STRING_SIZED_INLINE, PTR_SIZED_PAD, PTR_SIZED_PAD, HEAP>
where
    HEAP: Deref<Target = str>,
{
    /// ```
    /// use flexstr::{a_flex_str, AFlexStr, IntoFlex, IntoFlexStr};
    ///
    /// let a: AFlexStr = a_flex_str!("This can be just wrapped as a static string literal!");
    /// assert!(a.is_static());
    /// let b = a.clone().into_flex_str();
    /// assert!(b.is_static());
    /// assert_eq!(a, b);
    /// ```
    #[inline]
    fn into_flex_str(self) -> FlexStr {
        self.into_flex()
    }
}

impl IntoFlexStr for String {
    /// ```
    /// use flexstr::IntoFlexStr;
    ///
    /// let a = "This is a heap allocated string since it is a `String`".to_string().into_flex_str();
    /// assert!(a.is_heap());
    /// ```
    #[inline]
    fn into_flex_str(self) -> FlexStr {
        self.into_flex()
    }
}

// *** AFlexStr `Into` Traits ***

/// A trait that converts the source to a `AFlexStr` while consuming the original
/// ```
/// use flexstr::a_flex_str;
///
/// let a = a_flex_str!("This is a wrapped static string literal no matter how long it is!!!!!");
/// assert!(a.is_static());
/// ```
pub trait IntoAFlexStr {
    /// Converts the source to an `AFlexStr` while consuming the original
    fn into_a_flex_str(self) -> AFlexStr;
}

impl<HEAP> IntoAFlexStr for Flex<STRING_SIZED_INLINE, PTR_SIZED_PAD, PTR_SIZED_PAD, HEAP>
where
    HEAP: Deref<Target = str>,
{
    /// ```
    /// use flexstr::{AFlexStr, IntoAFlexStr, a_flex_str};
    ///
    /// let a: AFlexStr = a_flex_str!("This can be just wrapped as a static string literal!");
    /// assert!(a.is_static());
    /// let b = a.clone().into_a_flex_str();
    /// assert!(b.is_static());
    /// assert_eq!(a, b);
    /// ```
    #[inline]
    fn into_a_flex_str(self) -> AFlexStr {
        self.into_flex()
    }
}

impl IntoAFlexStr for String {
    /// ```
    /// use flexstr::IntoAFlexStr;
    ///
    /// let a = "This is a heap allocated string since it is a `String`".to_string().into_a_flex_str();
    /// assert!(a.is_heap());
    /// ```
    #[inline]
    fn into_a_flex_str(self) -> AFlexStr {
        self.into_flex()
    }
}
