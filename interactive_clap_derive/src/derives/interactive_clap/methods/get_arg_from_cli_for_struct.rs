extern crate proc_macro;

use proc_macro2::Span;
use syn;
use quote::quote;


pub fn from_cli_arg(ast: &syn::DeriveInput, fields: &syn::Fields) -> Vec<proc_macro2::TokenStream> {
    let interactive_clap_attrs_context = super::interactive_clap_attrs_context::InteractiveClapAttrsContext::new(&ast);
    if interactive_clap_attrs_context.is_skip_default_from_cli {
        return vec![quote! ()]; 
    };

    let fields_without_subcommand = fields.iter().map(|field| {
        super::fields_without_subcommand::field_without_subcommand(field)                
    })
    .filter(|token_stream| !token_stream.is_empty())
    .collect::<Vec<_>>();

    let fields_without_skip_default_from_cli = fields.iter().map(|field| {
        super::fields_with_skip_default_from_cli::field_with_skip_default_from_cli(field)                
    })
    .filter(|token_stream| !token_stream.is_empty())
    .collect::<Vec<_>>();

    let get_arg_for_fields = fields.iter().map(|field| {
        let ident_field = &field.clone().ident.expect("this field does not exist");
        let ty = &field.ty;
        let fields_without_subcommand_to_string = fields_without_subcommand.iter().map(|token_stream| token_stream.to_string()).collect::<Vec<_>>();
        let fields_with_skip_default_from_cli_to_string = fields_without_skip_default_from_cli.iter().map(|token_stream| token_stream.to_string()).collect::<Vec<_>>();
        if fields_without_subcommand_to_string.contains(&ident_field.to_string()) & !fields_with_skip_default_from_cli_to_string.contains(&ident_field.to_string()) {
            let fn_from_cli_arg = syn::Ident::new(&format!("from_cli_{}", &ident_field), Span::call_site());
            let optional_cli_field_name = syn::Ident::new(&format!("optional_cli_{}", ident_field), Span::call_site());
            let input_context_dir = interactive_clap_attrs_context.clone().get_inpun_context_dir();
            let cli_field_type = super::cli_field_type::cli_field_type(ty);
            let fn_input_arg = syn::Ident::new(&format!("input_{}", &ident_field), Span::call_site());
            quote! {
                fn #fn_from_cli_arg(
                    #optional_cli_field_name: #cli_field_type,
                    context: &#input_context_dir,
                ) -> color_eyre::eyre::Result<#ty> {
                    match #optional_cli_field_name {
                        Some(#ident_field) => Ok(#ident_field),
                        None => Self::#fn_input_arg(&context),
                    }
                }
            }
        } else {
            quote! ()
        }             
    })
    .filter(|token_stream| !token_stream.is_empty())
    .collect::<Vec<proc_macro2::TokenStream>>();

    get_arg_for_fields
}
