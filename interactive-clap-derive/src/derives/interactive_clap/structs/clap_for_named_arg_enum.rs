/*!
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
*/
use proc_macro2::Span;
use proc_macro_error::abort_call_site;
use quote::{quote, ToTokens};
use syn;

/// returns the whole result `TokenStream` of derive logic of containing module
pub fn token_stream(ast: &syn::DeriveInput, fields: &syn::Fields) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    fields
        .iter()
        .find_map(|field| field_transform(name, field))
        .unwrap_or(quote!())
}

fn field_transform(name: &syn::Ident, field: &syn::Field) -> Option<proc_macro2::TokenStream> {
    let ident_field = &field.clone().ident.expect("this field does not exist");
    let variant_name_string =
        crate::helpers::snake_case_to_camel_case::snake_case_to_camel_case(ident_field.to_string());
    let variant_name = &syn::Ident::new(&variant_name_string, Span::call_site());
    let attr_doc_vec: Vec<_> = field
        .attrs
        .iter()
        .filter(|attr| attr.path.is_ident("doc"))
        .map(|attr| attr.into_token_stream())
        .collect();
    field.attrs.iter()
        .filter(|attr| attr.path.is_ident("interactive_clap"))
        .flat_map(|attr| attr.tokens.clone())
        .filter(|attr_token| {
            match attr_token {
                proc_macro2::TokenTree::Group(group) => group.stream().to_string() == *"named_arg",
                _ => abort_call_site!("Only option `TokenTree::Group` is needed")
            }
        })
        .map(|_| {
            let ty = &field.ty;
            let type_string = match ty {
                syn::Type::Path(type_path) => {
                    match type_path.path.segments.last() {
                        Some(path_segment) => path_segment.ident.to_string(),
                        _ => String::new()
                    }
                },
                _ => String::new()
            };
            let enum_for_clap_named_arg = syn::Ident::new(&format!("ClapNamedArg{}For{}", &type_string, &name), Span::call_site());
            quote! {
                #[derive(Debug, Clone, clap::Parser, interactive_clap_derive::ToCliArgs)]
                pub enum #enum_for_clap_named_arg {
                    #(#attr_doc_vec)*
                    #variant_name(<#ty as interactive_clap::ToCli>::CliVariant)
                }

                impl From<#ty> for #enum_for_clap_named_arg {
                    fn from(item: #ty) -> Self {
                        Self::#variant_name(<#ty as interactive_clap::ToCli>::CliVariant::from(item))
                    }
                }
            }
        })
        .next()
}
