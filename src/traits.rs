// *** Generic `To` Custom Traits ***

use crate::{builder, AFlexStr, FlexStr, FlexStrInner};
use alloc::string::String;
use core::fmt::Write;
use core::ops::Deref;

// *** ToCase custom trait ***

/// Trait that provides uppercase/lowercase conversion functions for `FlexStr`
pub trait ToCase<T> {
    /// Converts string to uppercase and returns a `FlexStr`
    fn to_upper(&self) -> FlexStr<T>;

    /// Converts string to lowercase and returns a `FlexStr`
    fn to_lower(&self) -> FlexStr<T>;

    /// Converts string to ASCII uppercase and returns a `FlexStr`
    fn to_ascii_upper(&self) -> FlexStr<T>;

    /// Converts string to ASCII lowercase and returns a `FlexStr`
    fn to_ascii_lower(&self) -> FlexStr<T>;
}

impl<T> ToCase<T> for FlexStr<T>
where
    T: Deref<Target = str> + From<String> + for<'a> From<&'a str>,
{
    /// ```
    /// use flexstr::{FlexStr, IntoFlexStr, ToCase};
    ///
    /// let a: FlexStr = "test".into_flex_str().to_upper();
    /// assert_eq!(a, "TEST");
    /// ```
    #[inline]
    fn to_upper(&self) -> FlexStr<T> {
        str::to_upper(self)
    }

    /// ```
    /// use flexstr::{FlexStr, IntoFlexStr, ToCase};
    ///
    /// let a: FlexStr = "TEST".into_flex_str().to_lower();
    /// assert_eq!(a, "test");
    /// ```
    #[inline]
    fn to_lower(&self) -> FlexStr<T> {
        str::to_lower(self)
    }

    /// ```
    /// use flexstr::{FlexStr, IntoFlexStr, ToCase};
    ///
    /// let a: FlexStr = "test".into_flex_str().to_ascii_upper();
    /// assert_eq!(a, "TEST");
    /// ```
    #[inline]
    fn to_ascii_upper(&self) -> FlexStr<T> {
        str::to_ascii_upper(self)
    }

    /// ```
    /// use flexstr::{FlexStr, IntoFlexStr, ToCase};
    ///
    /// let a: FlexStr = "TEST".into_flex_str().to_ascii_lower();
    /// assert_eq!(a, "test");
    /// ```
    #[inline]
    fn to_ascii_lower(&self) -> FlexStr<T> {
        str::to_ascii_lower(self)
    }
}

impl<T> ToCase<T> for str
where
    T: From<String> + for<'a> From<&'a str>,
{
    /// ```
    /// use flexstr::{FlexStr, ToCase};
    ///
    /// let a: FlexStr = "test".to_upper();
    /// assert_eq!(a, "TEST");
    /// ```
    fn to_upper(&self) -> FlexStr<T> {
        // We estimate capacity based on previous string, but if not ASCII this might be wrong
        let mut builder = builder::FlexStrBuilder::with_capacity(self.len());

        for ch in self.chars() {
            let upper_chars = ch.to_uppercase();
            for ch in upper_chars {
                // SAFETY: Wraps `write_str` which always succeeds
                unsafe { builder.write_char(ch).unwrap_unchecked() }
            }
        }

        builder.into()
    }

    /// ```
    /// use flexstr::{FlexStr, ToCase};
    ///
    /// let a: FlexStr = "TEST".to_lower();
    /// assert_eq!(a, "test");
    /// ```
    fn to_lower(&self) -> FlexStr<T> {
        // We estimate capacity based on previous string, but if not ASCII this might be wrong
        let mut builder = builder::FlexStrBuilder::with_capacity(self.len());

        for ch in self.chars() {
            let lower_chars = ch.to_lowercase();
            for ch in lower_chars {
                // SAFETY: Wraps `write_str` which always succeeds
                unsafe { builder.write_char(ch).unwrap_unchecked() }
            }
        }

        builder.into()
    }

    /// ```
    /// use flexstr::{FlexStr, ToCase};
    ///
    /// let a: FlexStr = "test".to_ascii_upper();
    /// assert_eq!(a, "TEST");
    /// ```
    fn to_ascii_upper(&self) -> FlexStr<T> {
        let mut builder = builder::FlexStrBuilder::with_capacity(self.len());

        for mut ch in self.chars() {
            char::make_ascii_uppercase(&mut ch);
            // SAFETY: Wraps `write_str` which always succeeds
            unsafe { builder.write_char(ch).unwrap_unchecked() }
        }

        builder.into()
    }

    /// ```
    /// use flexstr::{FlexStr, ToCase};
    ///
    /// let a: FlexStr = "TEST".to_ascii_lower();
    /// assert_eq!(a, "test");
    /// ```
    fn to_ascii_lower(&self) -> FlexStr<T> {
        let mut builder = builder::FlexStrBuilder::with_capacity(self.len());

        for mut ch in self.chars() {
            char::make_ascii_lowercase(&mut ch);
            // SAFETY: Wraps `write_str` which always succeeds
            unsafe { builder.write_char(ch).unwrap_unchecked() }
        }

        builder.into()
    }
}

