extern crate proc_macro;

use proc_macro2::Span;
use proc_macro_error::abort_call_site;
use quote::{quote, ToTokens};
use syn;

use super::common_methods as structs_methods;
use crate::derives::interactive_clap::common_methods;

/// returns the whole result `TokenStream` of derive logic of containing module
pub fn token_stream(ast: &syn::DeriveInput, fields: &syn::Fields) -> proc_macro2::TokenStream {
    let name = &ast.ident;

    let interactive_clap_attrs_context =
        common_methods::interactive_clap_attrs_context::InteractiveClapAttrsContext::new(ast);
    if interactive_clap_attrs_context.is_skip_default_from_cli {
        return quote!();
    };

    let fields_without_subcommand_and_subargs = fields
        .iter()
        .filter(|field| {
            !structs_methods::is_field_with_subcommand::predicate(field)
                && !common_methods::fields_with_subargs::is_field_with_subargs(field)
        })
        .map(|field| {
            let ident_field = &field.clone().ident.expect("this field does not exist");
            quote! {#ident_field: #ident_field.into()}
        })
        .collect::<Vec<_>>();

    let fields_value = fields
        .iter()
        .map(fields_value)
        .filter(|token_stream| !token_stream.is_empty());

    let field_value_named_arg = fields
        .iter()
        .map(|field| field_value_named_arg(name, field))
        .find(|token_stream| !token_stream.is_empty())
        .unwrap_or(quote!());

    let field_value_subcommand = fields
        .iter()
        .map(field_value_subcommand)
        .find(|token_stream| !token_stream.is_empty())
        .unwrap_or(quote!());

    let field_value_subargs = fields
        .iter()
        .map(field_value_subargs)
        .find(|token_stream| !token_stream.is_empty())
        .unwrap_or(quote!());

    let input_context_dir = interactive_clap_attrs_context
        .clone()
        .get_input_context_dir();

    let interactive_clap_context_scope_for_struct = syn::Ident::new(
        &format!("InteractiveClapContextScopeFor{}", &name),
        Span::call_site(),
    );
    let new_context_scope = quote! {
        let new_context_scope = #interactive_clap_context_scope_for_struct { #(#fields_without_subcommand_and_subargs,)* };
    };

    let output_context = match &interactive_clap_attrs_context.output_context_dir {
        Some(output_context_dir) => {
            quote! {
                let output_context = match #output_context_dir::from_previous_context(context.clone(), &new_context_scope) {
                    Ok(new_context) => new_context,
                    Err(err) => return interactive_clap::ResultFromCli::Err(Some(clap_variant), err),
                };
                let context = output_context;
            }
        }
        None => quote!(),
    };

    quote! {
        impl interactive_clap::FromCli for #name {
            type FromCliContext = #input_context_dir;
            type FromCliError = color_eyre::eyre::Error;
            fn from_cli(
                optional_clap_variant: Option<<Self as interactive_clap::ToCli>::CliVariant>,
                context: Self::FromCliContext,
            ) -> interactive_clap::ResultFromCli<<Self as interactive_clap::ToCli>::CliVariant, Self::FromCliError> where Self: Sized + interactive_clap::ToCli {
                let mut clap_variant = optional_clap_variant.clone().unwrap_or_default();
                #(#fields_value)*
                #new_context_scope
                #output_context
                #field_value_subargs
                #field_value_named_arg
                #field_value_subcommand;
                interactive_clap::ResultFromCli::Ok(clap_variant)
            }
        }
    }
}

fn fields_value(field: &syn::Field) -> proc_macro2::TokenStream {
    let ident_field = &field.clone().ident.expect("this field does not exist");
    let fn_input_arg = syn::Ident::new(&format!("input_{}", &ident_field), Span::call_site());
    if field.ty.to_token_stream().to_string() == "bool"
        || common_methods::skip_interactive_input::is_skip_interactive_input(field)
    {
        quote! {
            let #ident_field = clap_variant.#ident_field.clone();
        }
    } else if field
        .ty
        .to_token_stream()
        .to_string()
        .starts_with("Option <")
    {
        quote! {
            if clap_variant.#ident_field.is_none() {
                clap_variant
                    .#ident_field = match Self::#fn_input_arg(&context) {
                    Ok(optional_field) => optional_field,
                    Err(err) => return interactive_clap::ResultFromCli::Err(Some(clap_variant), err),
                };
            };
            let #ident_field = clap_variant.#ident_field.clone();
        }
    } else if !structs_methods::is_field_with_subcommand::predicate(field)
        && !common_methods::fields_with_subargs::is_field_with_subargs(field)
    {
        quote! {
            if clap_variant.#ident_field.is_none() {
                clap_variant
                    .#ident_field = match Self::#fn_input_arg(&context) {
                    Ok(Some(#ident_field)) => Some(#ident_field),
                    Ok(None) => return interactive_clap::ResultFromCli::Cancel(Some(clap_variant)),
                    Err(err) => return interactive_clap::ResultFromCli::Err(Some(clap_variant), err),
                };
            };
            let #ident_field = clap_variant.#ident_field.clone().expect("Unexpected error");
        }
    } else {
        quote!()
    }
}

