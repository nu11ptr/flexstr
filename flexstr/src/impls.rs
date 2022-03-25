use alloc::string::String;
use core::cmp::Ordering;
use core::convert::Infallible;
use core::fmt;
use core::fmt::{Debug, Display, Formatter};
use core::hash::{Hash, Hasher};
#[cfg(feature = "serde")]
use core::marker::PhantomData;
use core::ops::{
    Add, Deref, Index, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};
use core::str::FromStr;

#[cfg(feature = "serde")]
use serde::de::{Error, Visitor};
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{from_iter_char, from_iter_str, FlexStr, IntoFlex};

// *** Debug / Display ***

// FIXME: Do we want to do something custom?
impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR, STRING> Debug
    for FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    HEAP: Deref<Target = str>,
    STR: ?Sized,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <str as Debug>::fmt(self, f)
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR, STRING> Display
    for FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    HEAP: Deref<Target = str>,
    STR: ?Sized,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <str as Display>::fmt(self, f)
    }
}

#[cfg(feature = "fast_format")]
impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR, STRING> ufmt::uDisplay
    for FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    HEAP: Deref<Target = str>,
    STR: ?Sized,
{
    #[inline]
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt_write::uWrite + ?Sized,
    {
        <str as ufmt::uDisplay>::fmt(self, f)
    }
}

#[cfg(feature = "fast_format")]
impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR, STRING> ufmt::uDebug
    for FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    HEAP: Deref<Target = str>,
    STR: ?Sized,
{
    #[inline]
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt_write::uWrite + ?Sized,
    {
        // uDebug is not implemented for str it seems which means we can't derive
        <str as ufmt::uDisplay>::fmt(self, f)
    }
}

