#![no_std]

extern crate alloc;

use alloc::borrow::ToOwned;
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use core::cmp::Ordering;
use core::fmt;
use core::fmt::{Debug, Display, Formatter};
use core::hash::{Hash, Hasher};
use core::ops::Deref;

use smartstring::{LazyCompact, SmartString};

// *** Stringy macro ***

macro_rules! stringy {
    ($name:ident, $rc:ty, $to_func:ident) => {
        #[derive(Clone, Debug)]
        pub enum $name {
            Static(&'static str),
            Inlined(SmartString<LazyCompact>),
            RefCounted($rc),
        }

        impl $name {
            /// Wrap string verbatim (without possibility of inlining). This can be useful in exclusive
            /// ownership situations where you need to extract the original String later
            pub fn wrap(s: String) -> Self {
                $name::RefCounted(<$rc>::new(s))
            }

            /// Try to retrieve the inner `String` if there is one and we have exclusive ownership. If not
            /// or we don't, then create a new String and return it instead.
            pub fn into_string(self) -> String {
                match self {
                    $name::Static(str) => str.to_string(),
                    $name::Inlined(ss) => ss.into(),
                    $name::RefCounted(rc) => match <$rc>::try_unwrap(rc) {
                        Ok(s) => s,
                        Err(rc) => (&*rc).to_owned(),
                    },
                }
            }

            /// Try to retrieve the inner `String` if there is one and we have exclusive ownership. If not
            /// or we don't, then return our Stringy as the error in the result.
            pub fn try_into_string(self) -> Result<String, $name> {
                match self {
                    s @ $name::Static(_) => Err(s),
                    ss @ $name::Inlined(_) => Err(ss),
                    $name::RefCounted(rc) => <$rc>::try_unwrap(rc).map_err($name::RefCounted),
                }
            }
        }

        // *** Hash, PartialEq, Eq, PartialOrd, Ord ***

        impl Hash for $name {
            #[inline]
            fn hash<H: Hasher>(&self, state: &mut H) {
                Hash::hash(&**self, state)
            }
        }

        impl PartialEq for $name {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                PartialEq::eq(&**self, &**other)
            }
        }

        impl PartialEq<str> for $name {
            #[inline]
            fn eq(&self, other: &str) -> bool {
                PartialEq::eq(&**self, other)
            }
        }

        impl PartialEq<String> for $name {
            #[inline]
            fn eq(&self, other: &String) -> bool {
                PartialEq::eq(&**self, other)
            }
        }

        impl Eq for $name {}

        impl PartialOrd for $name {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                PartialOrd::partial_cmp(&**self, &**other)
            }
        }

        impl PartialOrd<str> for $name {
            #[inline]
            fn partial_cmp(&self, other: &str) -> Option<Ordering> {
                PartialOrd::partial_cmp(&**self, other)
            }
        }

        impl PartialOrd<String> for $name {
            #[inline]
            fn partial_cmp(&self, other: &String) -> Option<Ordering> {
                PartialOrd::partial_cmp(&**self, other)
            }
        }

        impl Ord for $name {
            #[inline]
            fn cmp(&self, other: &Self) -> Ordering {
                Ord::cmp(&**self, &**other)
            }
        }

        // *** Deref / Display ***

        impl Deref for $name {
            type Target = str;

            #[inline]
            fn deref(&self) -> &Self::Target {
                match self {
                    $name::Static(str) => str,
                    $name::Inlined(ss) => ss,
                    $name::RefCounted(rc) => rc,
                }
            }
        }

        impl Display for $name {
            #[inline]
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                match self {
                    $name::Static(str) => Display::fmt(str, f),
                    $name::Inlined(ss) => Display::fmt(ss, f),
                    $name::RefCounted(s) => Display::fmt(s, f),
                }
            }
        }

        // *** From ***

        impl From<String> for $name {
            #[inline]
            fn from(s: String) -> Self {
                if s.len() > smartstring::MAX_INLINE {
                    $name::RefCounted(<$rc>::new(s))
                } else {
                    $name::Inlined(s.into())
                }
            }
        }

        impl From<&String> for $name {
            #[inline]
            fn from(s: &String) -> Self {
                s.$to_func()
            }
        }

        impl From<&'static str> for $name {
            #[inline]
            fn from(s: &'static str) -> Self {
                $name::Static(s)
            }
        }
    };
}

stringy!(Stringy, Rc<String>, to_stringy);
stringy!(AStringy, Arc<String>, to_astringy);

// *** Stringy ***

impl From<&AStringy> for Stringy {
    #[inline]
    fn from(s: &AStringy) -> Self {
        s.clone().into()
    }
}

impl From<AStringy> for Stringy {
    fn from(s: AStringy) -> Self {
        match s {
            AStringy::Static(s) => Stringy::Static(s),
            AStringy::Inlined(s) => Stringy::Inlined(s),
            AStringy::RefCounted(rc) => {
                let s = match Arc::try_unwrap(rc) {
                    Ok(s) => s,
                    Err(rc) => (&*rc).to_owned(),
                };
                Stringy::RefCounted(Rc::new(s))
            }
        }
    }
}

pub trait ToStringy {
    fn to_stringy(&self) -> Stringy;
}

impl ToStringy for str {
    #[inline]
    fn to_stringy(&self) -> Stringy {
        if self.len() > smartstring::MAX_INLINE {
            Stringy::wrap(self.to_string())
        } else {
            Stringy::Inlined(self.into())
        }
    }
}

// *** AStringy ***

impl From<&Stringy> for AStringy {
    #[inline]
    fn from(s: &Stringy) -> Self {
        s.clone().into()
    }
}

impl From<Stringy> for AStringy {
    fn from(s: Stringy) -> Self {
        match s {
            Stringy::Static(s) => AStringy::Static(s),
            Stringy::Inlined(s) => AStringy::Inlined(s),
            Stringy::RefCounted(rc) => {
                let s = match Rc::try_unwrap(rc) {
                    Ok(s) => s,
                    Err(rc) => (&*rc).to_owned(),
                };
                AStringy::RefCounted(Arc::new(s))
            }
        }
    }
}

pub trait ToAStringy {
    fn to_astringy(&self) -> AStringy;
}

impl ToAStringy for str {
    #[inline]
    fn to_astringy(&self) -> AStringy {
        if self.len() > smartstring::MAX_INLINE {
            AStringy::wrap(self.to_string())
        } else {
            AStringy::Inlined(self.into())
        }
    }
}

#[cfg(test)]
mod tests {}
