extern crate proc_macro;

use proc_macro2::Span;
use proc_macro_error::abort_call_site;
use syn;
use quote::quote;


pub fn from_cli_for_enum(ast: &syn::DeriveInput, variants: &syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let cli_name = syn::Ident::new(&format!("Cli{}", name), Span::call_site());

    let interactive_clap_attrs_context = super::interactive_clap_attrs_context::InteractiveClapAttrsContext::new(&ast);
    if interactive_clap_attrs_context.is_skip_default_from_cli {
         return quote! (); 
    };

    let from_cli_variants = variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                let ty = &fields.unnamed[0].ty;
                let context_name = syn::Ident::new(&format!("{}Context", &name), Span::call_site());


                match &interactive_clap_attrs_context.output_context_dir {
                    Some(_) => quote! {
                        Some(#cli_name::#variant_ident(args)) => {
                            type Alias = <#name as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope;
                            let new_context_scope = Alias::#variant_ident;
                            let new_context = #context_name::from_previous_context((), &new_context_scope);
                            Ok(Self::#variant_ident(#ty::from_cli(Some(args), new_context.into())?,))
                        }
                    },
                    None => quote! {
                        Some(#cli_name::#variant_ident(args)) => Ok(Self::#variant_ident(#ty::from_cli(Some(args), context.clone())?,)),
                    }
                }
            },
            syn::Fields::Unit => {
                quote! {
                    Some(#cli_name::#variant_ident) => Ok(Self::#variant_ident),
                }
            },
            _ => abort_call_site!("Only option `Fields::Unnamed` or `Fields::Unit` is needed")
        }
        
    });

    let input_context_dir = interactive_clap_attrs_context.clone().get_inpun_context_dir();
    
    quote! {
        pub fn from_cli(
            optional_clap_variant: Option<#cli_name>,
            context: #input_context_dir,
        ) -> color_eyre::eyre::Result<Self> {
            match optional_clap_variant {
                #(#from_cli_variants)*
                None => Self::choose_variant(context.clone()),                             
            }
        }
    }
}
