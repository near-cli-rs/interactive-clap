extern crate proc_macro;

use proc_macro2::Span;
use quote::quote;
use syn;

pub fn from_cli_arg(ast: &syn::DeriveInput, fields: &syn::Fields) -> Vec<proc_macro2::TokenStream> {
    let interactive_clap_attrs_context =
        super::interactive_clap_attrs_context::InteractiveClapAttrsContext::new(ast);

    let fields_without_subcommand = fields
        .iter()
        .filter(|field| super::fields_without_subcommand::is_field_without_subcommand(field))
        .map(|field| {
            let ident_field = &field.clone().ident.expect("this field does not exist");
            quote! {#ident_field}
        })
        .collect::<Vec<_>>();

    let fields_without_skip_default_from_cli_arg = fields
        .iter()
        .filter(|field| {
            super::fields_without_skip_default_from_cli_arg::is_field_without_skip_default_from_cli_arg(
                field,
            )
        })
        .map(|field| {
            let ident_field = &field.clone().ident.expect("this field does not exist");
            quote! {#ident_field}
        })
        .collect::<Vec<_>>();

    let get_arg_for_fields = fields
        .iter()
        .map(|field| {
            let ident_field = &field.clone().ident.expect("this field does not exist");
            let ty = &field.ty;
            let fields_without_subcommand_to_string = fields_without_subcommand
                .iter()
                .map(|token_stream| token_stream.to_string())
                .collect::<Vec<_>>();
            let fields_without_skip_default_from_cli_arg_to_string =
                fields_without_skip_default_from_cli_arg
                    .iter()
                    .map(|token_stream| token_stream.to_string())
                    .collect::<Vec<_>>();
            if fields_without_subcommand_to_string.contains(&ident_field.to_string())
                & fields_without_skip_default_from_cli_arg_to_string
                    .iter()
                    .map(|token_stream| token_stream.to_string())
                    .any(|x| *ident_field == x)
            {
                let fn_from_cli_arg =
                    syn::Ident::new(&format!("from_cli_{}", &ident_field), Span::call_site());
                let optional_cli_field_name =
                    syn::Ident::new(&format!("optional_cli_{}", ident_field), Span::call_site());
                let input_context_dir = interactive_clap_attrs_context
                    .clone()
                    .get_input_context_dir();
                let cli_field_type = super::cli_field_type::cli_field_type(ty);
                let fn_input_arg =
                    syn::Ident::new(&format!("input_{}", &ident_field), Span::call_site());

                let type_string = match &ty {
                    syn::Type::Path(type_path) => match type_path.path.segments.last() {
                        Some(path_segment) => path_segment.ident.to_string(),
                        _ => String::new(),
                    },
                    _ => String::new(),
                };
                if let "Option" = type_string.as_str() {
                    quote! {
                        fn #fn_from_cli_arg(
                            #optional_cli_field_name: #cli_field_type,
                            context: &#input_context_dir,
                        ) -> color_eyre::eyre::Result<#ty> {
                            match #optional_cli_field_name {
                                Some(#ident_field) => Ok(Some(#ident_field)),
                                None => Self::#fn_input_arg(&context),
                            }
                        }
                    }
                } else {
                    quote! {
                        fn #fn_from_cli_arg(
                            #optional_cli_field_name: #cli_field_type,
                            context: &#input_context_dir,
                        ) -> color_eyre::eyre::Result<#ty> {
                            match #optional_cli_field_name {
                                Some(#ident_field) => Ok(#ident_field),
                                None => Ok(Self::#fn_input_arg(&context)?.unwrap()), // XXX: I cannot remember where this function is used, but I had to use `.unwrap()` to make the example compilable. It must be implemented without `.unwrap()`
                            }
                        }
                    }
                }
            } else {
                quote!()
            }
        })
        .filter(|token_stream| !token_stream.is_empty())
        .collect::<Vec<proc_macro2::TokenStream>>();

    get_arg_for_fields
}
