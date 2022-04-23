use flexgen::var::{TokenValue, TokenVars};
use flexgen::{import_vars, CodeFragment, Error};
use flexstr::local_fmt;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use quote_doctest::{doc_comment, doc_test};

const B_STR: &str = "BStr";
const C_STR: &str = "CStr";
const OS_STR: &str = "OsStr";
const PATH: &str = "Path";
const RAW_STR: &str = "RawStr";
const STR: &str = "Str";

fn str_type_use(suffix: &TokenValue) -> TokenStream {
    match suffix {
        TokenValue::String(s) if s == B_STR => quote! { use bstr::BStr; },
        TokenValue::String(s) if s == C_STR => quote! { use std::ffi::CStr; },
        TokenValue::String(s) if s == OS_STR => quote! { use std::ffi::OsStr; },
        TokenValue::String(s) if s == PATH => quote! { use std::path::Path; },
        TokenValue::String(_) => quote! {},
        _ => panic!("'suffix' was not a string"),
    }
}

fn str_path(suffix: &TokenValue) -> TokenStream {
    match suffix {
        TokenValue::String(s) if s == B_STR => quote! { flexstr::b_str },
        TokenValue::String(s) if s == C_STR => quote! { flexstr::c_str },
        TokenValue::String(s) if s == OS_STR => quote! { flexstr::os_str },
        TokenValue::String(s) if s == PATH => quote! { flexstr::path },
        TokenValue::String(s) if s == RAW_STR => quote! { flexstr::raw_str },
        TokenValue::String(s) if s == STR => quote! { flexstr },
        TokenValue::String(s) => panic!("Unhandled 'suffix': {s}"),
        _ => panic!("'suffix' was not a string"),
    }
}

fn static_str_example(suffix: &TokenValue) -> TokenStream {
    match suffix {
        TokenValue::String(s) if s == B_STR => quote! { (b"test" as &[u8]).into() },
        TokenValue::String(s) if s == C_STR => {
            quote! { CStr::from_bytes_with_nul(b"test\0").unwrap() }
        }
        TokenValue::String(s) if s == OS_STR => quote! { OsStr::new("test") },
        TokenValue::String(s) if s == PATH => quote! { Path::new("test") },
        TokenValue::String(s) if s == RAW_STR => quote! { b"test" },
        TokenValue::String(s) if s == STR => quote! { "test" },
        TokenValue::String(s) => panic!("Unhandled 'suffix': {s}"),
        _ => panic!("'suffix' was not a string"),
    }
}

fn empty_str_example(suffix: &TokenValue) -> TokenStream {
    match suffix {
        TokenValue::String(s) if s == B_STR => quote! { b"" as &[u8] },
        TokenValue::String(s) if s == C_STR => {
            quote! { flexstr::c_str::EMPTY }
        }
        TokenValue::String(s) if s == OS_STR => quote! { OsStr::new("") },
        TokenValue::String(s) if s == PATH => quote! { Path::new("") },
        TokenValue::String(s) if s == RAW_STR => quote! { flexstr::raw_str::EMPTY },
        TokenValue::String(s) if s == STR => quote! { flexstr::EMPTY },
        TokenValue::String(s) => panic!("Unhandled 'suffix': {s}"),
        _ => panic!("'suffix' was not a string"),
    }
}

fn inline_str_example(suffix: &TokenValue) -> TokenStream {
    match suffix {
        TokenValue::String(s) if s == B_STR => quote! { b"inline" as &[u8] },
        TokenValue::String(s) if s == C_STR => {
            quote! { CStr::from_bytes_with_nul(b"inline\0").unwrap() }
        }
        TokenValue::String(s) if s == OS_STR => quote! { OsStr::new("inline") },
        TokenValue::String(s) if s == PATH => quote! { Path::new("inline") },
        TokenValue::String(s) if s == RAW_STR => quote! { b"inline" },
        TokenValue::String(s) if s == STR => quote! { "inline" },
        TokenValue::String(s) => panic!("Unhandled 'suffix': {s}"),
        _ => panic!("'suffix' was not a string"),
    }
}

