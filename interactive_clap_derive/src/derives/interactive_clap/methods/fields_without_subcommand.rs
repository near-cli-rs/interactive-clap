extern crate proc_macro;

use proc_macro_error::abort_call_site;
use syn;
use quote::quote;

pub fn is_field_without_subcommand(field: &syn::Field) -> bool {
    if field.attrs.is_empty() {
        return true
    }
    match field.attrs.iter()
        .map(|attr| {
            attr.tokens.clone()
        })
        .flatten()
        .filter(|attr_token| {
            match attr_token {
                proc_macro2::TokenTree::Group(group) => {
                    if group.stream().to_string().contains("named_arg") || group.stream().to_string().contains("subcommand") {
                        true
                    } else {
                        false
                    }
                },
                _ => false // abort_call_site!("Only option `TokenTree::Group` is needed")
            }
        })
        .next() {
            Some(token_stream) => false,
            None => true
        }
}
