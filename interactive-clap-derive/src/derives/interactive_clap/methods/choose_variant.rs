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
    let mut actions_push_back = quote! {.chain([SelectVariantOrBack::Back])};
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
                syn::Fields::Unnamed(fields_unnamed) => {
                    let ty = &fields_unnamed.unnamed[0].ty;
                    cli_variant = quote! {
                        let cli_args =
                        match <#ty as interactive_clap::FromCli>::from_cli(None, context) {
                            interactive_clap::ResultFromCli::Ok(cli_args) => cli_args,
                            interactive_clap::ResultFromCli::Cancel(optional_cli_args) => {
                                return interactive_clap::ResultFromCli::Cancel(Some(#cli_command::#variant_ident(
                                    optional_cli_args.unwrap_or_default(),
                                )));
                            }
                            interactive_clap::ResultFromCli::Back => return interactive_clap::ResultFromCli::Back,
                            interactive_clap::ResultFromCli::Err(optional_cli_args, err) => {
                                return interactive_clap::ResultFromCli::Err(
                                    Some(#cli_command::#variant_ident(optional_cli_args.unwrap_or_default())),
                                    err,
                                );
                            }
                        };
                        interactive_clap::ResultFromCli::Ok(#cli_command::#variant_ident(cli_args))
                    }
                }
                syn::Fields::Unit => {
                    cli_variant = quote! {
                        interactive_clap::ResultFromCli::Ok(#cli_command::#variant_ident)
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
                        syn::Fields::Unnamed(fields_unnamed) => {
                            let ty = &fields_unnamed.unnamed[0].ty;
                            quote! {
                                #command_discriminants::#variant_ident => {
                                    let cli_args =
                                    match <#ty as interactive_clap::FromCli>::from_cli(None, context) {
                                        interactive_clap::ResultFromCli::Ok(cli_args) => cli_args,
                                        interactive_clap::ResultFromCli::Cancel(optional_cli_args) => {
                                            return interactive_clap::ResultFromCli::Cancel(Some(#cli_command::#variant_ident(
                                                optional_cli_args.unwrap_or_default(),
                                            )));
                                        }
                                        interactive_clap::ResultFromCli::Back => return interactive_clap::ResultFromCli::Back,
                                        interactive_clap::ResultFromCli::Err(optional_cli_args, err) => {
                                            return interactive_clap::ResultFromCli::Err(
                                                Some(#cli_command::#variant_ident(optional_cli_args.unwrap_or_default())),
                                                err,
                                            );
                                        }
                                    };
                                    #cli_command::#variant_ident(cli_args)
                                }
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
                    use interactive_clap::SelectVariantOrBack;
                    use inquire::Select;
                    use strum::{EnumMessage, IntoEnumIterator};
                    let selected_variant = Select::new(
                        #literal,
                        #command_discriminants::iter()
                            .map(SelectVariantOrBack::Variant)
                            #actions_push_back
                            .collect(),
                    )
                    .prompt();
                    match selected_variant {
                        Ok(SelectVariantOrBack::Variant(variant)) => interactive_clap::ResultFromCli::Ok(match variant {
                            #( #enum_variants, )*
                        }),
                        Ok(SelectVariantOrBack::Back) => interactive_clap::ResultFromCli::Back,
                        Err(
                            inquire::error::InquireError::OperationCanceled
                            | inquire::error::InquireError::OperationInterrupted,
                        ) => interactive_clap::ResultFromCli::Cancel(None),
                        Err(err) => interactive_clap::ResultFromCli::Err(None, err.into()),
                    }
                };
            }
        }
    };
    // let input_context = interactive_clap_attrs_context.get_input_context_dir();
    let context = match &interactive_clap_attrs_context.output_context_dir {
        Some(output_context_dir) => quote! {#output_context_dir},
        None => interactive_clap_attrs_context
            .clone()
            .get_input_context_dir(),
    };

    quote! {
        pub fn choose_variant(context: #context) -> interactive_clap::ResultFromCli<
        <Self as interactive_clap::ToCli>::CliVariant,
        <Self as interactive_clap::FromCli>::FromCliError,
    > {
        #cli_variant
    }
    }
}
