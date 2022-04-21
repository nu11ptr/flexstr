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
        })
    }
}

struct TypeAliases;

impl CodeFragment for TypeAliases {
    fn uses(&self, vars: &TokenVars) -> Result<TokenStream, Error> {
        import_vars! { vars => local_heap_path, shared_heap_path }

        Ok(quote! {
            use crate::custom::{PTR_SIZED_PAD, STRING_SIZED_INLINE};
            use #local_heap_path;
            use #shared_heap_path;
        })
    }

    fn generate(&self, vars: &TokenVars) -> Result<TokenStream, Error> {
        import_vars! {
            vars =>
                suffix, heap_type,
                local_heap_type, local_heap_path, shared_heap_type, shared_heap_path,
                boxed_heap_type, boxed_heap_path
        }

        let flex_ident = format_ident!("Flex{suffix}");
        let ident_3usize = format_ident!("{flex_ident}3USize");

        let local_ident = format_ident!("Local{suffix}");
        let shared_ident = format_ident!("Shared{suffix}");
        let boxed_ident = format_ident!("Boxed{suffix}");

        let local_ref_ident = format_ident!("{local_ident}Ref");
        let shared_ref_ident = format_ident!("{shared_ident}Ref");
        let boxed_ref_ident = format_ident!("{boxed_ident}Ref");

        let full_docs_comm = doc_comment(shared_fmt!(
            "Since this is just a type alias for a generic type, full documentation can be found here: [{flex_ident}]"));

        // *** Basic comment ***

        let desc_comm = |h_type, h_path| {
            doc_comment(shared_fmt!(
            "A flexible string type that transparently wraps a string literal, inline string, or\n\
            a/an [`{h_type}<{heap_type}>`]({h_path})"
        ))
        };

        let local_desc_comm = desc_comm(local_heap_type, local_heap_path);
        let shared_desc_comm = desc_comm(shared_heap_type, shared_heap_path);
        let boxed_desc_comm = desc_comm(boxed_heap_type, boxed_heap_path);

        // *** Basic ref comment ***

        let ref_desc_comm = |h_type, h_path| {
            doc_comment(shared_fmt!(
                "A flexible string type that transparently wraps a string literal, inline string,\n\
            a/an [`{h_type}<{heap_type}>`]({h_path}), or borrowed string (with appropriate lifetime)"
            ))
        };

        let local_ref_desc_comm = ref_desc_comm(local_heap_type, local_heap_path);
        let shared_ref_desc_comm = ref_desc_comm(shared_heap_type, shared_heap_path);
        let boxed_ref_desc_comm = ref_desc_comm(boxed_heap_type, boxed_heap_path);

        // *** Box extra note ***

        let boxed_note = doc_comment(shared_fmt!(
            "This type is included for convenience for those who need wrapped \
            [`{boxed_heap_type}<{heap_type}>`]({boxed_heap_path})"
        ));

        Ok(quote! {
            _comment_!("*** Type Aliases ***");
            _blank_!();

            /// A flexible base string type that transparently wraps a string literal, inline string, or a custom `HEAP` type.
            ///
            /// It is three machine words in size (3x usize) and can hold 22 bytes of inline string data on 64-bit platforms.
            ///
            /// # Note
            #full_docs_comm
            ///
            /// # Note 2
            /// Custom concrete types need to specify a `HEAP` type with an exact size of two machine words (16 bytes
            /// on 64-bit, and 8 bytes on 32-bit). Any other sized parameter will result in a runtime panic on string
            /// creation.
            pub type #ident_3usize<'str, HEAP> =
                #flex_ident<'str, STRING_SIZED_INLINE, PTR_SIZED_PAD, PTR_SIZED_PAD, HEAP>;

            _blank_!();
            #local_desc_comm
            ///
            /// # Note
            #full_docs_comm
            pub type #local_ident = #ident_3usize<#local_heap_type<'static, #heap_type>>;

            _blank_!();
            #shared_desc_comm
            ///
            /// # Note
            #full_docs_comm
            pub type #shared_ident = #ident_3usize<#shared_heap_type<'static, #heap_type>>;

            _blank_!();
            #local_ref_desc_comm
            ///
            /// # Note
            #full_docs_comm
            pub type #local_ref_ident = #ident_3usize<#local_heap_type<'static, #heap_type>>;

            _blank_!();
            #shared_ref_desc_comm
            ///
            /// # Note
            #full_docs_comm
            pub type #shared_ref_ident = #ident_3usize<#shared_heap_type<'static, #heap_type>>;

            _blank_!();
            #boxed_desc_comm
            ///
            /// # Note
            #full_docs_comm
            ///
            /// # Note 2
            #boxed_note
            /// support. Those who do not have this special use case are encouraged to use `Local` or `Shared`
            /// variants for much better clone performance (without copy or additional allocation)
            pub type #boxed_ident = #ident_3usize<#boxed_heap_type<'static, #heap_type>>;

            _blank_!();
            #boxed_ref_desc_comm
            ///
            /// # Note
            #full_docs_comm
            ///
            /// # Note 2
            #boxed_note
            /// support. Those who do not have this special use case are encouraged to use `Local` or `Shared`
            /// variants for much better clone performance (without copy or additional allocation)
            pub type #boxed_ref_ident = #ident_3usize<#boxed_heap_type<'static, #heap_type>>;
        })
    }
}

fn main() -> Result<(), Error> {
    let fragments = register_fragments!(StrUse, FlexStruct, TypeAliases);
    let config = Config::from_default_toml_file()?;
    let gen = CodeGenerator::new(fragments, config)?;
    gen.generate_files()
}
