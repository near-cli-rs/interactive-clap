extern crate proc_macro;

use proc_macro2::Span;
use proc_macro_error::abort_call_site;
use quote::quote;
use syn;

#[derive(Debug, Clone)]
pub struct InteractiveClapAttrsCliField {
    pub ident_field: syn::Ident,
    pub args_without_attrs: Option<proc_macro2::TokenStream>,
    pub named_args: Option<proc_macro2::TokenStream>,
    pub unnamed_args: Option<proc_macro2::TokenStream>,
    pub subcommand_args: Option<proc_macro2::TokenStream>,
}

impl InteractiveClapAttrsCliField {
    pub fn new(field: syn::Field) -> Self {
        let ident_field = field.ident.clone().expect("this field does not exist");
        let mut subcommand_args = quote! {
            let mut args = std::collections::VecDeque::new();
        };
        let mut args_without_attrs = quote!();
        let mut named_args = quote!();
        let mut unnamed_args = quote!();

        if field.attrs.is_empty() {
            args_without_attrs = quote! {
                if let Some(arg) = &self.#ident_field {
                    args.push_front(arg.to_string())
                }
            };
        } else {
            for attr in &field.attrs {
                if attr.path.is_ident("clap") {
                    for attr_token in attr.tokens.clone() {
                        match attr_token {
                            proc_macro2::TokenTree::Group(group) => {
                                for item in group.stream() {
                                    match &item {
                                        proc_macro2::TokenTree::Ident(ident) => {
                                            if *ident == *"subcommand" {
                                                subcommand_args = quote! {
                                                    let mut args = self
                                                    .#ident_field
                                                    .as_ref()
                                                    .map(|subcommand| subcommand.to_cli_args())
                                                    .unwrap_or_default();
                                                };
                                            } else if *ident == *"value_enum" {
                                                args_without_attrs = quote! {
                                                    if let Some(arg) = &self.#ident_field {
                                                        args.push_front(arg.to_string())
                                                    }
                                                };
                                            } else if *ident == *"long" {
                                                let ident_field_to_kebab_case_string =
                                                    crate::helpers::to_kebab_case::to_kebab_case(
                                                        ident_field.to_string(),
                                                    );
                                                let ident_field_to_kebab_case = &syn::LitStr::new(
                                                    &ident_field_to_kebab_case_string,
                                                    Span::call_site(),
                                                );
                                                unnamed_args = quote! {
                                                    if let Some(arg) = &self.#ident_field {
                                                        args.push_front(arg.to_string());
                                                        args.push_front(std::concat!("--", #ident_field_to_kebab_case).to_string());
                                                    }
                                                };
                                            }
                                        }
                                        proc_macro2::TokenTree::Literal(literal) => {
                                            named_args = quote! {
                                                if let Some(arg) = &self.#ident_field {
                                                    args.push_front(arg.to_string());
                                                    args.push_front(std::concat!("--", #literal).to_string());
                                                }
                                            };
                                        }
                                        _ => (), //abort_call_site!("Only option `TokenTree::Ident` is needed")
                                    };
                                }
                            }
                            _ => abort_call_site!("Only option `TokenTree::Group` is needed"),
                        }
                    }
                }
            }
        };
        Self {
            ident_field,
            args_without_attrs: Some(args_without_attrs),
            named_args: Some(named_args.clone()),
            unnamed_args: {
                if !named_args.is_empty() {
                    None
                } else {
                    Some(unnamed_args)
                }
            },
            subcommand_args: Some(subcommand_args),
        }
    }
}
