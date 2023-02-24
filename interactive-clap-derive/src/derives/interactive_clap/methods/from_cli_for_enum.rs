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
                            let new_context = #context_name::from_previous_context(context.clone(), &new_context_scope)?;
                            let output_context = #output_context_dir::from(new_context);
                            let optional_inner_args = <#ty as interactive_clap::FromCli>::from_cli(Some(inner_cli_args), output_context)?;
                            if let Some(inner_args) = optional_inner_args {
                                Ok(Some(Self::#variant_ident(inner_args,)))
                            } else {
                                Self::choose_variant(context.clone())
                            }
                        }
                    },
                    None => quote! {
                        Some(#cli_name::#variant_ident(inner_cli_args)) => {
                            let optional_inner_args = <#ty as interactive_clap::FromCli>::from_cli(Some(inner_cli_args), context.clone().into())?;
                            if let Some(inner_args) = optional_inner_args {
                                Ok(Some(Self::#variant_ident(inner_args,)))
                            } else {
                                Self::choose_variant(context.clone())
                            }
                        }
                    }
                }
            },
            syn::Fields::Unit => {
                quote! {
                    Some(#cli_name::#variant_ident) => Ok(Some(Self::#variant_ident)),
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
            ) -> Result<Option<Self>, Self::FromCliError> where Self: Sized + interactive_clap::ToCli {
                match optional_clap_variant {
                    #(#from_cli_variants)*
                    None => Self::choose_variant(context.clone()),
                }
            }
        }
    }
}
