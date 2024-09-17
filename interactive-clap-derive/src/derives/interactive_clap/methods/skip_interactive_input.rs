extern crate proc_macro;

use syn;

use crate::VEC_MUTLIPLE_OPT;


pub fn is_skip_interactive_input(field: &syn::Field) -> bool {
    field
        .attrs
        .iter()
        .filter(|attr| attr.path.is_ident("interactive_clap"))
        .flat_map(|attr| attr.tokens.clone())
        .any(|attr_token| match attr_token {
            proc_macro2::TokenTree::Group(group) => {
                let group_string = group.stream().to_string();
                group_string.contains("skip_interactive_input")
                    || group_string.contains(VEC_MUTLIPLE_OPT)
            }
            _ => false,
        })
}
