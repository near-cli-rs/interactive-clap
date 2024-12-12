fn pretty_codegen(ts: &proc_macro2::TokenStream) -> String {
    let file = syn::parse_file(&ts.to_string()).unwrap();
    prettyplease::unparse(&file)
}

#[test]
fn test_simple_struct() {
    let input = syn::parse_quote! {
        struct Args {
            age: u64,
            first_name: String,
            second_name: String,
        }
    };

    let interactive_clap_codegen = crate::derives::interactive_clap::impl_interactive_clap(&input);
    insta::assert_snapshot!(pretty_codegen(&interactive_clap_codegen));

    let to_cli_args_codegen = crate::derives::to_cli_args::impl_to_cli_args(&input);
    insta::assert_snapshot!(pretty_codegen(&to_cli_args_codegen));
}

#[test]
fn test_flag() {
    let input = syn::parse_quote! {
        struct Args {
            /// Offline mode
            #[interactive_clap(long)]
            offline: bool
        }
    };

    let interactive_clap_codegen = crate::derives::interactive_clap::impl_interactive_clap(&input);
    insta::assert_snapshot!(pretty_codegen(&interactive_clap_codegen));

    let input = syn::parse_quote! {
        struct CliArgs {
            /// Offline mode
            #[clap(long)]
            offline: bool
        }
    };

    let to_cli_args_codegen = crate::derives::to_cli_args::impl_to_cli_args(&input);
    insta::assert_snapshot!(pretty_codegen(&to_cli_args_codegen));
}

#[test]
fn test_vec_multiple_opt() {
    let input = syn::parse_quote! {
        struct Args {
            #[interactive_clap(long_vec_multiple_opt)]
            pub env: Vec<String>,
        }
    };

    let interactive_clap_codegen = crate::derives::interactive_clap::impl_interactive_clap(&input);
    insta::assert_snapshot!(pretty_codegen(&interactive_clap_codegen));

}

#[test]
fn test_vec_multiple_opt_to_cli_args() {
    let input = syn::parse_quote! {
        pub struct CliArgs {
            #[clap(long)]
            pub env: Vec<String>,
        }
    };

    let to_cli_args_codegen = crate::derives::to_cli_args::impl_to_cli_args(&input);
    insta::assert_snapshot!(pretty_codegen(&to_cli_args_codegen));
}

#[test]
// testing correct panic msg isn't really very compatible with 
// `proc-macro-error` crate
#[should_panic]
fn test_vec_multiple_opt_err() {
    let input = syn::parse_quote! {
        struct Args {
            #[interactive_clap(long_vec_multiple_opt)]
            pub env: String,
        }
    };

    let interactive_clap_codegen = crate::derives::interactive_clap::impl_interactive_clap(&input);
    insta::assert_snapshot!(pretty_codegen(&interactive_clap_codegen));

}

/// this test checks if doc comments are propagated up to `CliArgs` struct,
/// which has `clap::Parser` derive on it
///
/// also it checks that `#[interactive_clap(verbatim_doc_comment)]` attribute 
/// gets transferred to `#[clap(verbatim_doc_comment)]` on `second_field` of 
/// the same `CliArgs` struct
#[test]
fn test_doc_comments_propagate() {
    let input = syn::parse_quote! {
        struct Args {
            /// short first field description
            ///
            /// a longer paragraph, describing the usage and stuff with first field's
            /// awarenes of its possible applications
            first_field: u64,
            /// short second field description
            ///
            /// a longer paragraph, describing the usage and stuff with second field's
            /// awareness of its possible applications
            #[interactive_clap(verbatim_doc_comment)]
            second_field: String,
        }
    };

    let interactive_clap_codegen = crate::derives::interactive_clap::impl_interactive_clap(&input);
    insta::assert_snapshot!(pretty_codegen(&interactive_clap_codegen));

}
