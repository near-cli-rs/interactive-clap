use proc_macro2::TokenStream;
use quote::quote;

/// returns the whole result `TokenStream` of derive logic of containing module
pub fn token_stream(name: &syn::Ident, cli_name: &syn::Ident) -> TokenStream {
    quote! {

        impl #name {
            pub fn try_parse() -> Result<#cli_name, clap::Error> {
                <#cli_name as clap::Parser>::try_parse()
            }

            pub fn parse() -> #cli_name {
                <#cli_name as clap::Parser>::parse()
            }

            pub fn try_parse_from<I, T>(itr: I) -> Result<#cli_name, clap::Error>
            where
                I: ::std::iter::IntoIterator<Item = T>,
                T: ::std::convert::Into<::std::ffi::OsString> + ::std::clone::Clone,
            {
                <#cli_name as clap::Parser>::try_parse_from(itr)
            }
        }
    }
}
