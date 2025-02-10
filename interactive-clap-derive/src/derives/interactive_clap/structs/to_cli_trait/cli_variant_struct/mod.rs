use proc_macro2::{Span, TokenStream};
use proc_macro_error::abort_call_site;
use quote::{quote, ToTokens};

use crate::{LONG_VEC_MUTLIPLE_OPT, VERBATIM_DOC_COMMENT};

/// describes derive of individual field of `#cli_name` struct
/// based on transformation of input field from `#name` struct
mod field;

/// returns the whole result `TokenStream` of derive logic of containing module
/// and additional info as second returned tuple's element, needed for another derive
pub fn token_stream(
    name: &syn::Ident,
    cli_name: &syn::Ident,
    input_fields: &syn::Fields,
) -> (TokenStream, Vec<syn::Ident>) {
    let (cli_fields, ident_skip_field_vec) = fields(input_fields, name);

    let token_stream = quote! {
        #[derive(Debug, Default, Clone, clap::Parser, interactive_clap::ToCliArgs)]
        #[clap(author, version, about, long_about = None)]
        pub struct #cli_name {
            #( #cli_fields, )*
        }

    };
    (token_stream, ident_skip_field_vec)
}

/// describes derive of all fields of `#cli_name` struct
/// based on transformation of input fields from `#name` struct
fn fields(fields: &syn::Fields, name: &syn::Ident) -> (Vec<TokenStream>, Vec<syn::Ident>) {
    let mut ident_skip_field_vec: Vec<syn::Ident> = Vec::new();

    let fields = fields
                .iter()
                .map(|field| {
                    let ident_field = field.ident.clone().expect("this field does not exist");
                    let ty = &field.ty;
                    let cli_ty = self::field::field_type(ty);
                    let mut cli_field = quote! {
                        pub #ident_field: #cli_ty
                    };
                    if field.attrs.is_empty() {
                        return cli_field;
                    };
                    let mut clap_attr_vec: Vec<proc_macro2::TokenStream> = Vec::new();
                    let mut cfg_attr_vec: Vec<proc_macro2::TokenStream> = Vec::new();
                    let mut doc_attr_vec: Vec<proc_macro2::TokenStream> = Vec::new();
                    for attr in &field.attrs {
                        dbg_cond!(attr.path.to_token_stream().into_iter().collect::<Vec<_>>());
                        if attr.path.is_ident("interactive_clap") || attr.path.is_ident("cfg") {
                            for (_index,  attr_token) in attr.tokens.clone().into_iter().enumerate() {
                                dbg_cond!((_index, &attr_token));
                                match attr_token {
                                    proc_macro2::TokenTree::Group(group) => {
                                        let group_string = group.stream().to_string();
                                        if group_string.contains("subcommand")
                                            || group_string.contains("value_enum")
                                            || group_string.contains("long")
                                            || (group_string == *"skip")
                                            || (group_string == *"flatten")
                                            || (group_string == VERBATIM_DOC_COMMENT)
                                        {
                                            if group_string != LONG_VEC_MUTLIPLE_OPT {
                                                clap_attr_vec.push(group.stream())
                                            }
                                        } else if group.stream().to_string() == *"named_arg" {
                                            let ident_subcommand =
                                                syn::Ident::new("subcommand", Span::call_site());
                                            clap_attr_vec.push(quote! {#ident_subcommand});
                                            let type_string = match ty {
                                                syn::Type::Path(type_path) => {
                                                    match type_path.path.segments.last() {
                                                        Some(path_segment) => {
                                                            path_segment.ident.to_string()
                                                        }
                                                        _ => String::new(),
                                                    }
                                                }
                                                _ => String::new(),
                                            };
                                            let enum_for_clap_named_arg = syn::Ident::new(
                                                &format!(
                                                    "ClapNamedArg{}For{}",
                                                    &type_string, &name
                                                ),
                                                Span::call_site(),
                                            );
                                            cli_field = quote! {
                                                pub #ident_field: Option<#enum_for_clap_named_arg>
                                            }
                                        };
                                        if group.stream().to_string().contains("feature") {
                                            cfg_attr_vec.push(attr.into_token_stream())
                                        };
                                        if group.stream().to_string().contains("subargs") {
                                            let ident_subargs =
                                                syn::Ident::new("flatten", Span::call_site());
                                            clap_attr_vec.push(quote! {#ident_subargs});
                                        };
                                        if group.stream().to_string() == *"skip" {
                                            ident_skip_field_vec.push(ident_field.clone());
                                            cli_field = quote!()
                                        };
                                        if group.stream().to_string() == LONG_VEC_MUTLIPLE_OPT {
                                            if !crate::helpers::type_starts_with_vec(ty) {
                                                abort_call_site!("`{}` attribute is only supposed to be used with `Vec` types", LONG_VEC_MUTLIPLE_OPT)
                                            }
                                            // implies `#[interactive_clap(long)]`
                                            clap_attr_vec.push(quote! { long });
                                            // type goes into output unchanged, otherwise it
                                            // prevents clap deriving correctly its `remove_many` thing  
                                            cli_field = quote! {
                                                pub #ident_field: #ty
                                            };
                                        }
                                    }
                                    _ => {
                                        abort_call_site!("Only option `TokenTree::Group` is needed")
                                    }
                                }
                            }
                        }
                        if attr.path.is_ident("doc") {
                            doc_attr_vec.push(attr.into_token_stream())
                        }
                    }
                    if cli_field.is_empty() {
                        return cli_field;
                    };
                    let cfg_attrs = cfg_attr_vec.iter();
                    if !clap_attr_vec.is_empty() {
                        let clap_attrs = clap_attr_vec.iter();
                        quote! {
                            #(#cfg_attrs)*
                            #(#doc_attr_vec)*
                            #[clap(#(#clap_attrs, )*)]
                            #cli_field
                        }
                    } else {
                        quote! {
                            #(#cfg_attrs)*
                            #(#doc_attr_vec)*
                            #cli_field
                        }
                    }
                })
                .filter(|token_stream| !token_stream.is_empty())
                .collect::<Vec<_>>();
    (fields, ident_skip_field_vec)
}
