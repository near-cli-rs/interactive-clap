extern crate proc_macro;

use syn;

pub fn is_field_without_skip_default_from_cli_arg(field: &syn::Field) -> bool {
    if field.attrs.is_empty() {
        return true;
    }
    match field
        .attrs
        .iter()
        .filter(|attr| attr.path.is_ident("interactive_clap".into()))
        .map(|attr| attr.tokens.clone())
        .flatten()
        .filter(|attr_token| match attr_token {
            proc_macro2::TokenTree::Group(group) => {
                if group
                    .stream()
                    .to_string()
                    .contains("skip_default_from_cli_arg")
                {
                    true
                } else {
                    false
                }
            }
            _ => false, // abort_call_site!("Only option `TokenTree::Group` is needed")
        })
        .next()
    {
        Some(_) => false,
        None => true,
    }
}