// *** Generic `To` trait ***

/// A trait that converts the source to a `FlexStr<T>` without consuming it
/// ```
/// use flexstr::{FlexStr, ToFlex};
///
/// let a: FlexStr = "This is a heap allocated string!!!!!".to_string().to_flex();
/// assert!(a.is_heap());
/// ```
pub trait ToFlex<T> {
    /// Converts the source to a `FlexStr<T>` without consuming it
    fn to_flex(&self) -> FlexStr<T>;
}

impl<T, T2> ToFlex<T> for FlexStr<T2>
where
    T: for<'a> From<&'a str>,
    T2: Clone + Deref<Target = str>,
{
    /// ```
    /// use flexstr::{AFlexStr, FlexStr, ToFlex};
    ///
    /// let a: AFlexStr = "test".into();
    /// let b: FlexStr = a.to_flex();
    /// assert_eq!(a, b);
    /// ```
    #[inline]
    fn to_flex(&self) -> FlexStr<T> {
        self.clone().into_flex()
    }
}

impl<T> ToFlex<T> for str
where
    T: for<'a> From<&'a str>,
{
    /// ```
    /// use flexstr::{AFlexStr, FlexStr, ToFlex};
    ///
    /// // *** Don't do this - use `into_flex` on literals ***
    /// let a: AFlexStr = "inlined".to_flex();
    /// assert!(a.is_inlined());
    ///
    /// let b: FlexStr = "This is too long to be inlined!!!!!!".to_flex();
    /// assert!(b.is_heap())
    /// ```
    #[inline]
    fn to_flex(&self) -> FlexStr<T> {
        FlexStr(match self.try_into() {
            Ok(s) => FlexStrInner::Inlined(s),
            Err(_) => FlexStrInner::Heap(self.into()),
        })
    }
}

// *** Generic `Into` Custom Traits ***

/// A trait that converts the source to a `FlexStr<T>` while consuming the original
/// ```
/// use flexstr::{FlexStr, IntoFlex};
///
/// let a: FlexStr = "This is a wrapped static string literal no matter how long it is!!!!!".into_flex();
/// assert!(a.is_static());
/// ```
pub trait IntoFlex<T> {
    /// Converts the source to a `FlexStr<T>` while consuming the original
    fn into_flex(self) -> FlexStr<T>;
}

impl<T, T2> IntoFlex<T> for FlexStr<T2>
where
    T: for<'a> From<&'a str>,
    T2: Deref<Target = str>,
{
    /// ```
    /// use flexstr::{AFlexStr, FlexStr, IntoFlex};
    ///
    /// let a: AFlexStr = "This can be just wrapped as a static string literal!".into_flex();
    /// assert!(a.is_static());
    /// let b: FlexStr = a.clone().into_flex();
    /// assert!(b.is_static());
    /// assert_eq!(a, b);
    ///
    /// let c: FlexStr = "Inlined!".to_string().into_flex();
    /// assert!(c.is_inlined());
    /// let d: AFlexStr = c.clone().into_flex();
    /// assert!(d.is_inlined());
    /// assert_eq!(c, d);
    ///
    /// let e: FlexStr = "This will be a wrapped heap allocated `String`!".to_string().into_flex();
    /// assert!(e.is_heap());
    /// let f: AFlexStr = e.clone().into_flex();
    /// assert!(f.is_heap());
    /// assert_eq!(e, f);
    /// ```
    #[inline]
    fn into_flex(self) -> FlexStr<T> {
        FlexStr(match self.0 {
            FlexStrInner::Static(s) => FlexStrInner::Static(s),
            FlexStrInner::Inlined(s) => FlexStrInner::Inlined(s),
            FlexStrInner::Heap(heap) => {
                // TODO: Any more efficient way to do this?
                // Would like to use `from_raw` and `into_raw`, but need to ensure
                // exclusive ownership for this to be safe. For `Rc` that might be possible,
                // but `Arc` could be multi-threaded so needs to be atomic
                FlexStrInner::Heap(T::from(&heap))
            }
        })
    }
}

impl<T> IntoFlex<T> for &'static str {
    /// ```
    /// use flexstr::{AFlexStr, IntoFlex};
    ///
    /// let a = "This can just be wrapped as a static string literal!";
    /// let b: AFlexStr = a.into_flex();
    /// assert!(b.is_static());
    /// assert_eq!(b, a);
    /// ```
    #[inline]
    fn into_flex(self) -> FlexStr<T> {
        self.into()
    }
}

