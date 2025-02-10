extern crate proc_macro;

use proc_macro2::Span;
use proc_macro_error::abort_call_site;
use quote::quote;
use syn;

pub fn fn_choose_variant(
    ast: &syn::DeriveInput,
    variants: &syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>,
) -> proc_macro2::TokenStream {
    dbg_cond!("entered `fn_choose_variant`");
    let name = &ast.ident;
    let interactive_clap_attrs_context =
        super::interactive_clap_attrs_context::InteractiveClapAttrsContext::new(ast);
    let command_discriminants = syn::Ident::new(&format!("{name}Discriminants"), Span::call_site());
    let cli_command = syn::Ident::new(&format!("Cli{name}"), Span::call_site());

    let mut cli_variant = quote!();
    let mut ast_attrs: Vec<&str> = std::vec::Vec::new();

    if !ast.attrs.is_empty() {
        for (_index, attr) in ast.attrs.clone().into_iter().enumerate() {
            dbg_cond!((_index, &attr));
            if attr.path.is_ident("interactive_clap") {
                for attr_token in attr.tokens.clone() {
                    if let proc_macro2::TokenTree::Group(group) = attr_token {
                        if group.stream().to_string().contains("disable_back") {
                            ast_attrs.push("disable_back");
                        };
                    }
                }
            };
            dbg_cond!(attr.path.is_ident("strum_discriminants"));
            if attr.path.is_ident("strum_discriminants") {
                for attr_token in attr.tokens.clone() {
                    if let proc_macro2::TokenTree::Group(group) = attr_token {
                        let group_stream_no_whitespace =
                            group.stream().to_string().replace(" ", "");
                        dbg_cond!(&group_stream_no_whitespace);
                        if &group_stream_no_whitespace == "derive(EnumMessage,EnumIter)" {
                            ast_attrs.push("strum_discriminants");
                        };
                    }
                }
            };
        }
        dbg_cond!(&ast_attrs);
        if ast_attrs.contains(&"strum_discriminants") {
            let doc_attrs = ast
                .attrs
                .iter()
                .filter(|attr| attr.path.is_ident("doc"))
                .filter_map(|attr| {
                    for attr_token in attr.tokens.clone() {
                        if let proc_macro2::TokenTree::Literal(literal) = attr_token {
                            return Some(literal);
                        }
                    }
                    None
                });

            let enum_variants = variants.iter().map(|variant| {
                let variant_ident = &variant.ident;
                match &variant.fields {
                    syn::Fields::Unnamed(_) => {
                        quote! {
                            #command_discriminants::#variant_ident => {
                                #cli_command::#variant_ident(Default::default())
                            }
                        }
                    }
                    syn::Fields::Unit => {
                        quote! {
                            #command_discriminants::#variant_ident => #cli_command::#variant_ident
                        }
                    }
                    _ => abort_call_site!(
                        "Only option `Fields::Unnamed` or `Fields::Unit` is needed"
                    ),
                }
            });
            let actions_push_back = if ast_attrs.contains(&"disable_back") {
                quote!()
            } else {
                quote! {.chain([SelectVariantOrBack::Back])}
            };

            cli_variant = quote! {
                use interactive_clap::SelectVariantOrBack;
                use inquire::Select;
                use strum::{EnumMessage, IntoEnumIterator};

                let selected_variant = Select::new(
                    concat!(#( #doc_attrs, )*).trim(),
                    #command_discriminants::iter()
                        .map(SelectVariantOrBack::Variant)
                        #actions_push_back
                        .collect(),
                )
                .prompt();
                match selected_variant {
                    Ok(SelectVariantOrBack::Variant(variant)) => {
                        let cli_args = match variant {
                            #( #enum_variants, )*
                        };
                        return interactive_clap::ResultFromCli::Ok(cli_args);
                    },
                    Ok(SelectVariantOrBack::Back) => return interactive_clap::ResultFromCli::Back,
                    Err(
                        inquire::error::InquireError::OperationCanceled
                        | inquire::error::InquireError::OperationInterrupted,
                    ) => return interactive_clap::ResultFromCli::Cancel(None),
                    Err(err) => return interactive_clap::ResultFromCli::Err(None, err.into()),
                }
            };
        }
    };
    let context = interactive_clap_attrs_context.get_input_context_dir();

    quote! {
        pub fn choose_variant(context: #context) -> interactive_clap::ResultFromCli<
        <Self as interactive_clap::ToCli>::CliVariant,
        <Self as interactive_clap::FromCli>::FromCliError,
    > {
        #cli_variant
    }
    }
}
