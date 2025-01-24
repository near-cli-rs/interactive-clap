`interactive_clap::ToInteractiveClapContextScope` derive

This modules describes derive of `interactive_clap::ToInteractiveClapContextScope` trait for `#name` struct,
which happens during derive of [`crate::InteractiveClap`] for `#name` struct:

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
impl #name pub struct InteractiveClapContextScopeFor#name {
    pub age: u64,
    pub first_name: String,
}
impl interactive_clap::ToInteractiveClapContextScope for #name {
    type InteractiveClapContextScope = InteractiveClapContextScopeFor#name;
}
```
