use super::pretty_codegen;

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

    let step_one_output = syn::parse_quote! {
        pub struct CliArgs {
            pub age: Option<<u64 as interactive_clap::ToCli>::CliVariant>,
            pub first_name: Option<<String as interactive_clap::ToCli>::CliVariant>,
            pub second_name: Option<<String as interactive_clap::ToCli>::CliVariant>,
        }
    };

    let to_cli_args_codegen = crate::derives::to_cli_args::impl_to_cli_args(&step_one_output);
    insta::assert_snapshot!(pretty_codegen(&to_cli_args_codegen));
}

#[test]
fn test_simple_struct_with_named_arg() {
    let input = syn::parse_quote! {
        struct Account {
            #[interactive_clap(named_arg)]
            field_name: Sender,
        }
    };

    let interactive_clap_codegen = crate::derives::interactive_clap::impl_interactive_clap(&input);
    insta::assert_snapshot!(pretty_codegen(&interactive_clap_codegen));

    let step_one_output = syn::parse_quote! {
        pub struct CliAccount {
            #[clap(subcommand)]
            pub field_name: Option<ClapNamedArgSenderForAccount>,
        }
    };

    let to_cli_args_codegen = crate::derives::to_cli_args::impl_to_cli_args(&step_one_output);
    insta::assert_snapshot!(pretty_codegen(&to_cli_args_codegen));
}

/// this tested this problem https://github.com/near/near-cli-rs/pull/444#issuecomment-2631866217
#[test]
fn test_bug_fix_of_to_cli_args_derive() {
    let input = syn::parse_quote! {
        pub struct ViewAccountSummary {
            /// What Account ID do you need to view?
            account_id: crate::types::account_id::AccountId,
        }
    };

    let interactive_clap_codegen = crate::derives::interactive_clap::impl_interactive_clap(&input);
    insta::assert_snapshot!(pretty_codegen(&interactive_clap_codegen));

    let step_one_output = syn::parse_quote! {
        pub struct CliViewAccountSummary {
            /// What Account ID do you need to view?
            pub account_id: Option<
                <crate::types::account_id::AccountId as interactive_clap::ToCli>::CliVariant,
            >,
        }
    };

    let to_cli_args_codegen = crate::derives::to_cli_args::impl_to_cli_args(&step_one_output);
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

    let step_one_output = syn::parse_quote! {
        pub struct CliArgs {
            /// Offline mode
            #[clap(long)]
            pub offline: bool,
        }
    };

    let to_cli_args_codegen = crate::derives::to_cli_args::impl_to_cli_args(&step_one_output);
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
    let step_one_output = syn::parse_quote! {
        pub struct CliArgs {
            #[clap(long)]
            pub env: Vec<String>,
        }
    };

    let to_cli_args_codegen = crate::derives::to_cli_args::impl_to_cli_args(&step_one_output);
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
            #[interactive_clap(long)]
            #[interactive_clap(skip_interactive_input)]
            first_field: u64,
            /// short second field description
            ///
            /// a longer paragraph, describing the usage and stuff with second field's
            /// awareness of its possible applications
            #[interactive_clap(long)]
            #[interactive_clap(skip_interactive_input)]
            #[interactive_clap(verbatim_doc_comment)]
            second_field: String,
            /// short third field description
            ///
            /// a longer paragraph, describing the usage and stuff with third field's
            /// awareness of its possible applications
            #[interactive_clap(long)]
            #[interactive_clap(skip_interactive_input)]
            #[interactive_clap(verbatim_doc_comment)]
            third_field: bool,
        }
    };

    let interactive_clap_codegen = crate::derives::interactive_clap::impl_interactive_clap(&input);
    insta::assert_snapshot!(pretty_codegen(&interactive_clap_codegen));

    let step_one_output = syn::parse_quote! {
        pub struct CliArgs {
            /// short first field description
            ///
            /// a longer paragraph, describing the usage and stuff with first field's
            /// awarenes of its possible applications
            #[clap(long)]
            pub first_field: Option<<u64 as interactive_clap::ToCli>::CliVariant>,
            /// short second field description
            ///
            /// a longer paragraph, describing the usage and stuff with second field's
            /// awareness of its possible applications
            #[clap(long, verbatim_doc_comment)]
            pub second_field: Option<<String as interactive_clap::ToCli>::CliVariant>,
            /// short third field description
            ///
            /// a longer paragraph, describing the usage and stuff with third field's
            /// awareness of its possible applications
            #[clap(long, verbatim_doc_comment)]
            pub third_field: bool,
        }
    };

    let to_cli_args_codegen = crate::derives::to_cli_args::impl_to_cli_args(&step_one_output);
    insta::assert_snapshot!(pretty_codegen(&to_cli_args_codegen));
}
