//! The Interactive-clap library is an add-on for the Command Line Argument
//! Parser <https://crates.io/crates/clap>. Interactive-clap allows you to parse
//! command line options. The peculiarity of this macro is that in the absence
//! of command line parameters, the interactive mode of entering these data by
//! the user is activated.

pub use interactive_clap_derive::{InteractiveClap, ToCliArgs};

/// Associated type [`Self::CliVariant`] is defined during derive of
/// [`macro@crate::InteractiveClap`]
///
/// This type has derive of [`clap::Parser`](https://docs.rs/clap/4.5.24/clap/trait.Parser.html), which allows to parse
/// initial input on cli, which may be incomplete
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

impl ToCli for bool {
    type CliVariant = bool;
}

pub trait ToInteractiveClapContextScope {
    type InteractiveClapContextScope;
}

pub trait ToCliArgs {
    fn to_cli_args(&self) -> std::collections::VecDeque<String>;
}

pub enum ResultFromCli<T, E> {
    Ok(T),
    Cancel(Option<T>),
    Back,
    Err(Option<T>, E),
}

pub trait FromCli {
    type FromCliContext;
    type FromCliError;
    fn from_cli(
        optional_clap_variant: Option<<Self as ToCli>::CliVariant>,
        context: Self::FromCliContext,
    ) -> ResultFromCli<<Self as ToCli>::CliVariant, Self::FromCliError>
    where
        Self: Sized + ToCli;
}

pub enum SelectVariantOrBack<T: strum::EnumMessage> {
    Variant(T),
    Back,
}
impl<T: strum::EnumMessage> std::fmt::Display for SelectVariantOrBack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Self::Variant(variant) = self {
            f.write_str(variant.get_message().unwrap())
        } else {
            f.write_str("back")
        }
    }
}
