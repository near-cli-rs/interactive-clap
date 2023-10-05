extern crate proc_macro;

use proc_macro2::{Span, TokenStream};
use proc_macro_error::abort_call_site;
use quote::quote;
use syn;

use self::methods::interactive_clap_attrs_cli_field::InteractiveClapAttrsCliField;

mod methods;

pub fn impl_to_cli_args(ast: &syn::DeriveInput) -> TokenStream {
    let cli_name = &ast.ident;
    match &ast.data {
        syn::Data::Struct(data_struct) => {
            let mut args_subcommand = quote! {
                let mut args = std::collections::VecDeque::new();
            };
            let mut args_push_front_vec: Vec<proc_macro2::TokenStream> = Vec::new();

            for field in data_struct.clone().fields.iter() {
                match InteractiveClapAttrsCliField::new(field.clone()) {
                    InteractiveClapAttrsCliField::RegularField(regular_field_args) => {
                        args_push_front_vec.push(regular_field_args)
                    }
                    InteractiveClapAttrsCliField::SubcommandField(subcommand_args) => {
                        args_subcommand = subcommand_args
                    }
                }
            }
            let args_push_front_vec = args_push_front_vec.into_iter().rev();

            quote! {
                impl interactive_clap::ToCliArgs for #cli_name {
                    fn to_cli_args(&self) -> std::collections::VecDeque<String> {
                        #args_subcommand;
                        #(#args_push_front_vec; )*
                        args
                    }
                }
            }
        }
        syn::Data::Enum(syn::DataEnum { variants, .. }) => {
            let enum_variants = variants.iter().map(|variant| {
                let ident = &variant.ident;
                let variant_name_string =
                    crate::helpers::to_kebab_case::to_kebab_case(ident.to_string());
                let variant_name = &syn::LitStr::new(&variant_name_string, Span::call_site());

                match &variant.fields {
                    syn::Fields::Unnamed(_) => {
                        quote! {
                            Self::#ident(subcommand) => {
                                let mut args = subcommand.to_cli_args();
                                args.push_front(#variant_name.to_owned());
                                args
                            }
                        }
                    }
                    syn::Fields::Unit => {
                        quote! {
                            Self::#ident => {
                                let mut args = std::collections::VecDeque::new();
                                args.push_front(#variant_name.to_owned());
                                args
                            }
                        }
                    }
                    _ => abort_call_site!(
                        "Only options `Fields::Unnamed` or `Fields::Unit` are needed"
                    ),
                }
            });
            quote! {
                impl interactive_clap::ToCliArgs for #cli_name {
                    fn to_cli_args(&self) -> std::collections::VecDeque<String> {
                        match self {
                            #( #enum_variants, )*
                        }
                    }
                }
            }
        }
        _ => abort_call_site!("`#[derive(InteractiveClap)]` only supports structs and enums"),
    }
}
