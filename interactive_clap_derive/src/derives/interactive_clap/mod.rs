extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_error::abort_call_site;
use quote::{quote, ToTokens};
use syn;

mod methods;

pub fn impl_interactive_clap(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let cli_name_string = format!("Cli{}", &ast.ident);
    let cli_name = &syn::Ident::new(&cli_name_string, Span::call_site());
    match &ast.data {
        syn::Data::Struct(data_struct) => {
            let fields = data_struct.fields.clone();
            let mut ident_skip_field_vec: Vec<syn::Ident> = Vec::new();

            let cli_fields = fields
                .iter()
                .map(|field| {
                    let ident_field = field.ident.clone().expect("this field does not exist");
                    let ty = &field.ty;
                    let cli_ty = self::methods::cli_field_type::cli_field_type(ty);
                    let mut cli_field = quote! {
                        pub #ident_field: #cli_ty
                    };
                    if field.attrs.is_empty() {
                        return cli_field;
                    };
                    let mut clap_attr_vec: Vec<proc_macro2::TokenStream> = Vec::new();
                    let mut cfg_attr_vec: Vec<proc_macro2::TokenStream> = Vec::new();
                    for attr in &field.attrs {
                        if attr.path.is_ident("interactive_clap") | attr.path.is_ident("cfg") {
                            for attr_token in attr.tokens.clone() {
                                match attr_token {
                                    proc_macro2::TokenTree::Group(group) => {
                                        if group.stream().to_string().contains("subcommand")
                                            | group.stream().to_string().contains("value_enum")
                                            | group.stream().to_string().contains("long")
                                            | (group.stream().to_string() == *"skip")
                                        {
                                            clap_attr_vec.push(group.stream())
                                        } else if group.stream().to_string().contains("named_arg") {
                                            let ident_subcommand =
                                                syn::Ident::new("subcommand", Span::call_site());
                                            clap_attr_vec.push(quote! {#ident_subcommand});
                                            let type_string = match ty {
                                                syn::Type::Path(type_path) => {
                                                    match type_path.path.segments.last() {
                                                        Some(path_segment) => {
                                                            path_segment.ident.to_string()
                                                        }
                                                        _ => String::new(),
                                                    }
                                                }
                                                _ => String::new(),
                                            };
                                            let enum_for_clap_named_arg = syn::Ident::new(
                                                &format!(
                                                    "ClapNamedArg{}For{}",
                                                    &type_string, &name
                                                ),
                                                Span::call_site(),
                                            );
                                            cli_field = quote! {
                                                pub #ident_field: Option<#enum_for_clap_named_arg>
                                            }
                                        };
                                        if group.stream().to_string().contains("feature") {
                                            cfg_attr_vec.push(attr.into_token_stream())
                                        };
                                        if group.stream().to_string() == *"skip" {
                                            ident_skip_field_vec.push(ident_field.clone());
                                            cli_field = quote!()
                                        };
                                    }
                                    _ => {
                                        abort_call_site!("Only option `TokenTree::Group` is needed")
                                    }
                                }
                            }
                        }
                    }
                    if cli_field.is_empty() {
                        return cli_field;
                    };
                    let cfg_attrs = cfg_attr_vec.iter();
                    if !clap_attr_vec.is_empty() {
                        let clap_attrs = clap_attr_vec.iter();
                        quote! {
                            #(#cfg_attrs)*
                            #[clap(#(#clap_attrs, )*)]
                            #cli_field
                        }
                    } else {
                        quote! {
                            #(#cfg_attrs)*
                            #cli_field
                        }
                    }
                })
                .filter(|token_stream| !token_stream.is_empty())
                .collect::<Vec<_>>();

            let for_cli_fields = fields
                .iter()
                .map(|field| for_cli_field(field, &ident_skip_field_vec))
                .filter(|token_stream| !token_stream.is_empty());

            let fn_from_cli_for_struct =
                self::methods::from_cli_for_struct::from_cli_for_struct(ast, &fields);

            let fn_get_arg = self::methods::get_arg_from_cli_for_struct::from_cli_arg(ast, &fields);

            let vec_fn_input_arg = self::methods::input_arg::vec_fn_input_arg(ast, &fields);

            let context_scope_fields = fields
                .iter()
                .map(context_scope_for_struct_field)
                .filter(|token_stream| !token_stream.is_empty())
                .collect::<Vec<_>>();
            let context_scope_for_struct = context_scope_for_struct(name, context_scope_fields);

            let clap_enum_for_named_arg =
                if let Some(token_stream) = fields.iter().find_map(|field| {
                    let ident_field = &field.clone().ident.expect("this field does not exist");
                    let variant_name_string = crate::helpers::snake_case_to_camel_case::snake_case_to_camel_case(ident_field.to_string());
                    let variant_name = &syn::Ident::new(&variant_name_string, Span::call_site());
                    let attr_doc_vec: Vec<_> = field.attrs.iter()
                        .filter(|attr| attr.path.is_ident("doc"))
                        .map(|attr| attr.into_token_stream())
                        .collect();
                    field.attrs.iter()
                        .filter(|attr| attr.path.is_ident("interactive_clap"))
                        .flat_map(|attr| attr.tokens.clone())
                        .filter(|attr_token| {
                            match attr_token {
                                proc_macro2::TokenTree::Group(group) => group.stream().to_string().contains("named_arg"),
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
                }) {
                    token_stream
                } else {
                    quote! ()
                };

            let gen = quote! {
                #[derive(Debug, Default, Clone, clap::Parser, interactive_clap_derive::ToCliArgs)]
                #[clap(author, version, about, long_about = None)]
                pub struct #cli_name {
                    #( #cli_fields, )*
                }

                impl interactive_clap::ToCli for #name {
                    type CliVariant = #cli_name;
                }

                #context_scope_for_struct

                #fn_from_cli_for_struct

                impl #name {
                    #(#fn_get_arg)*
                    #(#vec_fn_input_arg)*

                    fn try_parse() -> Result<#cli_name, clap::Error> {
                        <#cli_name as clap::Parser>::try_parse()
                    }

                    fn parse() -> #cli_name {
                        <#cli_name as clap::Parser>::parse()
                    }
                }

                impl From<#name> for #cli_name {
                    fn from(args: #name) -> Self {
                        Self {
                            #( #for_cli_fields, )*
                        }
                    }
                }

                #clap_enum_for_named_arg
            };
            gen.into()
        }
        syn::Data::Enum(syn::DataEnum { variants, .. }) => {
            let enum_variants = variants.iter().map(|variant| {
                let ident = &variant.ident;
                let mut attrs: Vec<proc_macro2::TokenStream> = Vec::new();
                if !&variant.attrs.is_empty() {
                    for attr in &variant.attrs {
                        if attr.path.is_ident("doc") {
                            attrs.push(attr.into_token_stream());
                            // break;
                        };
                        if attr.path.is_ident("cfg") {
                            for attr_token in attr.tokens.clone() {
                                match attr_token {
                                    proc_macro2::TokenTree::Group(group) => {
                                        if group.stream().to_string().contains("feature") {
                                            attrs.push(attr.into_token_stream());
                                        } else {
                                            continue;
                                        };
                                    }
                                    _ => {
                                        abort_call_site!("Only option `TokenTree::Group` is needed")
                                    }
                                }
                            }
                        };
                    }
                    match &variant.fields {
                        syn::Fields::Unnamed(fields) => {
                            let ty = &fields.unnamed[0].ty;
                            if attrs.is_empty() {
                                quote! {#ident(<#ty as interactive_clap::ToCli>::CliVariant)}
                            } else {
                                quote! {
                                    #(#attrs)*
                                    #ident(<#ty as interactive_clap::ToCli>::CliVariant)
                                }
                            }
                        }
                        syn::Fields::Unit => {
                            if attrs.is_empty() {
                                quote! {#ident}
                            } else {
                                quote! {
                                    #(#attrs)*
                                    #ident
                                }
                            }
                        }
                        _ => abort_call_site!(
                            "Only option `Fields::Unnamed` or `Fields::Unit` is needed"
                        ),
                    }
                } else {
                    match &variant.fields {
                        syn::Fields::Unnamed(fields) => {
                            let ty = &fields.unnamed[0].ty;
                            quote! { #ident(<#ty as interactive_clap::ToCli>::CliVariant) }
                        }
                        syn::Fields::Unit => {
                            quote! { #ident }
                        }
                        _ => abort_call_site!(
                            "Only option `Fields::Unnamed` or `Fields::Unit` is needed"
                        ),
                    }
                }
            });
            let for_cli_enum_variants = variants.iter().map(|variant| {
                let ident = &variant.ident;
                match &variant.fields {
                    syn::Fields::Unnamed(_) => {
                        quote! { #name::#ident(arg) => Self::#ident(arg.into()) }
                    }
                    syn::Fields::Unit => {
                        quote! { #name::#ident => Self::#ident }
                    }
                    _ => abort_call_site!(
                        "Only option `Fields::Unnamed` or `Fields::Unit` is needed"
                    ),
                }
            });

            let scope_for_enum = context_scope_for_enum(name);

            let fn_choose_variant = self::methods::choose_variant::fn_choose_variant(ast, variants);

            let fn_from_cli_for_enum =
                self::methods::from_cli_for_enum::from_cli_for_enum(ast, variants);

            let gen = quote! {
                #[derive(Debug, Clone, clap::Parser, interactive_clap_derive::ToCliArgs)]
                pub enum #cli_name {
                    #( #enum_variants, )*
                }

                impl interactive_clap::ToCli for #name {
                    type CliVariant = #cli_name;
                }

                #scope_for_enum

                impl From<#name> for #cli_name {
                    fn from(command: #name) -> Self {
                        match command {
                            #( #for_cli_enum_variants, )*
                        }
                    }
                }

                #fn_from_cli_for_enum

                impl #name {
                    #fn_choose_variant

                    fn try_parse() -> Result<#cli_name, clap::Error> {
                        <#cli_name as clap::Parser>::try_parse()
                    }

                    fn parse() -> #cli_name {
                        <#cli_name as clap::Parser>::parse()
                    }
                }
            };
            gen.into()
        }
        _ => abort_call_site!("`#[derive(InteractiveClap)]` only supports structs and enums"),
    }
}

