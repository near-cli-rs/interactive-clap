use crate::LONG_VEC_MUTLIPLE_OPT;
use proc_macro2::TokenStream;
use proc_macro_error::abort_call_site;
use quote::quote;

/// returns the whole result `TokenStream` of derive logic of containing module
pub fn token_stream(
    name: &syn::Ident,
    cli_name: &syn::Ident,
    input_fields: &syn::Fields,
    ident_skip_field_vec: &[syn::Ident],
) -> TokenStream {
    let fields_conversion = input_fields
        .iter()
        .map(|field| field_conversion(field, ident_skip_field_vec))
        .filter(|token_stream| !token_stream.is_empty());

    quote! {

        impl From<#name> for #cli_name {
            fn from(args: #name) -> Self {
                Self {
                    #( #fields_conversion, )*
                }
            }
        }
    }
}

fn field_conversion(field: &syn::Field, ident_skip_field_vec: &[syn::Ident]) -> TokenStream {
    let ident_field = &field.clone().ident.expect("this field does not exist");
    if ident_skip_field_vec.contains(ident_field) {
        quote!()
    } else {
        let ty = &field.ty;
        if field.attrs.iter().any(|attr|
            attr.path.is_ident("interactive_clap") &&
            attr.tokens.clone().into_iter().any(
                |attr_token|
                matches!(
                    attr_token,
                    proc_macro2::TokenTree::Group(group) if group.stream().to_string() == LONG_VEC_MUTLIPLE_OPT
                )
            )
        ) {
            return quote! {
                #ident_field: args.#ident_field.into()
            };
        }

        match &ty {
            syn::Type::Path(type_path) => match type_path.path.segments.first() {
                Some(path_segment) => {
                    if path_segment.ident == "Option" || path_segment.ident == "bool" {
                        quote! {
                            #ident_field: args.#ident_field.into()
                        }
                    } else {
                        quote! {
                            #ident_field: Some(args.#ident_field.into())
                        }
                    }
                }
                _ => abort_call_site!("Only option `PathSegment` is needed"),
            },
            _ => abort_call_site!("Only option `Type::Path` is needed"),
        }
    }
}
