use flexgen::var::{TokenValue, TokenVars};
use flexgen::{import_vars, CodeFragment, Error};
use flexstr::local_fmt;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use quote_doctest::{doc_comment, doc_test};

fn str_type_use(suffix: &TokenValue) -> TokenStream {
    match suffix {
        TokenValue::String(s) if s == "BStr" => quote! { use bstr::BStr; },
        TokenValue::String(s) if s == "CStr" => quote! { use std::ffi::CStr; },
        TokenValue::String(s) if s == "OsStr" => quote! { use std::ffi::OsStr; },
        TokenValue::String(s) if s == "Path" => quote! { use std::path::Path; },
        _ => quote! {},
    }
}

pub(crate) struct FlexStruct;

impl CodeFragment for FlexStruct {
    fn uses(&self, vars: &TokenVars) -> Result<TokenStream, Error> {
        import_vars! { vars => suffix }

        let str_type_use = str_type_use(suffix);

        Ok(quote! {
            #str_type_use
            use crate::FlexStrInner;
            use crate::traits::FlexStrCore;
            use crate::traits::private::FlexStrCoreInner;
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
            impl<const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
                FlexStrCore<'str, SIZE, BPAD, HPAD, HEAP, #str_type> for #ident<SIZE, BPAD, HPAD, HEAP>
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
            use crate::FlexStrInner;
        })
    }

    fn generate(&self, vars: &TokenVars) -> Result<TokenStream, Error> {
        import_vars! { vars => suffix, str_type }

        let local_ident = format_ident!("Local{suffix}");

        let doc_test = doc_test!(quote! {
            use flexstr::#local_ident;
            _blank_!();

            const S: #local_ident = #local_ident::from_static("test");
            assert!(S.is_static());
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

pub(crate) struct FlexImpls;

impl CodeFragment for FlexImpls {
    fn uses(&self, vars: &TokenVars) -> Result<TokenStream, Error> {
        let from_static_uses = FromStatic.uses(vars)?;

        Ok(quote! {
            #from_static_uses
        })
    }

    fn generate(&self, vars: &TokenVars) -> Result<TokenStream, Error> {
        import_vars! { vars => suffix }

        let ident = format_ident!("Flex{suffix}");

        let from_static = FromStatic.generate(vars)?;

        Ok(quote! {
            impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
                #ident<'str, SIZE, BPAD, HPAD, HEAP> {
                    #from_static
                }
        })
    }
}
