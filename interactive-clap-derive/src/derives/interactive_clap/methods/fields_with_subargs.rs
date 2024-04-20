extern crate proc_macro;

use syn;

pub fn is_field_with_subargs(field: &syn::Field) -> bool {
    if field.attrs.is_empty() {
        return false;
    }
    field
        .attrs
        .iter()
        .flat_map(|attr| attr.tokens.clone())
        .any(|attr_token| attr_token.to_string().contains("subargs"))
}