impl<T> IntoFlex<T> for String
where
    T: From<String>,
{
    /// ```
    /// use flexstr::{AFlexStr, IntoFlex};
    ///
    /// let a = "Inlined!".to_string();
    /// let b: AFlexStr = a.clone().into_flex();
    /// assert!(b.is_inlined());
    /// assert_eq!(b, a);
    /// ```
    #[inline]
    fn into_flex(self) -> FlexStr<T> {
        self.into()
    }
}

impl<T> IntoFlex<T> for char
where
    T: From<String> + for<'a> From<&'a str>,
{
    /// ```
    /// use flexstr::{AFlexStr, IntoFlex};
    ///
    /// let a: AFlexStr = 't'.into_flex();
    /// assert!(a.is_inlined());
    /// assert_eq!(a, "t");
    /// ```
    #[inline]
    fn into_flex(self) -> FlexStr<T> {
        self.into()
    }
}

impl<T> IntoFlex<T> for builder::FlexStrBuilder
where
    T: From<String> + for<'a> From<&'a str>,
{
    #[inline]
    fn into_flex(self) -> FlexStr<T> {
        self.into()
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

impl<T> ToFlexStr for FlexStr<T>
where
    T: Clone + Deref<Target = str>,
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
    /// assert!(a.is_inlined());
    /// ```
    #[inline]
    fn to_flex_str(&self) -> FlexStr {
        self.to_flex()
    }
}

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

impl<T> ToAFlexStr for FlexStr<T>
where
    T: Clone + Deref<Target = str>,
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
    /// assert!(a.is_inlined());
    /// ```
    #[inline]
    fn to_a_flex_str(&self) -> AFlexStr {
        self.to_flex()
    }
}

// *** FlexStr `Into` Traits ***

/// A trait that converts the source to a `FlexStr` while consuming the original
/// ```
/// use flexstr::IntoFlexStr;
///
/// let a = "This is a wrapped static string literal no matter how long it is!!!!!".into_flex_str();
/// assert!(a.is_static());
/// ```
pub trait IntoFlexStr {
    /// Converts the source to a `FlexStr` while consuming the original
    fn into_flex_str(self) -> FlexStr;
}

impl<T> IntoFlexStr for FlexStr<T>
where
    T: Deref<Target = str>,
{
    /// ```
    /// use flexstr::{AFlexStr, IntoFlex, IntoFlexStr};
    ///
    /// let a: AFlexStr = "This can be just wrapped as a static string literal!".into_flex();
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

impl IntoFlexStr for &'static str {
    /// ```
    /// use flexstr::IntoFlexStr;
    ///
    /// let a = "This is a wrapped static string literal no matter how long it is!!!!!".into_flex_str();
    /// assert!(a.is_static());
    /// ```
    #[inline]
    fn into_flex_str(self) -> FlexStr {
        self.into()
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
        self.into()
    }
}

impl IntoFlexStr for char {
    /// ```
    /// use flexstr::IntoFlexStr;
    ///
    /// let a = 't'.into_flex_str();
    /// assert!(a.is_inlined());
    /// assert_eq!(a, "t");
    /// ```
    #[inline]
    fn into_flex_str(self) -> FlexStr {
        self.into()
    }
}

// *** AFlexStr `Into` Traits ***

/// A trait that converts the source to a `AFlexStr` while consuming the original
/// ```
/// use flexstr::IntoAFlexStr;
///
/// let a = "This is a wrapped static string literal no matter how long it is!!!!!".into_a_flex_str();
/// assert!(a.is_static());
/// ```
pub trait IntoAFlexStr {
    /// Converts the source to an `AFlexStr` while consuming the original
    fn into_a_flex_str(self) -> AFlexStr;
}

impl<T> IntoAFlexStr for FlexStr<T>
where
    T: Deref<Target = str>,
{
    /// ```
    /// use flexstr::{AFlexStr, IntoFlex, IntoAFlexStr};
    ///
    /// let a: AFlexStr = "This can be just wrapped as a static string literal!".into_flex();
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

impl IntoAFlexStr for &'static str {
    /// ```
    /// use flexstr::IntoAFlexStr;
    ///
    /// let a = "This is a wrapped static string literal no matter how long it is!!!!!".into_a_flex_str();
    /// assert!(a.is_static());
    /// ```
    #[inline]
    fn into_a_flex_str(self) -> AFlexStr {
        self.into()
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
        self.into()
    }
}

impl IntoAFlexStr for char {
    /// ```
    /// use flexstr::IntoAFlexStr;
    ///
    /// let a = 't'.into_a_flex_str();
    /// assert!(a.is_inlined());
    /// assert_eq!(a, "t");
    /// ```
    #[inline]
    fn into_a_flex_str(self) -> AFlexStr {
        self.into()
    }
}
