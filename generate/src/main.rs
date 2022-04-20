use flexgen::config::Config;
use flexgen::var::{TokenValue, TokenVars};
use flexgen::{import_vars, register_fragments, CodeFragment, CodeGenerator, Error};
use flexstr::{local_fmt, shared_fmt};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use quote_doctest::doc_comment;

struct StrUse;

impl CodeFragment for StrUse {
    fn uses(&self, vars: &TokenVars) -> Result<TokenStream, Error> {
        import_vars! { vars => suffix }

        Ok(match suffix {
            TokenValue::String(s) if s == "BStr" => quote! { use bstr::BStr; },
            TokenValue::String(s) if s == "CStr" => quote! { use std::ffi::CStr; },
            TokenValue::String(s) if s == "OsStr" => quote! { use std::ffi::OsStr; },
            TokenValue::String(s) if s == "Path" => quote! { use std::path::Path; },
            _ => quote! {},
        })
    }
}

struct FlexStruct;

impl CodeFragment for FlexStruct {
    fn uses(&self, _vars: &TokenVars) -> Result<TokenStream, Error> {
        Ok(quote! {
            use crate::FlexStrInner;
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
               FlexStrInner<'str, SIZE, BPAD, HPAD, HEAP, #str_type>);

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
        })
    }
}

struct TypeAliases;

impl CodeFragment for TypeAliases {
    fn uses(&self, _vars: &TokenVars) -> Result<TokenStream, Error> {
        Ok(quote! {
            use crate::custom::{PTR_SIZED_PAD, STRING_SIZED_INLINE};
        })
    }

    fn generate(&self, vars: &TokenVars) -> Result<TokenStream, Error> {
        import_vars! { vars => suffix }

        let ident = format_ident!("Flex{suffix}");
        let ident_3usize = format_ident!("Flex{suffix}3USize");

        let doc_comm = doc_comment(shared_fmt!(
            "Since this is just a type alias for a generic type, full documentation can be found here: [{ident}]"));

        Ok(quote! {
            _comment_!("*** Type Aliases ***");
            _blank_!();

            /// A flexible base string type that transparently wraps a string literal, inline string, or a custom `HEAP` type.
            ///
            /// It is three machine words in size (3x usize) and can hold 22 bytes of inline string data on 64-bit platforms.
            ///
            /// # Note
            #doc_comm
            ///
            /// # Note 2
            /// Custom concrete types need to specify a `HEAP` type with an exact size of two machine words (16 bytes
            /// on 64-bit, and 8 bytes on 32-bit). Any other sized parameter will result in a runtime panic on string
            /// creation.
            pub type #ident_3usize<HEAP> = #ident<STRING_SIZED_INLINE, PTR_SIZED_PAD, PTR_SIZED_PAD, HEAP>;
        })
    }
}

fn main() -> Result<(), Error> {
    let fragments = register_fragments!(StrUse, FlexStruct, TypeAliases);
    let config = Config::from_default_toml_file()?;
    let gen = CodeGenerator::new(fragments, config)?;
    gen.generate_files()
}
