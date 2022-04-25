use alloc::borrow::Cow;
use alloc::string::String;

use crate::storage::WrongStorageType;
use crate::string::Utf8Error;
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
        fn inner(&self) -> &FlexStrInner<'str, SIZE, BPAD, HPAD, HEAP, STR>;
    }
}

/// Ths trait contains most of the core methods and is implemented by all FlexStr types
pub trait FlexStrCore<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP, STR>:
    private::FlexStrCoreInner<'str, SIZE, BPAD, HPAD, HEAP, STR>
where
    HEAP: Storage<STR> + 'static,
    STR: Str + ?Sized + 'static,
{
    /// Attempts to extract a static inline string literal if one is stored inside this [LocalStr].
    /// Returns [WrongStorageType] if this is not a static string literal.
    /// ```
    /// use flexstr::{FlexStrCore, LocalStr};
    ///
    /// let s = "abc";
    /// let s2 = LocalStr::from_static("abc");
    /// assert_eq!(s2.try_as_static_str().unwrap(), s);
    /// ```
    #[inline(always)]
    fn try_as_static_str(&self) -> Result<&'static STR, WrongStorageType> {
        self.inner().try_as_static_str()
    }

    /// Extracts a string slice containing the entire [FlexStr] in the final string type
    /// ```
    /// use flexstr::{FlexStrCore, LocalStr};
    ///
    /// let s = LocalStr::from_ref("abc");
    /// assert_eq!(s.as_str_type(), "abc");
    /// ```
    #[inline(always)]
    fn as_str_type(&'str self) -> &STR {
        self.inner().as_str_type()
    }

    /// Converts this string into its native heap-based string type (ie. `String`, `CString`, `PathBuf`, etc.)
    /// ```
    /// use flexstr::{FlexStrCore, LocalStr};
    ///
    /// let s = LocalStr::from_ref("abc");
    /// assert_eq!(s.to_string_type(), "abc");
    /// ```
    #[inline(always)]
    fn to_string_type(&self) -> STR::StringType {
        self.inner().to_string_type()
    }

    /// Extracts a string slice containing the entire [FlexStr]
    /// ```
    /// use flexstr::{FlexStrCore, LocalStr};
    ///
    /// let s = LocalStr::from_ref("abc");
    /// assert_eq!(s.try_to_str().unwrap(), "abc");
    /// ```
    #[inline(always)]
    fn try_to_str(&'str self) -> Result<&str, Utf8Error> {
        self.inner().try_to_str()
    }

    /// Converts this [FlexStr] into a [String]. This should be more efficient than using the [ToString]
    /// trait (which we cannot implement due to a blanket stdlib implementation) as this avoids the
    /// [Display](alloc::fmt::Display)-based implementation.
    /// ```
    /// use flexstr::{FlexStrCore, LocalStr};
    ///
    /// let s = LocalStr::from_ref("abc").try_to_string().unwrap();
    /// assert_eq!(s, "abc");
    /// ```
    #[inline(always)]
    fn try_to_string(&self) -> Result<String, Utf8Error> {
        self.inner().try_to_string()
    }

    /// Convert the string into an owned or borrowed `str`. If the conversion is possible without
    /// alternations a borrowed `str` will be returned. If it is not possible, non-unicode sequences
    /// will be replaced with `U+FFFD REPLACEMENT CHARACTER` and an owned (`String`) will be returned
    /// instead
    /// ```
    /// use flexstr::{FlexStrCore, LocalStr};
    ///
    /// let s = LocalStr::from_ref("abc");
    /// assert_eq!(s.to_string_lossy(), "abc");
    /// ```
    #[inline(always)]
    fn to_string_lossy(&'str self) -> Cow<'str, str> {
        self.inner().to_string_lossy()
    }

    /// Returns true if this [FlexStr] is empty
    /// ```
    /// use flexstr::{FlexStrCore, LocalStr};
    ///
    /// let s = LocalStr::from_static("");
    /// assert!(s.is_empty());
    /// ```
    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.inner().is_empty()
    }

    /// Returns the length of this [FlexStr] in bytes (not chars or graphemes)
    /// ```
    /// use flexstr::{FlexStrCore, LocalStr};
    ///
    /// let s = LocalStr::from_ref("len");
    /// assert_eq!(s.len(), 3);
    /// ```
    #[inline(always)]
    fn len(&self) -> usize {
        self.inner().len()
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

    /// Returns true if this is a wrapped string using borrowed storage
    #[inline(always)]
    fn is_borrow(&self) -> bool {
        self.inner().is_borrow()
    }
}
