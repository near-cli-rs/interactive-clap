//! The Interactive-clap library is an add-on for the Command Line Argument
//! Parser (https://crates.io/crates/clap). Interactive-clap allows you to parse
//! command line options. The peculiarity of this macro is that in the absence
//! of command line parameters, the interactive mode of entering these data by 
//! the user is activated.


pub use interactive_clap_derive::{InteractiveClap, ToCliArgs};

// pub trait InteractiveClap {
//     fn interactive_clap();
// }

// pub trait ToCliArgs {
//     fn to_cli_args();
// }

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
