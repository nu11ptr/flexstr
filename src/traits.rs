// *** Generic `To` Custom Traits ***

use crate::{builder, AFlexStr, FlexStr, FlexStrInner};
use alloc::string::String;
use core::ops::Deref;

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

impl<T, U> ToFlex<T> for FlexStr<U>
where
    T: for<'a> From<&'a str>,
    U: Clone + Deref<Target = str>,
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
            Err(_) => FlexStrInner::Heap(T::from(self)),
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

impl<T, U> IntoFlex<T> for FlexStr<U>
where
    T: for<'a> From<&'a str>,
    U: Deref<Target = str>,
{
    /// ```
    /// use flexstr::{AFlexStr, FlexStr, IntoFlex, WFlexStr};
    ///
    /// let a: AFlexStr = "This can be just wrapped as a static string literal!".into_flex();
    /// assert!(a.is_static());
    /// let b: FlexStr = a.clone().into_flex();
    /// assert!(b.is_static());
    /// assert_eq!(a, b);
    ///
    /// let c: FlexStr = "Inlined!".to_string().into_flex();
    /// assert!(c.is_inlined());
    /// let d: WFlexStr = c.clone().into_flex();
    /// assert!(d.is_inlined());
    /// assert_eq!(c, d);
    ///
    /// let e: WFlexStr = "This will be a wrapped heap allocated `String`!".to_string().into_flex();
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
                FlexStrInner::Heap(heap.deref().into())
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
    Self: Into<FlexStr<T>>,
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

// *** AFlexStr `Into` Traits ***

/// A trait that converts the source to a `AFlexStr` while consuming the original
/// ```
/// use flexstr::IntoAFlexStr;
///
/// let a = "This is a wrapped static string literal no matter how long it is!!!!!".into_a_flex_str();
/// assert!(a.is_static());
/// ```
pub trait IntoAFlexStr {
    /// Converts the source to a `AFlexStr` while consuming the original
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
