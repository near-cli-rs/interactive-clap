extern crate proc_macro;

use quote::quote;
use syn;

#[derive(Debug, Clone)]
pub struct InteractiveClapAttrsContext {
    pub context_dir: Option<proc_macro2::TokenStream>,
    pub input_context_dir: Option<proc_macro2::TokenStream>,
    pub output_context_dir: Option<proc_macro2::TokenStream>,
    pub is_skip_default_from_cli: bool,
}

impl InteractiveClapAttrsContext {
    pub fn new(ast: &syn::DeriveInput) -> Self {
        let mut context_dir = quote!();
        let mut input_context_dir = quote!();
        let mut output_context_dir = quote!();
        let mut is_skip_default_from_cli = false;
        if !ast.attrs.is_empty() {
            for attr in ast.attrs.clone() {
                if attr.path.is_ident("interactive_clap") {
                    for attr_token in attr.tokens.clone() {
                        if let proc_macro2::TokenTree::Group(group) = attr_token {
                            if group.stream().to_string().contains("output_context") {
                                let group_stream =
                                    &group.stream().into_iter().collect::<Vec<_>>()[2..];
                                output_context_dir = quote! {#(#group_stream)*};
                            } else if group.stream().to_string().contains("input_context") {
                                let group_stream =
                                    &group.stream().into_iter().collect::<Vec<_>>()[2..];
                                input_context_dir = quote! {#(#group_stream)*};
                            } else if group.stream().to_string().contains("context") {
                                let group_stream =
                                    &group.stream().into_iter().collect::<Vec<_>>()[2..];
                                context_dir = quote! {#(#group_stream)*};
                            };
                            if group.stream().to_string().contains("skip_default_from_cli") {
                                is_skip_default_from_cli = true;
                            };
                        }
                    }
                };
            }
        };
        let context_dir: Option<proc_macro2::TokenStream> = if let true = !context_dir.is_empty() {
            Some(context_dir)
        } else {
            None
        };
        let input_context_dir: Option<proc_macro2::TokenStream> =
            if let true = !input_context_dir.is_empty() {
                Some(input_context_dir)
            } else {
                None
            };
        let output_context_dir: Option<proc_macro2::TokenStream> =
            if let true = !output_context_dir.is_empty() {
                Some(output_context_dir)
            } else {
                None
            };
        Self {
            context_dir,
            input_context_dir,
            output_context_dir,
            is_skip_default_from_cli,
        }
    }

    pub fn get_input_context_dir(self) -> proc_macro2::TokenStream {
        let context_dir = match self.context_dir {
            Some(context_dir) => context_dir,
            None => quote!(),
        };
        if !context_dir.is_empty() {
            return context_dir;
        };
        match self.input_context_dir {
            Some(input_context_dir) => input_context_dir,
            None => quote! {()},
        }
    }
}
