per-field input with [inquire::CustomType](https://docs.rs/inquire/0.6.2/inquire/struct.CustomType.html) impl block

This modules describes derive of input args implementation block for `#name` struct,
which contains functions `input_#field_ident` per each field,
which prompt for value of each field via [inquire::CustomType](https://docs.rs/inquire/0.6.2/inquire/struct.CustomType.html)
, which happens during derive of [`crate::InteractiveClap`] for `#name` struct:

derive input `#name`

```rust,ignore
struct #name {
    age: u64,
    first_name: String,
}
```


gets transformed
=> 

```rust,ignore
impl #name {
    fn input_age(_context: &()) -> color_eyre::eyre::Result<Option<u64>> {
        match inquire::CustomType::new("age").prompt() {
            Ok(value) => Ok(Some(value)),
            Err(
                inquire::error::InquireError::OperationCanceled
                | inquire::error::InquireError::OperationInterrupted,
            ) => Ok(None),
            Err(err) => Err(err.into()),
        }
    }
    fn input_first_name(_context: &()) -> color_eyre::eyre::Result<Option<String>> {
        match inquire::CustomType::new("first_name").prompt() {
            Ok(value) => Ok(Some(value)),
            Err(
                inquire::error::InquireError::OperationCanceled
                | inquire::error::InquireError::OperationInterrupted,
            ) => Ok(None),
            Err(err) => Err(err.into()),
        }
    }
}
```