fn field_value_named_arg(name: &syn::Ident, field: &syn::Field) -> proc_macro2::TokenStream {
    let ident_field = &field.clone().ident.expect("this field does not exist");
    let ty = &field.ty;
    if field.attrs.is_empty() {
        quote!()
    } else {
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
                let variant_name_string = crate::helpers::snake_case_to_camel_case::snake_case_to_camel_case(ident_field.to_string());
                let variant_name = &syn::Ident::new(&variant_name_string, Span::call_site());
                quote! {
                    let optional_field = match clap_variant.#ident_field.take() {
                        Some(#enum_for_clap_named_arg::#variant_name(cli_arg)) => Some(cli_arg),
                        None => None,
                    };
                    match <#ty as interactive_clap::FromCli>::from_cli(
                        optional_field,
                        context.into(),
                    ) {
                        interactive_clap::ResultFromCli::Ok(cli_field) => {
                            clap_variant.#ident_field = Some(#enum_for_clap_named_arg::#variant_name(cli_field));
                        }
                        interactive_clap::ResultFromCli::Cancel(optional_cli_field) => {
                            clap_variant.#ident_field = optional_cli_field.map(#enum_for_clap_named_arg::#variant_name);
                            return interactive_clap::ResultFromCli::Cancel(Some(clap_variant));
                        }
                        interactive_clap::ResultFromCli::Back => return interactive_clap::ResultFromCli::Back,
                        interactive_clap::ResultFromCli::Err(optional_cli_field, err) => {
                            clap_variant.#ident_field = optional_cli_field.map(#enum_for_clap_named_arg::#variant_name);
                            return interactive_clap::ResultFromCli::Err(Some(clap_variant), err);
                        }
                    }
                }
            })
            .next()
            .unwrap_or(quote!())
    }
}

fn field_value_subcommand(field: &syn::Field) -> proc_macro2::TokenStream {
    let ident_field = &field.clone().ident.expect("this field does not exist");
    let ty = &field.ty;
    if field.attrs.is_empty() {
        quote!()
    } else {
        field.attrs.iter()
            .filter(|attr| attr.path.is_ident("interactive_clap"))
            .flat_map(|attr| attr.tokens.clone())
            .filter(|attr_token| {
                match attr_token {
                    proc_macro2::TokenTree::Group(group) => group.stream().to_string().contains("subcommand"),
                    _ => abort_call_site!("Only option `TokenTree::Group` is needed")
                }
            })
            .map(|_| {
                quote! {
                    match <#ty as interactive_clap::FromCli>::from_cli(clap_variant.#ident_field.take(), context.into()) {
                        interactive_clap::ResultFromCli::Ok(cli_field) => {
                            clap_variant.#ident_field = Some(cli_field);
                        }
                        interactive_clap::ResultFromCli::Cancel(option_cli_field) => {
                            clap_variant.#ident_field = option_cli_field;
                            return interactive_clap::ResultFromCli::Cancel(Some(clap_variant));
                        }
                        interactive_clap::ResultFromCli::Cancel(option_cli_field) => {
                            clap_variant.#ident_field = option_cli_field;
                            return interactive_clap::ResultFromCli::Cancel(Some(clap_variant));
                        }
                        interactive_clap::ResultFromCli::Back => return interactive_clap::ResultFromCli::Back,
                        interactive_clap::ResultFromCli::Err(option_cli_field, err) => {
                            clap_variant.#ident_field = option_cli_field;
                            return interactive_clap::ResultFromCli::Err(Some(clap_variant), err);
                        }
                    }
                }
            })
            .next()
            .unwrap_or(quote!())
    }
}

fn field_value_subargs(field: &syn::Field) -> proc_macro2::TokenStream {
    let ident_field = &field.clone().ident.expect("this field does not exist");
    let ty = &field.ty;
    if field.attrs.is_empty() {
        quote!()
    } else {
        field.attrs.iter()
            .filter(|attr| attr.path.is_ident("interactive_clap"))
            .flat_map(|attr| attr.tokens.clone())
            .filter(|attr_token| {
                match attr_token {
                    proc_macro2::TokenTree::Group(group) => group.stream().to_string().contains("subargs"),
                    _ => abort_call_site!("Only option `TokenTree::Group` is needed")
                }
            })
            .map(|_| {
                quote! {
                    match #ty::from_cli(
                        optional_clap_variant.unwrap_or_default().#ident_field,
                        context.into(),
                    ) {
                        interactive_clap::ResultFromCli::Ok(cli_field) => clap_variant.#ident_field = Some(cli_field),
                        interactive_clap::ResultFromCli::Cancel(optional_cli_field) => {
                            clap_variant.#ident_field = optional_cli_field;
                            return interactive_clap::ResultFromCli::Cancel(Some(clap_variant));
                        }
                        interactive_clap::ResultFromCli::Back => return interactive_clap::ResultFromCli::Back,
                        interactive_clap::ResultFromCli::Err(optional_cli_field, err) => {
                            clap_variant.#ident_field = optional_cli_field;
                            return interactive_clap::ResultFromCli::Err(Some(clap_variant), err);
                        }
                    };
                }
            })
            .next()
            .unwrap_or(quote!())
    }
}
