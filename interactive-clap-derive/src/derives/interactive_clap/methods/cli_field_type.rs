extern crate proc_macro;

use proc_macro_error::abort_call_site;
use quote::quote;
use syn;

pub fn cli_field_type(ty: &syn::Type) -> proc_macro2::TokenStream {
    match &ty {
        syn::Type::Path(type_path) => match type_path.path.segments.first() {
            Some(path_segment) => {
                if path_segment.ident == "Option" {
                    match &path_segment.arguments {
                        syn::PathArguments::AngleBracketed(gen_args) => {
                            let ty_option = &gen_args.args;
                            quote! {
                                Option<<#ty_option as interactive_clap::ToCli>::CliVariant>
                            }
                        }
                        _ => {
                            quote! {
                                Option<<#ty as interactive_clap::ToCli>::CliVariant>
                            }
                        }
                    }
                } else if path_segment.ident == "bool" {
                    quote! {
                        bool
                    }
                } else {
                    quote! {
                        Option<<#ty as interactive_clap::ToCli>::CliVariant>
                    }
                }
            }
            _ => abort_call_site!("Only option `PathSegment` is needed"),
        },
        _ => abort_call_site!("Only option `Type::Path` is needed"),
    }
}

pub fn starts_with_vec(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(path_segment) = type_path.path.segments.first() {
            if path_segment.ident == "Vec" {
                return true;
            }
        }
    }
    false
}
