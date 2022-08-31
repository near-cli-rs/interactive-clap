extern crate proc_macro;

use proc_macro2::Span;
use proc_macro_error::abort_call_site;
use quote::quote;
use syn;

pub fn fn_choose_variant(
    ast: &syn::DeriveInput,
    variants: &syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>,
) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let interactive_clap_attrs_context =
        super::interactive_clap_attrs_context::InteractiveClapAttrsContext::new(&ast);
    let command_discriminants =
        syn::Ident::new(&format!("{}Discriminants", name), Span::call_site());
    let cli_command = syn::Ident::new(&format!("Cli{}", name), Span::call_site());

    let variant_ident = &variants[0].ident;
    let mut cli_variant = quote!();

    if !ast.attrs.is_empty() {
        for attr in ast.attrs.clone() {
            if attr.path.is_ident("interactive_clap".into()) {
                for attr_token in attr.tokens.clone() {
                    match attr_token {
                        proc_macro2::TokenTree::Group(group) => {
                            if group
                                .stream()
                                .to_string()
                                .contains("disable_strum_discriminants")
                                .clone()
                            {
                                match &variants[0].fields {
                                    syn::Fields::Unnamed(_) => {
                                        cli_variant = quote! {
                                            let cli_variant = #cli_command::#variant_ident(Default::default());
                                        };
                                    }
                                    syn::Fields::Unit => {
                                        cli_variant = quote! {
                                            let cli_variant = #cli_command::#variant_ident;
                                        };
                                    }
                                    _ => abort_call_site!(
                                        "Only option `Fields::Unnamed` or `Fields::Unit` is needed"
                                    ),
                                }
                            };
                        }
                        _ => (), //abort_call_site!("Only option `TokenTree::Group` is needed")
                    }
                }
            };
            if attr.path.is_ident("strum_discriminants".into()) {
                for attr_token in attr.tokens.clone() {
                    match attr_token {
                        proc_macro2::TokenTree::Group(group) => {
                            if &group.stream().to_string() == "derive(EnumMessage, EnumIter)" {
                                let doc_attrs = ast
                                    .attrs
                                    .iter()
                                    .filter(|attr| attr.path.is_ident("doc".into()))
                                    .map(|attr| {
                                        let mut literal_string = String::new();
                                        for attr_token in attr.tokens.clone() {
                                            match attr_token {
                                                proc_macro2::TokenTree::Literal(literal) => {
                                                    literal_string = literal.to_string();
                                                }
                                                _ => (), //abort_call_site!("Only option `TokenTree::Literal` is needed")
                                            }
                                        }
                                        literal_string
                                    })
                                    .collect::<Vec<_>>();
                                let literal_vec = doc_attrs
                                    .iter()
                                    .map(|s| s.replace("\"", ""))
                                    .collect::<Vec<_>>();
                                let literal =
                                    proc_macro2::Literal::string(literal_vec.join("\n  ").as_str());

                                let enum_variants = variants.iter().map(|variant| {
                                    let variant_ident = &variant.ident;

                                    match &variant.fields {
                                        syn::Fields::Unnamed(_) => {
                                            quote! {
                                                #command_discriminants::#variant_ident => #cli_command::#variant_ident(Default::default())
                                            }
                                        },
                                        syn::Fields::Unit => {
                                            quote! {
                                                #command_discriminants::#variant_ident => #cli_command::#variant_ident
                                            }
                                        },
                                        _ => abort_call_site!("Only option `Fields::Unnamed` or `Fields::Unit` is needed")
                                    }
                                });

                                cli_variant = quote! {
                                    use dialoguer::{theme::ColorfulTheme, Select};
                                    use strum::{EnumMessage, IntoEnumIterator};
                                    fn prompt_variant<T>(prompt: &str) -> Option<T>
                                    where
                                    T: IntoEnumIterator + EnumMessage,
                                    T: Copy + Clone,
                                    {
                                        let variants = T::iter().collect::<Vec<_>>();
                                        let mut actions = variants
                                        .iter()
                                        .map(|p| {
                                            p.get_message()
                                            .unwrap_or_else(|| "error[This entry does not have an option message!!]")
                                            .to_owned()
                                        })
                                        .collect::<Vec<_>>();
                                        actions.push("back".to_string());

                                        let selected = Select::with_theme(&ColorfulTheme::default())
                                        .with_prompt(prompt)
                                        .items(&actions)
                                        .default(0)
                                        .interact()
                                        .unwrap();

                                        variants.get(selected).cloned()
                                    };
                                    let variant = if let Some(variant) = prompt_variant(#literal.to_string().as_str()) {
                                        variant
                                    } else {
                                        return Ok(None);
                                    };
                                    let cli_variant = match variant {
                                        #( #enum_variants, )*
                                    };
                                };
                            };
                        }
                        _ => (), //abort_call_site!("Only option `TokenTree::Group` is needed")
                    }
                }
            };
        }
    };
    let input_context = interactive_clap_attrs_context.get_input_context_dir();

    quote! {
        pub fn choose_variant(context: #input_context) -> color_eyre::eyre::Result<Option<Self>> {
            loop {
                #cli_variant
                if let Some(variant) = Self::from_cli(Some(cli_variant), context.clone())? {
                    return Ok(Some(variant));
                }
            }
        }
    }
}
