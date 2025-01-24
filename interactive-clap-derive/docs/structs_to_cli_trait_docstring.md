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
