/// Create compile time constant [LocalStr] (equivalent, but less typing than:
/// `LocalStr::from_static("my_literal")`
/// ```
/// use flexstr::{local_str, LocalStr};
///
/// const STR: LocalStr = local_str!("This is a constant!");
/// assert!(STR.is_static())
/// ```
#[macro_export]
macro_rules! local_str {
    ($str:expr) => {
        <$crate::LocalStr>::from_static($str)
    };
}

/// Create compile time constant [SharedStr] (equivalent, but less typing than:
/// `SharedStr::from_static("my_literal")`
/// ```
/// use flexstr::{shared_str, SharedStr};
///
/// const STR: SharedStr = shared_str!("This is a constant!");
/// assert!(STR.is_static())
/// ```
#[macro_export]
macro_rules! shared_str {
    ($str:expr) => {
        <$crate::SharedStr>::from_static($str)
    };
}

/// Equivalent to [local_fmt] except that it uses `ufmt` which is much faster, but has limitations.
/// See [ufmt docs](https://docs.rs/ufmt/latest/ufmt/) for more details
/// ```
/// use flexstr::{local_str, local_ufmt};
///
/// let a = local_ufmt!("Is {}{}", local_str!("inline"), "!");
/// assert!(a.is_inline());
/// assert_eq!(a, "Is inline!");
/// ```
#[cfg(feature = "fast_format")]
#[macro_export(local_inner_macros)]
macro_rules! local_ufmt {
    ($($arg:tt)*) => {{
        let mut buffer = buffer_new!({ $crate::STRING_SIZED_INLINE });
        let mut builder = builder_new!(buffer);

        ufmt::uwrite!(&mut builder, $($arg)*).expect("a formatting trait implementation returned an error");
        let s: $crate::LocalStr = builder_into!(builder, buffer);
        s
    }}
}

/// Equivalent to [shared_fmt] except that it uses `ufmt` which is much faster, but has limitations.
/// See [ufmt docs](https://docs.rs/ufmt/latest/ufmt/) for more details
/// ```
/// use flexstr::{shared_str, shared_ufmt};
///
/// let a = shared_ufmt!("Is {}{}", shared_str!("inline"), "!");
/// assert!(a.is_inline());
/// assert_eq!(a, "Is inline!");
/// ```
#[cfg(feature = "fast_format")]
#[macro_export(local_inner_macros)]
macro_rules! shared_ufmt {
    ($($arg:tt)*) => {{
        let mut buffer = buffer_new!({ $crate::STRING_SIZED_INLINE });
        let mut builder = builder_new!(buffer);

        ufmt::uwrite!(&mut builder, $($arg)*).expect("a formatting trait implementation returned an error");
        let s: $crate::SharedStr = builder_into!(builder, buffer);
        s
    }}
}

/// Equivalent to [format!] macro from stdlib. Efficiently creates a native [LocalStr]
/// ```
/// use flexstr::local_fmt;
///
/// let a = local_fmt!("Is {}", "inline");
/// assert!(a.is_inline());
/// assert_eq!(a, "Is inline")
/// ```
#[macro_export]
macro_rules! local_fmt {
    ($($arg:tt)*) => {{
        let s: flexstr::LocalStr = flexstr::flex_fmt(format_args!($($arg)*));
        s
    }}
}

/// Equivalent to [format!] macro from stdlib. Efficiently creates a native [SharedStr]
/// ```
/// use flexstr::shared_fmt;
///
/// let a = shared_fmt!("Is {}", "inline");
/// assert!(a.is_inline());
/// assert_eq!(a, "Is inline")
/// ```
#[macro_export]
macro_rules! shared_fmt {
    ($($arg:tt)*) => {{
        let s: flexstr::SharedStr = flexstr::flex_fmt(format_args!($($arg)*));
        s
    }}
}