fn heap_str_example(suffix: &TokenValue) -> TokenStream {
    match suffix {
        TokenValue::String(s) if s == B_STR => {
            quote! { b"This is too long to inline!" as &[u8] }
        }
        TokenValue::String(s) if s == C_STR => {
            quote! { CStr::from_bytes_with_nul(b"This is too long to inline!\0").unwrap() }
        }
        TokenValue::String(s) if s == OS_STR => {
            quote! { OsStr::new("This is too long to inline!") }
        }
        TokenValue::String(s) if s == PATH => quote! { Path::new("This is too long to inline!") },
        TokenValue::String(s) if s == RAW_STR => quote! { b"This is too long to inline!" },
        TokenValue::String(s) if s == STR => quote! { "This is too long to inline!" },
        TokenValue::String(s) => panic!("Unhandled 'suffix': {s}"),
        _ => panic!("'suffix' was not a string"),
    }
}

pub(crate) struct FlexStruct;

impl CodeFragment for FlexStruct {
    fn uses(&self, vars: &TokenVars) -> Result<TokenStream, Error> {
        import_vars! { vars => suffix }

        let str_type_use = str_type_use(suffix);

        Ok(quote! {
            #str_type_use
            use core::ops::Deref;
            use crate::inner::FlexStrInner;
            use crate::storage::Storage;
            use crate::traits::private::FlexStrCoreInner;
            use crate::traits::{FlexStrCore, private};
        })
    }

    fn generate(&self, vars: &TokenVars) -> Result<TokenStream, Error> {
        import_vars! { vars => suffix, str_type }

        let doc_comm = doc_comment(local_fmt!(
            "A flexible string type that transparently wraps a string literal, inline string, or an \n\
             [`Rc<{str_type}>`](std::rc::Rc)"
        ));

        let ident = format_ident!("Flex{suffix}");

        Ok(quote! {
            _comment_!("*** Regular Type ***\n");
            _blank_!();

            #doc_comm
            #[repr(transparent)]
            pub struct #ident<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>(
               pub(crate) FlexStrInner<'str, SIZE, BPAD, HPAD, HEAP, #str_type>);

            _blank_!();
            _comment_!("###  Clone ###\n");
            _blank_!();
            impl<'str, const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> Clone
                for #ident<'str, SIZE, PAD1, PAD2, HEAP>
            where
                HEAP: Storage<#str_type> + Clone,
            {
                #[inline(always)]
                fn clone(&self) -> Self {
                   Self(self.0.clone())
                }
            }

            _blank_!();
            _comment_!("### Deref ###\n");
            _blank_!();
            impl<'str, const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> Deref
                for #ident<'str, SIZE, PAD1, PAD2, HEAP>
            where
                HEAP: Storage<#str_type>,
            {
                type Target = #str_type;

                #[inline(always)]
                fn deref(&self) -> &Self::Target {
                   self.0.as_str_type()
                }
            }

            _blank_!();
            _comment_!("### FlexStrCoreInner ###\n");
            _blank_!();
            impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
                private::FlexStrCoreInner<'str, SIZE, BPAD, HPAD, HEAP, #str_type>
                for #ident<'str, SIZE, BPAD, HPAD, HEAP>
            where
                HEAP: Storage<#str_type>,
            {
                type This = Self;

                #[inline(always)]
                fn wrap(
                    inner: FlexStrInner<'str, SIZE, BPAD, HPAD, HEAP, #str_type>,
                ) -> Self::This {
                    Self(inner)
                }

                #[inline(always)]
                fn inner(&self) -> &FlexStrInner<'str, SIZE, BPAD, HPAD, HEAP, #str_type> {
                    &self.0
                }
            }

            _blank_!();
            _comment_!("### FlexStrCore ###\n");
            _blank_!();
            impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
                FlexStrCore<'str, SIZE, BPAD, HPAD, HEAP, #str_type> for #ident<'str, SIZE, BPAD, HPAD, HEAP>
            where
                HEAP: Storage<#str_type>,
            {
                #[inline(always)]
                fn as_str_type(&self) -> &#str_type {
                    self.inner().as_str_type()
                }
            }
        })
    }
}

struct FromStatic;

