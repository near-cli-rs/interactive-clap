extern crate proc_macro;

use proc_macro2::Span;
use proc_macro_error::abort_call_site;
use quote::{__private::ext::RepToTokensExt, quote};
use syn;

pub fn vec_input_arg(
    ast: &syn::DeriveInput,
    fields: &syn::Fields,
) -> Vec<proc_macro2::TokenStream> {
    let interactive_clap_attrs_context =
        super::interactive_clap_attrs_context::InteractiveClapAttrsContext::new(&ast);
    if interactive_clap_attrs_context.is_skip_default_from_cli {
        return vec![quote!()];
    };
    let vec_fn_input_arg = fields
        .iter()
        .filter(|field| super::fields_without_subcommand::is_field_without_subcommand(field))
        .filter(|field| {
            super::fields_without_skip_default_input_arg::is_field_without_skip_default_input_arg(
                field,
            )
        })
        .map(|field| {
            let ident_field = &field.clone().ident.expect("this field does not exist");
            let ty = &field.ty;

            let input_context_dir = interactive_clap_attrs_context
                .clone()
                .get_input_context_dir();

            let fn_input_arg =
                syn::Ident::new(&format!("input_{}", &ident_field), Span::call_site());

            if field.attrs.is_empty() {
                return quote! {
                    fn #fn_input_arg(
                        _context: &#input_context_dir,
                    ) -> color_eyre::eyre::Result<#ty> {
                        Ok(dialoguer::Input::new()
                            .with_prompt("")
                            .interact_text()?)
                    }
                };
            }

            let doc_attrs = field
                .attrs
                .iter()
                .filter(|attr| attr.path.is_ident("doc".into()))
                .map(|attr| {
                    let mut literal_string = String::new();
                    for attr_token in attr.tokens.clone() {
                        match attr_token {
                            proc_macro2::TokenTree::Literal(literal) => {
                                literal_string = literal.to_string();
                            }
                            _ => (), //abort_call_site!("Only option `TokenTree::Literal` is needed")
                        }
                    }
                    literal_string
                })
                .collect::<Vec<_>>();
            let literal_vec = doc_attrs
                .iter()
                .map(|s| s.replace("\"", ""))
                .collect::<Vec<_>>();
            let literal = proc_macro2::Literal::string(literal_vec.join("\n  ").as_str());

            quote! {
                fn #fn_input_arg(
                    _context: &#input_context_dir,
                ) -> color_eyre::eyre::Result<#ty> {
                    Ok(dialoguer::Input::new()
                        .with_prompt(#literal.to_string().as_str())
                        .interact_text()?)
                }
            }
        })
        .filter(|token_stream| !token_stream.is_empty())
        .collect::<Vec<proc_macro2::TokenStream>>();
    vec_fn_input_arg
}