// *** Hash, PartialEq, Eq ***

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR, STRING> Hash
    for FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    HEAP: Deref<Target = str>,
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        str::hash(self, state)
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, HEAP2, STR, STRING>
    PartialEq<FlexStr<SIZE, PAD1, PAD2, HEAP2, STR, STRING>>
    for FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    HEAP: Deref<Target = str>,
    HEAP2: Deref<Target = str>,
    STR: ?Sized,
{
    /// ```
    /// use flexstr::{SharedStr, LocalStr, ToFlex};
    ///
    /// let lit = "inlined";
    /// let s: LocalStr = lit.into();
    /// let s2: SharedStr = lit.into();
    /// assert_eq!(s, s2);
    /// ```
    #[inline]
    fn eq(&self, other: &FlexStr<SIZE, PAD1, PAD2, HEAP2, STR, STRING>) -> bool {
        str::eq(self, &**other)
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, HEAP2, STR, STRING>
    PartialEq<FlexStr<SIZE, PAD1, PAD2, HEAP2, STR, STRING>>
    for &FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    HEAP: Deref<Target = str>,
    HEAP2: Deref<Target = str>,
    STR: ?Sized,
{
    /// ```
    /// use flexstr::{SharedStr, LocalStr, ToFlex};
    ///
    /// let lit = "inlined";
    /// let s: LocalStr = lit.into();
    /// let s2: SharedStr = lit.into();
    /// assert_eq!(&s, s2);
    /// ```
    #[inline]
    fn eq(&self, other: &FlexStr<SIZE, PAD1, PAD2, HEAP2, STR, STRING>) -> bool {
        str::eq(self, &**other)
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR, STRING> PartialEq<&str>
    for FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    HEAP: Deref<Target = str>,
    STR: ?Sized,
{
    /// ```
    /// use flexstr::{LocalStr, ToFlex};
    ///
    /// let lit = "inlined";
    /// let s: LocalStr = lit.to_flex();
    /// assert_eq!(s, lit);
    /// ```
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        str::eq(self, *other)
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR, STRING> PartialEq<str>
    for FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    HEAP: Deref<Target = str>,
    STR: ?Sized,
{
    /// ```
    /// use flexstr::{LocalStr, ToFlex};
    ///
    /// let lit = "inlined";
    /// let s: LocalStr = lit.to_flex();
    /// assert_eq!(s, lit);
    /// ```
    #[inline]
    fn eq(&self, other: &str) -> bool {
        str::eq(self, other)
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR, STRING> PartialEq<String>
    for FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    HEAP: Deref<Target = str>,
    STR: ?Sized,
{
    /// ```
    /// use flexstr::LocalStr;
    ///
    /// let lit = "inlined";
    /// let s: LocalStr = lit.into();
    /// assert_eq!(s, lit.to_string());
    /// ```
    #[inline]
    fn eq(&self, other: &String) -> bool {
        str::eq(self, other)
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR, STRING> Eq
    for FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    HEAP: Deref<Target = str>,
    STR: ?Sized,
{
}

// *** PartialOrd / Ord ***

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR, STRING> PartialOrd
    for FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    HEAP: Deref<Target = str>,
    STR: ?Sized,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        str::partial_cmp(self, other)
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR, STRING> PartialOrd<str>
    for FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    HEAP: Deref<Target = str>,
    STR: ?Sized,
{
    #[inline]
    fn partial_cmp(&self, other: &str) -> Option<Ordering> {
        str::partial_cmp(self, other)
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR, STRING> PartialOrd<String>
    for FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    HEAP: Deref<Target = str>,
    STR: ?Sized,
{
    #[inline]
    fn partial_cmp(&self, other: &String) -> Option<Ordering> {
        str::partial_cmp(self, other)
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR, STRING> Ord
    for FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    HEAP: Deref<Target = str>,
    STR: ?Sized,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        str::cmp(self, other)
    }
}

// *** Index ***

macro_rules! impl_ranges {
    ($($type:ty),+) => {
        $(impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR, STRING> Index<$type> for FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
        where
            HEAP: Deref<Target = str>,
            STR: ?Sized,
        {
            type Output = str;

            #[inline]
            fn index(&self, index: $type) -> &Self::Output {
                str::index(self, index)
            }
        })+
    }
}

impl_ranges!(
    Range<usize>,
    RangeTo<usize>,
    RangeFrom<usize>,
    RangeFull,
    RangeInclusive<usize>,
    RangeToInclusive<usize>
);

// *** Add ***

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR, STRING> Add<&str>
    for FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    HEAP: for<'a> From<&'a str> + Deref<Target = str>,
    STR: ?Sized,
{
    type Output = Self;

    /// ```
    /// use flexstr::{local_str, IntoLocalStr};
    ///
    /// let a = local_str!("in") + "line";
    /// assert!(a.is_inline());
    /// assert_eq!(a, "inline");
    ///
    /// let a = "in".to_string().into_local_str() + "line";
    /// assert!(a.is_inline());
    /// assert_eq!(a, "inline");
    /// ```
    #[inline]
    fn add(self, rhs: &str) -> Self::Output {
        self.add(rhs)
    }
}

// *** Misc. standard traits ***

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR, STRING> AsRef<str>
    for FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    HEAP: Deref<Target = str>,
    STR: ?Sized,
{
    #[inline]
    fn as_ref(&self) -> &str {
        self
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR, STRING> Default
    for FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    STR: ?Sized,
{
    #[inline]
    fn default() -> Self {
        Self::from_static("")
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR, STRING> FromStr
    for FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    HEAP: for<'a> From<&'a str>,
    STR: ?Sized,
{
    type Err = Infallible;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

// *** From ***

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, HEAP2, STR, STRING>
    From<&FlexStr<SIZE, PAD1, PAD2, HEAP2, STR, STRING>>
    for FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    HEAP: for<'a> From<&'a str>,
    HEAP2: Clone + Deref<Target = str>,
    STR: ?Sized,
{
    #[inline]
    fn from(s: &FlexStr<SIZE, PAD1, PAD2, HEAP2, STR, STRING>) -> Self {
        s.clone().into_flex()
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR, STRING> From<String>
    for FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    HEAP: for<'a> From<&'a str>,
    STR: ?Sized,
{
    /// ```
    /// use flexstr::LocalStr;
    ///
    /// let lit = "inlined";
    /// let s: LocalStr = lit.to_string().into();
    /// assert!(s.is_inline());
    /// assert_eq!(&s, lit);
    ///
    /// let lit = "This is too long too be inlined!";
    /// let s: LocalStr = lit.to_string().into();
    /// assert!(s.is_heap());
    /// assert_eq!(&s, lit);
    /// ```
    #[inline]
    fn from(s: String) -> Self {
        <Self as From<&str>>::from(&s)
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR, STRING> From<&String>
    for FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    HEAP: for<'a> From<&'a str>,
    STR: ?Sized,
{
    /// ```
    /// use flexstr::LocalStr;
    ///
    /// let lit = "inlined";
    /// let s: LocalStr = (&lit.to_string()).into();
    /// assert!(s.is_inline());
    /// assert_eq!(&s, lit);
    ///
    /// let lit = "This is too long too be inlined!";
    /// let s: LocalStr = (&lit.to_string()).into();
    /// assert!(s.is_heap());
    /// assert_eq!(&s, lit);
    /// ```
    #[inline]
    fn from(s: &String) -> Self {
        <Self as From<&str>>::from(s)
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR, STRING> From<&str>
    for FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    HEAP: for<'a> From<&'a str>,
    STR: ?Sized,
{
    /// ```
    /// use flexstr::LocalStr;
    ///
    /// let lit = "inline";
    /// let s: LocalStr  = lit.into();
    /// assert!(s.is_inline());
    /// assert_eq!(&s, lit);
    /// ```
    #[inline]
    fn from(s: &str) -> Self {
        Self::from_ref(s)
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR, STRING> From<char>
    for FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    HEAP: Deref<Target = str>,
    STR: ?Sized,
{
    /// ```
    /// use flexstr::LocalStr;
    ///
    /// let s: LocalStr  = 't'.into();
    /// assert!(s.is_inline());
    /// assert_eq!(&s, "t");
    /// ```
    #[inline]
    fn from(ch: char) -> Self {
        Self::from_char(ch)
    }
}

// *** FromIterator ***

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, HEAP2, STR, STRING>
    FromIterator<FlexStr<SIZE, PAD1, PAD2, HEAP2, STR, STRING>>
    for FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    HEAP: for<'b> From<&'b str>,
    HEAP2: Deref<Target = str>,
    STR: ?Sized,
{
    /// ```
    /// use flexstr::LocalStr;
    ///
    /// let v: Vec<LocalStr> = vec!["best".into(), "test".into()];
    /// let s: LocalStr = v.into_iter().map(|s| if s == "best" { "test".into() } else { s }).collect();
    /// assert!(s.is_inline());
    /// assert_eq!(s, "testtest");
    /// ```
    #[inline]
    fn from_iter<I: IntoIterator<Item = FlexStr<SIZE, PAD1, PAD2, HEAP2, STR, STRING>>>(
        iter: I,
    ) -> Self {
        from_iter_str(iter)
    }
}

impl<'a, const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, HEAP2, STR, STRING>
    FromIterator<&'a FlexStr<SIZE, PAD1, PAD2, HEAP2, STR, STRING>>
    for FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    HEAP: for<'b> From<&'b str>,
    HEAP2: Deref<Target = str> + 'a,
    STR: ?Sized + 'a,
    STRING: 'a,
{
    /// ```
    /// use flexstr::LocalStr;
    ///
    /// let v: Vec<LocalStr> = vec!["best".into(), "test".into()];
    /// let s: LocalStr = v.iter().filter(|s| *s == "best").collect();
    /// assert!(s.is_inline());
    /// assert_eq!(s, "best");
    /// ```
    #[inline]
    fn from_iter<I: IntoIterator<Item = &'a FlexStr<SIZE, PAD1, PAD2, HEAP2, STR, STRING>>>(
        iter: I,
    ) -> Self {
        from_iter_str(iter)
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR, STRING>
    FromIterator<String> for FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    HEAP: for<'b> From<&'b str>,
    STR: ?Sized,
{
    /// ```
    /// use flexstr::LocalStr;
    ///
    /// let v = vec!["best".to_string(), "test".to_string()];
    /// let s: LocalStr = v.into_iter().map(|s| if s == "best" { "test".into() } else { s }).collect();
    /// assert!(s.is_inline());
    /// assert_eq!(s, "testtest");
    /// ```
    #[inline]
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        from_iter_str(iter)
    }
}

impl<'a, const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR, STRING>
    FromIterator<&'a str> for FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    HEAP: for<'b> From<&'b str>,
    STR: ?Sized,
{
    /// ```
    /// use flexstr::LocalStr;
    ///
    /// let v = vec!["best", "test"];
    /// let s: LocalStr = v.into_iter().map(|s| if s == "best" { "test" } else { s }).collect();
    /// assert!(s.is_inline());
    /// assert_eq!(s, "testtest");
    /// ```
    #[inline]
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        from_iter_str(iter)
    }
}

impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR, STRING> FromIterator<char>
    for FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    HEAP: for<'b> From<&'b str>,
    STR: ?Sized,
{
    /// ```
    /// use flexstr::LocalStr;
    ///
    /// let v = "besttest";
    /// let s: LocalStr = v.chars().map(|c| if c == 'b' { 't' } else { c }).collect();
    /// assert!(s.is_inline());
    /// assert_eq!(s, "testtest");
    /// ```
    #[inline]
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
        from_iter_char(iter, |ch| ch)
    }
}

impl<'a, const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR, STRING>
    FromIterator<&'a char> for FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    HEAP: for<'b> From<&'b str>,
    STR: ?Sized,
{
    /// ```
    /// use flexstr::LocalStr;
    ///
    /// let v = vec!['b', 'e', 's', 't', 't', 'e', 's', 't'];
    /// let s: LocalStr = v.iter().filter(|&ch| *ch != 'b').collect();
    /// assert!(s.is_inline());
    /// assert_eq!(s, "esttest");
    /// ```
    #[inline]
    fn from_iter<I: IntoIterator<Item = &'a char>>(iter: I) -> Self {
        from_iter_char(iter, |ch| *ch)
    }
}

// *** Optional serialization support ***

#[cfg(feature = "serde")]
impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR, STRING> Serialize
    for FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    HEAP: Deref<Target = str>,
    STR: ?Sized,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self)
    }
}

#[cfg(feature = "serde")]
struct FlexStrVisitor<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR, STRING>(
    PhantomData<(HEAP, STRING, STR)>,
)
where
    STR: ?Sized;

#[cfg(feature = "serde")]
impl<'de, const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR, STRING> Visitor<'de>
    for FlexStrVisitor<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    HEAP: for<'a> From<&'a str>,
    STR: ?Sized,
{
    type Value = FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>;

    #[inline]
    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }

    #[inline]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v.into())
    }

    #[inline]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v.into())
    }
}

#[cfg(feature = "serde")]
impl<'de, const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP, STR, STRING>
    Deserialize<'de> for FlexStr<SIZE, PAD1, PAD2, HEAP, STR, STRING>
where
    HEAP: for<'a> From<&'a str>,
    STR: ?Sized,
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(FlexStrVisitor(PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use crate::local_str;

    #[cfg(feature = "serde")]
    #[test]
    fn serialization() {
        use crate::{LocalStr, SharedStr};
        use alloc::string::ToString;
        use serde_json::json;

        #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq)]
        struct Test {
            a: LocalStr,
            b: SharedStr,
            c: LocalStr,
        }

        let a = "test";
        let b = "testing";
        let c = "testing testing testing testing testing testing testing testing testing";

        // Create our struct and values and verify storage
        let test = Test {
            a: local_str!(a),
            b: b.to_string().into(),
            c: c.to_string().into(),
        };
        assert!(test.a.is_static());
        assert!(test.b.is_inline());
        assert!(test.c.is_heap());

        // Serialize and ensure our JSON value actually matches
        let val = serde_json::to_value(test.clone()).unwrap();
        assert_eq!(json!({"a": a, "b": b, "c": c}), val);

        // Deserialize and validate storage and contents
        let test2: Test = serde_json::from_value(val).unwrap();
        assert!(test2.a.is_inline());
        assert!(test2.b.is_inline());
        assert!(test2.c.is_heap());

        assert_eq!(&test, &test2);
    }
}
