extern crate proc_macro;

use syn;

pub fn is_field_without_subcommand(field: &syn::Field) -> bool {
    if field.attrs.is_empty() {
        return true;
    }
    match field
        .attrs
        .iter()
        .flat_map(|attr| attr.tokens.clone())
        .find(|attr_token| {
            match attr_token {
                proc_macro2::TokenTree::Group(group) => {
                    group.stream().to_string().contains("named_arg")
                        || group.stream().to_string().contains("subcommand")
                }
                _ => false, // abort_call_site!("Only option `TokenTree::Group` is needed")
            }
        }) {
        Some(_token_stream) => false,
        None => true,
    }
}
