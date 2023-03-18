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

    let mut cli_variant = quote!();
    let mut ast_attrs: Vec<&str> = std::vec::Vec::new();

    if !ast.attrs.is_empty() {
        for attr in ast.attrs.clone() {
            if attr.path.is_ident("interactive_clap") {
                for attr_token in attr.tokens.clone() {
                    if let proc_macro2::TokenTree::Group(group) = attr_token {
                        if group.stream().to_string().contains("disable_back") {
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
                    #literal,
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
