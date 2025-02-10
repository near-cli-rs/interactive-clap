/*!
`interactive_clap::ToCli` derive

This module describes the derive logic of `#cli_name` struct used as `CliVariant` in
implementation of `interactive_clap::ToCli`, which happens during derive of [`crate::InteractiveClap`] for `#name` struct.

```rust,ignore
#[derive(Debug, Default, Clone, clap::Parser, interactive_clap::ToCliArgs)]
#[clap(author, version, about, long_about = None)]
pub struct #cli_name {
   #( #cli_fields, )*
}

impl interactive_clap::ToCli for #name {
   type CliVariant = #cli_name;
}
```

Where `interactive_clap::ToCli` is:

```rust,ignore
pub trait ToCli {
   type CliVariant;
}
```
Additionally a [`clap::Parser`](https://docs.rs/clap/4.5.24/clap/trait.Parser.html) adapter
for `#name` and `From<#name> for #cli_name` conversion are defined:

```rust,ignore
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

impl From<#name> for #cli_name {
   fn from(args: #name) -> Self {
       Self {
           #( #fields_conversion, )*
       }
   }
}
```
*/
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
pub(crate) mod cli_variant_struct;

/// describes logic of derive of [`clap::Parser`](https://docs.rs/clap/4.5.24/clap/trait.Parser.html) adapter
/// for `#name` struct, which returns instances of `#cli_name` struct
mod clap_parser_trait_adapter;

/// describes the derive of `impl From<#name> for #cli_name`
mod from_trait;
