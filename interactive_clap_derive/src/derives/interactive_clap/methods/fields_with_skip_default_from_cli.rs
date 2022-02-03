extern crate proc_macro;

use proc_macro_error::abort_call_site;
use syn;
use quote::quote;

pub fn field_with_skip_default_from_cli(field: &syn::Field) -> proc_macro2::TokenStream {
    let ident_field = &field.clone().ident.expect("this field does not exist");
    if field.attrs.is_empty() {
        quote! ()
    } else {
        match field.attrs.iter()
        .filter(|attr| attr.path.is_ident("interactive_clap".into()))
        .map(|attr| attr.tokens.clone())
        .flatten()
        .filter(|attr_token| {
            match attr_token {
                proc_macro2::TokenTree::Group(group) => {
                    if group.stream().to_string().contains("skip_default_from_cli") {
                        true
                    } else {
                        false
                    }
                },
                _ => abort_call_site!("Only option `TokenTree::Group` is needed")
            }
        })
        .map(|_| {
            quote! {#ident_field}
        })
        .next() {
            Some(token_stream) => token_stream,
            None => quote! ()
        }
    }
}
