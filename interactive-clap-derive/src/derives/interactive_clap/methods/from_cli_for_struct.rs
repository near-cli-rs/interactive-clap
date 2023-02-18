extern crate proc_macro;

use proc_macro2::Span;
use proc_macro_error::abort_call_site;
use quote::quote;
use syn;

pub fn from_cli_for_struct(
    ast: &syn::DeriveInput,
    fields: &syn::Fields,
) -> proc_macro2::TokenStream {
    let name = &ast.ident;

    let interactive_clap_attrs_context =
        super::interactive_clap_attrs_context::InteractiveClapAttrsContext::new(ast);
    if interactive_clap_attrs_context.is_skip_default_from_cli {
        return quote!();
    };

    let fields_without_subcommand = fields
        .iter()
        .filter(|field| super::fields_without_subcommand::is_field_without_subcommand(field))
        .map(|field| {
            let ident_field = &field.clone().ident.expect("this field does not exist");
            quote! {#ident_field}
        })
        .collect::<Vec<_>>();

    let fields_value = fields
        .iter()
        .map(fields_value)
        .filter(|token_stream| !token_stream.is_empty());

    let field_value_named_arg = if let Some(token_stream) = fields
        .iter()
        .map(|field| {
            field_value_named_arg(
                name,
                field,
                &interactive_clap_attrs_context.output_context_dir,
            )
        })
        .find(|token_stream| !token_stream.is_empty())
    {
        token_stream
    } else {
        quote!()
    };

    let field_value_subcommand = if let Some(token_stream) = fields
        .iter()
        .map(|field| {
            field_value_subcommand(
                name,
                field,
                &interactive_clap_attrs_context.output_context_dir,
            )
        })
        .find(|token_stream| !token_stream.is_empty())
    {
        token_stream
    } else {
        quote!()
    };

    let struct_fields = fields
        .iter()
        .map(|field| struct_field(field, &fields_without_subcommand));

    let input_context_dir = interactive_clap_attrs_context.get_input_context_dir();

    let interactive_clap_context_scope_for_struct = syn::Ident::new(
        &format!("InteractiveClapContextScopeFor{}", &name),
        Span::call_site(),
    );
    let new_context_scope = quote! {
        let new_context_scope = #interactive_clap_context_scope_for_struct { #(#fields_without_subcommand,)* };
    };

    quote! {
        impl interactive_clap::FromCli for #name {
            type FromCliContext = #input_context_dir;
            type FromCliError = color_eyre::eyre::Error;
            fn from_cli(
                optional_clap_variant: Option<<Self as interactive_clap::ToCli>::CliVariant>,
                context: Self::FromCliContext,
            ) -> Result<Option<Self>, Self::FromCliError> where Self: Sized + interactive_clap::ToCli {
                #(#fields_value)*
                #new_context_scope
                #field_value_named_arg
                #field_value_subcommand;
                Ok(Some(Self{ #(#struct_fields,)* }))
            }
        }
    }
}

fn fields_value(field: &syn::Field) -> proc_macro2::TokenStream {
    let ident_field = &field.clone().ident.expect("this field does not exist");
    let fn_from_cli_arg = syn::Ident::new(&format!("from_cli_{}", &ident_field), Span::call_site());
    if super::fields_without_subcommand::is_field_without_subcommand(field) {
        let type_string = match &field.ty {
            syn::Type::Path(type_path) => match type_path.path.segments.last() {
                Some(path_segment) => path_segment.ident.to_string(),
                _ => String::new(),
            },
            _ => String::new(),
        };
        if let "Option" = type_string.as_str() {
            quote! {
                let #ident_field = optional_clap_variant
                    .as_ref()
                    .and_then(|clap_variant| clap_variant.#ident_field.clone());
            }
        } else {
            quote! {
                let #ident_field = Self::#fn_from_cli_arg(
                    optional_clap_variant
                        .clone()
                        .and_then(|clap_variant| clap_variant.#ident_field),
                        &context,
                )?;
            }
        }
    } else {
        quote!()
    }
}

fn field_value_named_arg(
    name: &syn::Ident,
    field: &syn::Field,
    output_context_dir: &Option<proc_macro2::TokenStream>,
) -> proc_macro2::TokenStream {
    let ident_field = &field.clone().ident.expect("this field does not exist");
    let ty = &field.ty;
    if field.attrs.is_empty() {
        quote!()
    } else {
        match field.attrs.iter()
        .filter(|attr| attr.path.is_ident("interactive_clap"))
        .flat_map(|attr| attr.tokens.clone())
        .filter(|attr_token| {
            match attr_token {
                proc_macro2::TokenTree::Group(group) => group.stream().to_string().contains("named_arg"), 
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
            match output_context_dir {
                Some(_) => {
                    let context_for_struct = syn::Ident::new(&format!("{}Context", &name), Span::call_site());
                    quote! {
                        let new_context = #context_for_struct::from_previous_context(context, &new_context_scope)?;
                        let #ident_field = <#ty as interactive_clap::FromCli>::from_cli(
                            optional_clap_variant.and_then(|clap_variant| match clap_variant.#ident_field {
                                Some(#enum_for_clap_named_arg::#variant_name(cli_arg)) => Some(cli_arg),
                                None => None,
                            }),
                            new_context.into(),
                        )?;
                        let #ident_field = if let Some(value) = #ident_field {
                            value
                        } else {
                            return Ok(None);
                        }
                    }
                },
                None => quote! {
                    let #ident_field = <#ty as interactive_clap::FromCli>::from_cli(
                        optional_clap_variant.and_then(|clap_variant| match clap_variant.#ident_field {
                            Some(#enum_for_clap_named_arg::#variant_name(cli_sender)) => Some(cli_sender),
                            None => None,
                        }),
                        context.into(),
                    )?;
                    let #ident_field = if let Some(value) = #ident_field {
                        value
                    } else {
                        return Ok(None);
                    }
                }
            }
        })
        .next() {
            Some(token_stream) => token_stream,
            None => quote! ()
        }
    }
}

fn field_value_subcommand(
    name: &syn::Ident,
    field: &syn::Field,
    output_context_dir: &Option<proc_macro2::TokenStream>,
) -> proc_macro2::TokenStream {
    let ident_field = &field.clone().ident.expect("this field does not exist");
    let ty = &field.ty;
    if field.attrs.is_empty() {
        quote!()
    } else {
        match field.attrs.iter()
        .filter(|attr| attr.path.is_ident("interactive_clap"))
        .flat_map(|attr| attr.tokens.clone())
        .filter(|attr_token| {
            match attr_token {
                proc_macro2::TokenTree::Group(group) => group.stream().to_string().contains("subcommand"),
                _ => abort_call_site!("Only option `TokenTree::Group` is needed")
            }
        })
        .map(|_| {
            match output_context_dir {
                Some(_) => {
                    let context_for_struct = syn::Ident::new(&format!("{}Context", &name), Span::call_site());
                    quote! {
                        let new_context = #context_for_struct::from_previous_context(context, &new_context_scope)?;
                        let #ident_field = match optional_clap_variant.and_then(|clap_variant| clap_variant.#ident_field) {
                            Some(cli_arg) => <#ty as interactive_clap::FromCli>::from_cli(Some(cli_arg), new_context.into())?,
                            None => #ty::choose_variant(new_context.into())?,
                        };
                        let #ident_field = if let Some(value) = #ident_field {
                            value
                        } else {
                            return Ok(None);
                        }
                    }
                },
                None => quote! {
                    let #ident_field = match optional_clap_variant.and_then(|clap_variant| clap_variant.#ident_field) {
                        Some(cli_arg) => <#ty as interactive_clap::FromCli>::from_cli(Some(cli_arg), context)?,
                        None => #ty::choose_variant(context.into())?,
                    };
                    let #ident_field = if let Some(value) = #ident_field {
                        value
                    } else {
                        return Ok(None);
                    }
                }
            }
        })
        .next() {
            Some(token_stream) => token_stream,
            None => quote! ()
        }
    }
}

fn struct_field(
    field: &syn::Field,
    fields_without_subcommand: &[proc_macro2::TokenStream],
) -> proc_macro2::TokenStream {
    let ident_field = &field.clone().ident.expect("this field does not exist");
    if fields_without_subcommand
        .iter()
        .map(|token_stream| token_stream.to_string())
        .any(|x| *ident_field == x)
    {
        quote! {
            #ident_field: new_context_scope.#ident_field
        }
    } else {
        quote! {
            #ident_field
        }
    }
}
