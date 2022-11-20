//! The Interactive-clap library is an add-on for the Command Line Argument
//! Parser (https://crates.io/crates/clap). Interactive-clap allows you to parse
//! command line options. The peculiarity of this macro is that in the absence
//! of command line parameters, the interactive mode of entering these data by
//! the user is activated.

pub use interactive_clap_derive::{InteractiveClap, ToCliArgs};

pub trait ToCli {
    type CliVariant;
}

impl ToCli for String {
    type CliVariant = String;
}

impl ToCli for u128 {
    type CliVariant = u128;
}

impl ToCli for u64 {
    type CliVariant = u64;
}

pub trait ToInteractiveClapContextScope {
    type InteractiveClapContextScope;
}

pub trait ToCliArgs {
    fn to_cli_args(&self) -> std::collections::VecDeque<String>;
}

pub trait FromCli {
    type FromCliContext;
    type FromCliError;
    fn from_cli(
        optional_clap_variant: Option<<Self as ToCli>::CliVariant>,
        context: Self::FromCliContext,
    ) -> Result<Option<Self>, Self::FromCliError> where Self: Sized + ToCli;
}
