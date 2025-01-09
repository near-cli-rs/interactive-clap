pub mod snake_case_to_camel_case;
pub mod to_kebab_case;

pub fn type_starts_with_vec(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(path_segment) = type_path.path.segments.first() {
            if path_segment.ident == "Vec" {
                return true;
            }
        }
    }
    false
}
