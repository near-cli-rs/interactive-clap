derive of helper enum for structs with `#[interactive_clap(named_arg)]` on fields


```rust,ignore
struct #name {
    #[interactive_clap(named_arg)]
    ///Specify a sender
    field_name: Sender,
}
```

gets transformed
=> 

```rust,ignore
#[derive(Debug, Clone, clap::Parser, interactive_clap_derive::ToCliArgs)]
pub enum ClapNamedArgSenderFor#name {
    ///Specify a sender
    FieldName(<Sender as interactive_clap::ToCli>::CliVariant),
}
impl From<Sender> for ClapNamedArgSenderFor#name {
    fn from(item: Sender) -> Self {
        Self::FieldName(<Sender as interactive_clap::ToCli>::CliVariant::from(item))
    }
}
```