impl CodeFragment for FromStatic {
    fn uses(&self, vars: &TokenVars) -> Result<TokenStream, Error> {
        import_vars! { vars => suffix }

        let str_type_use = str_type_use(suffix);

        Ok(quote! {
            #str_type_use
            use crate::inner::FlexStrInner;
        })
    }

    fn generate(&self, vars: &TokenVars) -> Result<TokenStream, Error> {
        import_vars! { vars => suffix, str_type }

        let local_ident = format_ident!("Local{suffix}");
        let str_type_use = str_type_use(suffix);
        let path = str_path(suffix);
        let example = static_str_example(suffix);

        let doc_test = doc_test!(quote! {
            #str_type_use
            use flexstr::FlexStrCore;
            use #path::#local_ident;
            _blank_!();

            let s = #local_ident::from_static(#example);
            assert!(s.is_static());
        })?;

        Ok(quote! {
            /// Creates a wrapped static string literal. This function is equivalent to using the macro and
            /// is `const fn` so it can be used to initialize a constant at compile time with zero runtime cost.
            #doc_test
            #[inline(always)]
            pub const fn from_static(s: &'static #str_type) -> Self {
                Self(FlexStrInner::from_static(s))
            }
        })
    }
}

struct FromRef;

impl CodeFragment for FromRef {
    fn uses(&self, vars: &TokenVars) -> Result<TokenStream, Error> {
        import_vars! { vars => suffix }

        let str_type_use = str_type_use(suffix);

        Ok(quote! {
            #str_type_use
            use crate::inner::FlexStrInner;
        })
    }

    fn generate(&self, vars: &TokenVars) -> Result<TokenStream, Error> {
        import_vars! { vars => suffix, str_type }

        let comm_line_first = doc_comment(local_fmt!(
            "Creates a new string from a `{str_type}` reference. If the string is empty, an empty static string"));
        let comm_line_last = doc_comment(local_fmt!(
            "create strings from a non-static borrowed `{str_type}` where you don't have ownership."
        ));

        let local_ident = format_ident!("Local{suffix}");
        let path = str_path(suffix);
        let empty = empty_str_example(suffix);
        let inline = inline_str_example(suffix);
        let heap = heap_str_example(suffix);
        let str_type_use = str_type_use(suffix);

        let doc_test = doc_test!(quote! {
            #str_type_use
            use flexstr::FlexStrCore;
            use #path::#local_ident;
            _blank_!();

            let s = #local_ident::from_ref(#empty);
            assert!(s.is_static());
            _blank_!();

            let s = #local_ident::from_ref(#inline);
            assert!(s.is_inline());
            _blank_!();

            let s = #local_ident::from_ref(#heap);
            assert!(s.is_heap());
        })?;

        Ok(quote! {
            #comm_line_first
            /// is returned. If at or under the inline length limit, an inline string will be returned.
            /// Otherwise, a heap based string will be allocated and returned. This is typically used to
            #comm_line_last
            ///
            /// # NOTE
            /// Don't use this for string literals or other `'static` strings. Use `from_static` or
            /// the macros instead. Those simply wrap instead of copy and/or allocate.
            #doc_test
            #[inline(always)]
            pub fn from_ref(s: impl AsRef<#str_type>) -> Self {
                Self(FlexStrInner::from_ref(s))
            }
        })
    }
}

pub(crate) struct FlexImpls;

impl CodeFragment for FlexImpls {
    fn uses(&self, vars: &TokenVars) -> Result<TokenStream, Error> {
        let from_static_uses = FromStatic.uses(vars)?;
        let from_ref_uses = FromRef.uses(vars)?;

        Ok(quote! {
            #from_static_uses
            #from_ref_uses
            use crate::storage::Storage;
        })
    }

    fn generate(&self, vars: &TokenVars) -> Result<TokenStream, Error> {
        import_vars! { vars => str_type, suffix }

        let ident = format_ident!("Flex{suffix}");

        let from_static = FromStatic.generate(vars)?;
        let from_ref = FromRef.generate(vars)?;

        Ok(quote! {
            impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
                #ident<'str, SIZE, BPAD, HPAD, HEAP>
            {
                #from_static
            }

            impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
                #ident<'str, SIZE, BPAD, HPAD, HEAP>
            where
                HEAP: Storage<#str_type>
            {
                #from_ref
            }
        })
    }
}
