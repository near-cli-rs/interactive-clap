extern crate proc_macro;

use syn;

pub fn is_field_with_skip_default_input_arg(field: &syn::Field) -> bool {
    if field.attrs.is_empty() {
        return false;
    }
    field
        .attrs
        .iter()
        .filter(|attr| attr.path.is_ident("interactive_clap"))
        .flat_map(|attr| attr.tokens.clone())
        .any(|attr_token| {
            attr_token.to_string().contains("skip_default_input_arg")
                || attr_token.to_string().contains("flatten")
        })
}
