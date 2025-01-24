use proc_macro2::Span;
use quote::quote;

use super::common_field_methods as field_methods;

/// returns the whole result `TokenStream` of derive logic of containing module
pub fn token_stream(ast: &syn::DeriveInput, fields: &syn::Fields) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let context_scope_fields = fields
        .iter()
        .map(field_transform)
        .filter(|token_stream| !token_stream.is_empty())
        .collect::<Vec<_>>();
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

fn field_transform(field: &syn::Field) -> proc_macro2::TokenStream {
    let ident_field = &field.ident.clone().expect("this field does not exist");
    let ty = &field.ty;
    if !field_methods::with_subcommand::predicate(field)
        && !field_methods::with_subargs::predicate(field)
    {
        quote! {
            pub #ident_field: #ty
        }
    } else {
        quote!()
    }
}
