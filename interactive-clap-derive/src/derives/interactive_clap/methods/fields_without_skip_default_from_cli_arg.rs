extern crate proc_macro;

use syn;

pub fn is_field_without_skip_default_from_cli_arg(field: &syn::Field) -> bool {
    if field.attrs.is_empty() {
        return true;
    }
    !field
        .attrs
        .iter()
        .filter(|attr| attr.path.is_ident("interactive_clap"))
        .flat_map(|attr| attr.tokens.clone())
        .any(|attr_token| match attr_token {
            proc_macro2::TokenTree::Group(group) => group
                .stream()
                .to_string()
                .contains("skip_default_from_cli_arg"),
            _ => false, // abort_call_site!("Only option `TokenTree::Group` is needed")
        })
}
