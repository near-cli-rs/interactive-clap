extern crate proc_macro;

use proc_macro2::Span;
use proc_macro_error::abort_call_site;
use quote::quote;
use syn;

pub fn from_cli_for_enum(
    ast: &syn::DeriveInput,
    variants: &syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>,
) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let cli_name = syn::Ident::new(&format!("Cli{}", name), Span::call_site());

    let interactive_clap_attrs_context =
        super::interactive_clap_attrs_context::InteractiveClapAttrsContext::new(ast);
    if interactive_clap_attrs_context.is_skip_default_from_cli {
        return quote!();
    };

    let from_cli_variants = variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                let ty = &fields.unnamed[0].ty;
                let context_name = syn::Ident::new(&format!("{}Context", &name), Span::call_site());

                match &interactive_clap_attrs_context.output_context_dir {
                    Some(output_context_dir) => quote! {
                        Some(#cli_name::#variant_ident(inner_cli_args)) => {
                            type Alias = <#name as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope;
                            let new_context_scope = Alias::#variant_ident;
                            let new_context = match #context_name::from_previous_context(context, &new_context_scope) {
                                Ok(new_context) => new_context,
                                Err(err) => return interactive_clap::ResultFromCli::Err(Some(#cli_name::#variant_ident(inner_cli_args)), err),
                            };
                                let output_context = #output_context_dir::from(new_context);
                            let cli_inner_args = <#ty as interactive_clap::FromCli>::from_cli(Some(inner_cli_args), output_context);
                            match cli_inner_args {
                                interactive_clap::ResultFromCli::Ok(cli_args) => {
                                    interactive_clap::ResultFromCli::Ok(#cli_name::#variant_ident(cli_args))
                                }
                                interactive_clap::ResultFromCli::Back => {
                                    interactive_clap::ResultFromCli::Back
                                }
                                interactive_clap::ResultFromCli::Cancel(Some(cli_args)) => {
                                    interactive_clap::ResultFromCli::Cancel(Some(#cli_name::#variant_ident(cli_args)))
                                }
                                interactive_clap::ResultFromCli::Cancel(None) => {
                                    interactive_clap::ResultFromCli::Cancel(None)
                                }
                                interactive_clap::ResultFromCli::Err(Some(cli_args), err) => {
                                    interactive_clap::ResultFromCli::Err(Some(#cli_name::#variant_ident(cli_args)), err)
                                }
                                interactive_clap::ResultFromCli::Err(None, err) => {
                                    interactive_clap::ResultFromCli::Err(None, err)
                                }
                            }
                        }
                    },
                    None => quote! {
                        Some(#cli_name::#variant_ident(inner_cli_args)) => {
                            let cli_inner_args = <#ty as interactive_clap::FromCli>::from_cli(Some(inner_cli_args), context.clone().into());
                            match cli_inner_args {
                                interactive_clap::ResultFromCli::Ok(cli_args) => {
                                    interactive_clap::ResultFromCli::Ok(#cli_name::#variant_ident(cli_args))
                                }
                                interactive_clap::ResultFromCli::Back => {
                                    interactive_clap::ResultFromCli::Back
                                }
                                interactive_clap::ResultFromCli::Cancel(Some(cli_args)) => {
                                    interactive_clap::ResultFromCli::Cancel(Some(#cli_name::#variant_ident(cli_args)))
                                }
                                interactive_clap::ResultFromCli::Cancel(None) => {
                                    interactive_clap::ResultFromCli::Cancel(None)
                                }
                                interactive_clap::ResultFromCli::Err(Some(cli_args), err) => {
                                    interactive_clap::ResultFromCli::Err(Some(#cli_name::#variant_ident(cli_args)), err)
                                }
                                interactive_clap::ResultFromCli::Err(None, err) => {
                                    interactive_clap::ResultFromCli::Err(None, err)
                                }
                            }
                        }
                    }
                }
            },
            syn::Fields::Unit => {
                quote! {
                    Some(#cli_name::#variant_ident) => interactive_clap::ResultFromCli::Ok(#cli_name::#variant_ident),
                }
            },
            _ => abort_call_site!("Only option `Fields::Unnamed` or `Fields::Unit` is needed")
        }
    });

    let input_context_dir = interactive_clap_attrs_context
        .clone()
        .get_input_context_dir();

    quote! {
        impl interactive_clap::FromCli for #name {
            type FromCliContext = #input_context_dir;
            type FromCliError = color_eyre::eyre::Error;
            fn from_cli(
                optional_clap_variant: Option<<Self as interactive_clap::ToCli>::CliVariant>,
                context: Self::FromCliContext,
            ) -> interactive_clap::ResultFromCli<<Self as interactive_clap::ToCli>::CliVariant, Self::FromCliError> where Self: Sized + interactive_clap::ToCli {
                match optional_clap_variant {
                    #(#from_cli_variants)*
                    None => Self::choose_variant(context.into()),
                }
            }
        }
    }
}
