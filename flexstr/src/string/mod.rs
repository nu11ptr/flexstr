pub(crate) mod b_str;
pub(crate) mod c_str;
pub(crate) mod os_str;
pub(crate) mod raw_str;
pub(crate) mod std_str;

/// Trait used for implementing a custom inner string type ([str], [OsStr](std::ffi::OsStr), [Cstr](std::ffi::CStr), etc.)
pub trait Str {
    /// Regular (typically [Vec]-based) heap allocate string type
    type StringType;
    /// Type of the individual element of the underlying inline array
    type InlineType: Copy;

    /// Transforms a slice of the inline type into the final string type. This is unsafe because
    /// we need to do zero overhead conversions for deref.
    ///
    /// # Safety
    /// This is will only be used in deref with data that has been pre-validated to meet proper invariants
    unsafe fn from_raw_data(bytes: &[Self::InlineType]) -> &Self;

    /// Returns the storage length for this particular string in bytes (not the # of chars)
    fn length(&self) -> usize;

    /// Returns a representation of the inline type as a pointer
    fn as_pointer(&self) -> *const Self::InlineType;
}
