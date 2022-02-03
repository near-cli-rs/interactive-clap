extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn;

mod derives;
mod helpers;


#[proc_macro_derive(InteractiveClap, attributes(interactive_clap))]
#[proc_macro_error]
pub fn interactive_clap(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input);
    derives::interactive_clap::impl_interactive_clap(&ast)
}

#[proc_macro_derive(ToCliArgs, attributes(to_cli_args))]
#[proc_macro_error]
pub fn to_cli_args(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input);
    derives::to_cli_args::impl_to_cli_args(&ast)
}
