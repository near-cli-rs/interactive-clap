extern crate proc_macro;

use syn;

pub fn is_skip_interactive_input(field: &syn::Field) -> bool {
    if field.attrs.is_empty() {
        return false;
    }
    match field
        .attrs
        .iter()
        .filter(|attr| attr.path.is_ident("interactive_clap"))
        .flat_map(|attr| attr.tokens.clone())
        .find(|attr_token| match attr_token {
            proc_macro2::TokenTree::Group(group) => group
                .stream()
                .to_string()
                .contains("skip_interactive_input"),
            _ => false,
        }) {
        Some(_token_stream) => true,
        None => false,
    }
}