fn context_scope_for_struct(
    name: &syn::Ident,
    context_scope_fields: Vec<proc_macro2::TokenStream>,
) -> proc_macro2::TokenStream {
    let interactive_clap_context_scope_for_struct = syn::Ident::new(
        &format!("InteractiveClapContextScopeFor{}", &name),
        Span::call_site(),
    );
    quote! {
        pub struct #interactive_clap_context_scope_for_struct {
            #(#context_scope_fields,)*
        }
        impl interactive_clap::ToInteractiveClapContextScope for #name {
            type InteractiveClapContextScope = #interactive_clap_context_scope_for_struct;
        }
    }
}

fn context_scope_for_struct_field(field: &syn::Field) -> proc_macro2::TokenStream {
    let ident_field = &field.ident.clone().expect("this field does not exist");
    let ty = &field.ty;
    if self::methods::fields_without_subcommand::is_field_without_subcommand(field) {
        quote! {
            pub #ident_field: #ty
        }
    } else {
        quote!()
    }
}

fn context_scope_for_enum(name: &syn::Ident) -> proc_macro2::TokenStream {
    let interactive_clap_context_scope_for_enum = syn::Ident::new(
        &format!("InteractiveClapContextScopeFor{}", &name),
        Span::call_site(),
    );
    let enum_discriminants = syn::Ident::new(&format!("{}Discriminants", &name), Span::call_site());
    quote! {
        pub type #interactive_clap_context_scope_for_enum = #enum_discriminants;
        impl interactive_clap::ToInteractiveClapContextScope for #name {
                    type InteractiveClapContextScope = #interactive_clap_context_scope_for_enum;
                }
    }
}

fn for_cli_field(
    field: &syn::Field,
    ident_skip_field_vec: &[syn::Ident],
) -> proc_macro2::TokenStream {
    let ident_field = &field.clone().ident.expect("this field does not exist");
    if ident_skip_field_vec.contains(ident_field) {
        quote!()
    } else {
        let ty = &field.ty;
        match &ty {
            syn::Type::Path(type_path) => match type_path.path.segments.first() {
                Some(path_segment) => {
                    if path_segment.ident.eq("Option") {
                        quote! {
                            #ident_field: args.#ident_field.into()
                        }
                    } else {
                        quote! {
                            #ident_field: Some(args.#ident_field.into())
                        }
                    }
                }
                _ => abort_call_site!("Only option `PathSegment` is needed"),
            },
            _ => abort_call_site!("Only option `Type::Path` is needed"),
        }
    }
}
