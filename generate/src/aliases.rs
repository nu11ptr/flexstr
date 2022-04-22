use flexgen::var::TokenVars;
use flexgen::{import_vars, CodeFragment, Error};
use flexstr::local_fmt;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use quote_doctest::doc_comment;

pub(crate) struct TypeAliases;

impl CodeFragment for TypeAliases {
    fn uses(&self, vars: &TokenVars) -> Result<TokenStream, Error> {
        import_vars! { vars => local_heap_path, shared_heap_path, boxed_heap_path }

        Ok(quote! {
            use crate::custom::{PTR_SIZED_PAD, STRING_SIZED_INLINE};
            use #local_heap_path;
            use #shared_heap_path;
            use #boxed_heap_path;
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

        let full_docs_comm = doc_comment(local_fmt!(
            "Since this is just a type alias for a generic type, full documentation can be found here: [{flex_ident}]"));

        // *** Basic comment ***

        let desc_comm = |h_type, h_path| {
            doc_comment(local_fmt!(
            "A flexible string type that transparently wraps a string literal, inline string, or\n\
            a/an [`{h_type}<{heap_type}>`]({h_path})"
        ))
        };

        let local_desc_comm = desc_comm(local_heap_type, local_heap_path);
        let shared_desc_comm = desc_comm(shared_heap_type, shared_heap_path);
        let boxed_desc_comm = desc_comm(boxed_heap_type, boxed_heap_path);

        // *** Basic ref comment ***

        let ref_desc_comm = |h_type, h_path| {
            doc_comment(local_fmt!(
                "A flexible string type that transparently wraps a string literal, inline string,\n\
            a/an [`{h_type}<{heap_type}>`]({h_path}), or borrowed string (with appropriate lifetime)"
            ))
        };

        let local_ref_desc_comm = ref_desc_comm(local_heap_type, local_heap_path);
        let shared_ref_desc_comm = ref_desc_comm(shared_heap_type, shared_heap_path);
        let boxed_ref_desc_comm = ref_desc_comm(boxed_heap_type, boxed_heap_path);

        // *** Box extra note ***

        let boxed_note = doc_comment(local_fmt!(
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
            pub type #local_ident = #ident_3usize<'static, #local_heap_type<#heap_type>>;

            _blank_!();
            #local_ref_desc_comm
            ///
            /// # Note
            #full_docs_comm
            pub type #local_ref_ident<'str> = #ident_3usize<'str, #local_heap_type<#heap_type>>;

            _blank_!();
            #shared_desc_comm
            ///
            /// # Note
            #full_docs_comm
            pub type #shared_ident = #ident_3usize<'static, #shared_heap_type<#heap_type>>;

            _blank_!();
            #shared_ref_desc_comm
            ///
            /// # Note
            #full_docs_comm
            pub type #shared_ref_ident<'str> = #ident_3usize<'str, #shared_heap_type<#heap_type>>;

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
            pub type #boxed_ident = #ident_3usize<'static, #boxed_heap_type<#heap_type>>;

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
            pub type #boxed_ref_ident<'str> = #ident_3usize<'str, #boxed_heap_type<#heap_type>>;
        })
    }
}
