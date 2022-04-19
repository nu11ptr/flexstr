use flexgen::config::Config;
use flexgen::var::TokenVars;
use flexgen::{import_vars, register_fragments, CodeFragment, CodeGenError, CodeGenerator};
use flexstr::local_fmt;
use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use quote_doctest::doc_comment;

struct FlexStruct;

impl CodeFragment for FlexStruct {
    fn uses(&self) -> Option<TokenStream> {
        Some(quote! {
            use crate::FlexStrInner;
            use crate::traits::private::FlexStrCoreInner;
        })
    }

    fn generate(&self, vars: &TokenVars) -> Result<TokenStream, CodeGenError> {
        import_vars! { vars => str_type }

        let doc_comm = doc_comment(local_fmt!(
            "A flexible string type that transparently wraps a string literal, inline string, or an \n\
             [`Rc<{str_type}>`](std::rc::Rc)"
        ));

        let str_ident = format_ident!("{str_type}");
        let ident = format_ident!("Flex{}", str_type.to_string().to_upper_camel_case());

        Ok(quote! {
            _comment_!("*** Regular Type ***\n");
            _blank_!();

            #doc_comm
            #[repr(transparent)]
            pub struct #ident<const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>(
               FlexStrInner<'static, SIZE, BPAD, HPAD, HEAP, #str_ident>);

            _blank_!();
            _comment_!("###  Clone ###\n");
            _blank_!();
            impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> Clone
                for #ident<SIZE, PAD1, PAD2, HEAP>
            where
                HEAP: Storage<#str_ident> + Clone,
            {
                #[inline(always)]
                fn clone(&self) -> Self {
                   Self(self.0.clone())
                }
            }

            _blank_!();
            _comment_!("### Deref ###\n");
            _blank_!();
            impl<const SIZE: usize, const PAD1: usize, const PAD2: usize, HEAP> Deref
                for #ident<SIZE, PAD1, PAD2, HEAP>
            where
                HEAP: Storage<#str_ident>,
            {
                type Target = #str_ident;

                #[inline(always)]
                fn deref(&self) -> &Self::Target {
                   self.0.as_str_type()
                }
            }

            _blank_!();
            _comment_!("### FlexStrCoreInner ###\n");
            _blank_!();
            impl<const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP>
                private::FlexStrCoreInner<'static, SIZE, BPAD, HPAD, HEAP, #str_ident>
                for #ident<SIZE, BPAD, HPAD, HEAP>
            where
                HEAP: Storage<#str_ident>,
            {
                type This = Self;

                #[inline(always)]
                fn wrap(
                    inner: FlexStrInner<'static, SIZE, BPAD, HPAD, HEAP, #str_ident>,
                ) -> Self::This {
                    Self(inner)
                }

                #[inline(always)]
                fn inner(&self) -> &FlexStrInner<'static, SIZE, BPAD, HPAD, HEAP, #str_ident> {
                    &self.0
                }
            }
        })
    }
}

fn main() -> Result<(), CodeGenError> {
    let fragments = register_fragments!(FlexStruct);
    let config = Config::from_default_toml_file()?;
    let gen = CodeGenerator::new(fragments, config)?;
    gen.generate_files()
}
