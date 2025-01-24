This modules describes of `interactive_clap::FromCli` trait for `#name` struct,
which happens during derive of [`crate::InteractiveClap`] for `#name` struct:

derive input `#name`

```rust,ignore
struct #name {
    age: u64,
    first_name: String,
    second_name: String,
}
```

gets transformed
=> 

```rust,ignore
impl interactive_clap::FromCli for #name {
    type FromCliContext = ();
    type FromCliError = color_eyre::eyre::Error;
    fn from_cli(
        optional_clap_variant: Option<<Self as interactive_clap::ToCli>::CliVariant>,
        context: Self::FromCliContext,
    ) -> interactive_clap::ResultFromCli<
        <Self as interactive_clap::ToCli>::CliVariant,
        Self::FromCliError,
    >
    where
        Self: Sized + interactive_clap::ToCli,
    {
        let mut clap_variant = optional_clap_variant.clone().unwrap_or_default();
        if clap_variant.age.is_none() {
            clap_variant
                .age = match Self::input_age(&context) {
                Ok(Some(age)) => Some(age),
                Ok(None) => {
                    return interactive_clap::ResultFromCli::Cancel(Some(clap_variant));
                }
                Err(err) => {
                    return interactive_clap::ResultFromCli::Err(Some(clap_variant), err);
                }
            };
        }
        let age = clap_variant.age.clone().expect("Unexpected error");
        if clap_variant.first_name.is_none() {
            clap_variant
                .first_name = match Self::input_first_name(&context) {
                Ok(Some(first_name)) => Some(first_name),
                Ok(None) => {
                    return interactive_clap::ResultFromCli::Cancel(Some(clap_variant));
                }
                Err(err) => {
                    return interactive_clap::ResultFromCli::Err(Some(clap_variant), err);
                }
            };
        }
        let first_name = clap_variant.first_name.clone().expect("Unexpected error");
        let new_context_scope = InteractiveClapContextScopeForArgs {
            age: age.into(),
            first_name: first_name.into(),
        };
        interactive_clap::ResultFromCli::Ok(clap_variant)
    }
}
```
