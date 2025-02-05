use proc_macro2::TokenStream;
use quote::quote;

/// returns the whole result `TokenStream` of derive logic of containing module
pub fn token_stream(
    name: &syn::Ident,
    cli_name: &syn::Ident,
    input_fields: &syn::Fields,
) -> TokenStream {
    let (cli_variant_struct, ident_skip_field_vec) =
        cli_variant_struct::token_stream(name, cli_name, input_fields);

    let clap_parser_adapter = clap_parser_trait_adapter::token_stream(name, cli_name);
    let from_trait_impl =
        from_trait::token_stream(name, cli_name, input_fields, &ident_skip_field_vec);
    quote! {
        #cli_variant_struct

        impl interactive_clap::ToCli for #name {
            type CliVariant = #cli_name;
        }

        #clap_parser_adapter

        #from_trait_impl
    }
}

/// describes derive of `#cli_name` struct based on input `#name` struct
mod cli_variant_struct;

/// describes logic of derive of [`clap::Parser`](https://docs.rs/clap/4.5.24/clap/trait.Parser.html) adapter
/// for `#name` struct, which returns instances of `#cli_name` struct
mod clap_parser_trait_adapter;

/// describes the derive of `impl From<#name> for #cli_name`
mod from_trait;
