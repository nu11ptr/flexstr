use alloc::borrow::Cow;
use core::fmt;

pub(crate) mod b_str;
pub(crate) mod c_str;
pub(crate) mod os_str;
pub(crate) mod path;
pub(crate) mod raw_str;
pub(crate) mod std_str;

/// An error occurred during string conversion due to the source string not being UTF-8 compliant
///
/// # Note
/// Usage of `Unknown` vs `WithData` variant is determined on a per string type basis. Currently,
/// only [OsStr](std::ffi::OsStr) and [Path](std::path::Path) don't support `WithData`.
#[derive(Copy, Clone, Debug)]
pub enum Utf8Error {
    /// The source string was not UTF-8, but no further information was available
    Unknown,
    /// The source string was not UTF-8. The enclosed data is equivalent to that of the methods
    /// on the stdlib error of the [same name](core::str::Utf8Error).
    WithData {
        /// Equivalent to the method of the [same name](core::str::Utf8Error::valid_up_to) in stdlib
        valid_up_to: usize,
        /// Equivalent to the method of the [same name](core::str::Utf8Error::error_len) in stdlib
        error_len: Option<usize>,
    },
}

impl fmt::Display for Utf8Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Utf8Error::Unknown => {
                f.write_str("The source string was not UTF-8. No further information is available")
            }
            Utf8Error::WithData { valid_up_to, .. } => {
                write!(
                    f,
                    "The source string was not UTF-8. It is valid up to position {valid_up_to}"
                )
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Utf8Error {}

/// Trait used for implementing a custom inner string type ([str], [OsStr](std::ffi::OsStr), [Cstr](std::ffi::CStr), etc.)
pub trait Str {
    /// Regular (typically [Vec]-based) heap allocate string type
    type StringType;
    /// Type held by the underlying heap storage
    type HeapType: ?Sized;
    /// Error returned when a conversion from raw type to representative type fails
    type ConvertError;

    /// Transforms a slice of the inline stored type into the final string type. This can't fail so
    /// it is only called when the data is already vetted to be valid
    fn from_inline_data(bytes: &[u8]) -> &Self;

    /// Transforms a slice of the heap stored type into the final string type. This can't fail so it
    ///is only called when the data is already vetted to be valid
    fn from_heap_data(bytes: &Self::HeapType) -> &Self;

    /// Tries to transform raw data that has not yet been vetted to the final string type. If it is not
    /// possible, a [Self::ConvertError] is returned
    fn try_from_raw_data(bytes: &[u8]) -> Result<&Self, Self::ConvertError>;

    /// If self is_empty return a static empty string. If not supported by this string type, None is returned
    fn empty(&self) -> Option<&'static Self>;

    /// Returns the storage length for this particular string in bytes (not the # of chars)
    fn length(&self) -> usize;

    /// Returns a representation of the storage type
    fn as_heap_type(&self) -> &Self::HeapType;

    /// Returns a representation of the inline type as a pointer
    fn as_inline_ptr(&self) -> *const u8;

    /// Converts this str reference into a native heap allocated string
    fn to_string_type(&self) -> Self::StringType;

    /// Converts this to a str, if possible, otherwise a UTF8 error is returned
    fn try_to_str(&self) -> Result<&str, Utf8Error>;

    /// Converts this to a str if no alternations needed or an owned `String` with `U+FFFD` chars
    /// if required
    fn to_string_lossy(&self) -> Cow<str>;
}
