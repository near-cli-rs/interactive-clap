use super::pretty_codegen;

#[test]
#[ignore]
fn test_simple_enum() {
    let input = syn::parse_quote! {
        pub enum Mode {
            /// Prepare and, optionally, submit a new transaction with online mode
            Network,
            /// Prepare and, optionally, submit a new transaction with offline mode
            Offline,
        }
    };

    let interactive_clap_codegen = crate::derives::interactive_clap::impl_interactive_clap(&input);
    insta::assert_snapshot!(pretty_codegen(&interactive_clap_codegen));

    let to_cli_args_codegen = crate::derives::to_cli_args::impl_to_cli_args(&input);
    insta::assert_snapshot!(pretty_codegen(&to_cli_args_codegen));
}

#[test]
fn test_simple_enum_with_strum_discriminants() {
    let input = syn::parse_quote! {
        #[strum_discriminants(derive(EnumMessage, EnumIter))]
        /// A little beautiful comment about our choice
        pub enum Mode {
            /// Prepare and, optionally, submit a new transaction with online mode
            #[strum_discriminants(strum(message = "Yes, I keep it simple"))]
            Network,
            /// Prepare and, optionally, submit a new transaction with offline mode
            #[strum_discriminants(strum(
                message = "No, I want to work in no-network (air-gapped) environment"
            ))]
            Offline,
        }
    };

    let interactive_clap_codegen = crate::derives::interactive_clap::impl_interactive_clap(&input);
    insta::assert_snapshot!(pretty_codegen(&interactive_clap_codegen));

    let to_cli_args_codegen = crate::derives::to_cli_args::impl_to_cli_args(&input);
    insta::assert_snapshot!(pretty_codegen(&to_cli_args_codegen));
}
