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
        super::interactive_clap_attrs_context::InteractiveClapAttrsContext::new(ast);
    let command_discriminants =
        syn::Ident::new(&format!("{}Discriminants", name), Span::call_site());
    let cli_command = syn::Ident::new(&format!("Cli{}", name), Span::call_site());

    let variant_ident = &variants[0].ident;
    let mut cli_variant = quote!();
    let mut actions_push_back = quote! {actions.push("back".to_string())};
    let mut ast_attrs: Vec<&str> = std::vec::Vec::new();

    if !ast.attrs.is_empty() {
        for attr in ast.attrs.clone() {
            if attr.path.is_ident("interactive_clap") {
                for attr_token in attr.tokens.clone() {
                    if let proc_macro2::TokenTree::Group(group) = attr_token {
                        if group
                            .stream()
                            .to_string()
                            .contains("disable_strum_discriminants")
                        {
                            ast_attrs.push("disable_strum_discriminants");
                        } else if group.stream().to_string().contains("disable_back") {
                            ast_attrs.push("disable_back");
                        };
                    }
                }
            };
            if attr.path.is_ident("strum_discriminants") {
                for attr_token in attr.tokens.clone() {
                    if let proc_macro2::TokenTree::Group(group) = attr_token {
                        if &group.stream().to_string() == "derive(EnumMessage, EnumIter)" {
                            ast_attrs.push("strum_discriminants");
                        };
                    }
                }
            };
        }
        if ast_attrs.contains(&"disable_strum_discriminants") {
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
                _ => abort_call_site!("Only option `Fields::Unnamed` or `Fields::Unit` is needed"),
            }
        } else {
            if ast_attrs.contains(&"disable_back") {
                actions_push_back = quote!();
            }
            if ast_attrs.contains(&"strum_discriminants") {
                let doc_attrs = ast
                    .attrs
                    .iter()
                    .filter(|attr| attr.path.is_ident("doc"))
                    .map(|attr| {
                        let mut literal_string = String::new();
                        for attr_token in attr.tokens.clone() {
                            if let proc_macro2::TokenTree::Literal(literal) = attr_token {
                                literal_string = literal.to_string();
                            }
                        }
                        literal_string
                    })
                    .collect::<Vec<_>>();
                let literal_vec = doc_attrs
                    .iter()
                    .map(|s| s.replace('\"', ""))
                    .collect::<Vec<_>>();
                let literal = proc_macro2::Literal::string(literal_vec.join("\n  ").as_str());

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
                    use inquire::Select;
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
                        #actions_push_back;

                        let selected = Select::new(prompt, actions.clone())
                        .prompt()
                        .unwrap();
                        let mut selected_index: usize = 0;
                        for (i, item) in actions.iter().enumerate() {
                            if item == &selected {
                                selected_index = i;
                                break;
                            }
                        };
                        variants.get(selected_index).cloned()
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
            }
        }
    };
    let input_context = interactive_clap_attrs_context.get_input_context_dir();

    quote! {
        pub fn choose_variant(context: #input_context) -> color_eyre::eyre::Result<Option<Self>> {
            loop {
                #cli_variant
                if let Some(variant) = <Self as interactive_clap::FromCli>::from_cli(Some(cli_variant), context.clone())? {
                    return Ok(Some(variant));
                }
            }
        }
    }
}
