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
