// This example shows additional functionality of the "interactive-clap" macro for parsing command line data into a structure using a subcommand in the macro attribute.

// 1) build an example: cargo build --example struct_with_subcommand
// 2) go to the `examples` folder: cd target/debug/examples
// 3) run an example: ./struct_with_subcommand (without parameters) => entered interactive mode
//                    ./struct_with_subcommand network => operation_mode: Ok(OperationMode { mode: Network })
//                    ./struct_with_subcommand offline => operation_mode: Ok(OperationMode { mode: Offline })
// To learn more about the parameters, use "help" flag: ./struct_with_subcommand --help

use interactive_clap::{ResultFromCli, ToCliArgs};

mod simple_enum;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
// #[interactive_clap(skip_default_from_cli)]
pub struct OperationMode {
    #[interactive_clap(subcommand)]
    pub mode: simple_enum::Mode,
}

// impl interactive_clap::FromCli for OperationMode {
//     type FromCliContext = ();
//     type FromCliError = color_eyre::eyre::Error;
//     fn from_cli(
//         optional_clap_variant: Option<<Self as interactive_clap::ToCli>::CliVariant>,
//         context: Self::FromCliContext,
//     ) -> ResultFromCli<<Self as interactive_clap::ToCli>::CliVariant, Self::FromCliError>
//     where
//         Self: Sized + interactive_clap::ToCli,
//     {
//         let mut clap_variant = optional_clap_variant.unwrap_or_default();
//         let new_context_scope = InteractiveClapContextScopeForOperationMode {};
//         match <simple_enum::Mode as interactive_clap::FromCli>::from_cli(clap_variant.mode.take(), context.into()) {
//             interactive_clap::ResultFromCli::Ok(cli_mode) => {
//                 clap_variant.mode = Some(cli_mode);
//             }
//             interactive_clap::ResultFromCli::Cancel(optional_cli_mode) => {
//                 clap_variant.mode = optional_cli_mode;
//                 return interactive_clap::ResultFromCli::Cancel(Some(clap_variant))
//             }
//             interactive_clap::ResultFromCli::Cancel(None) => return interactive_clap::ResultFromCli::Cancel(Some(clap_variant)),
//             interactive_clap::ResultFromCli::Back => return interactive_clap::ResultFromCli::Back,
//             interactive_clap::ResultFromCli::Err(optional_cli_mode, err) => {
//                 clap_variant.mode = optional_cli_mode;
//                 return interactive_clap::ResultFromCli::Err(Some(clap_variant), err);
//             }
// };

//         ResultFromCli::Ok(clap_variant)
//     }
// }

fn main() -> color_eyre::Result<()> {
    let mut cli_operation_mode = OperationMode::parse();
    let context = (); // default: input_context = ()
    loop {
        let operation_mode =
            <OperationMode as interactive_clap::FromCli>::from_cli(Some(cli_operation_mode), context);
        match operation_mode {
            ResultFromCli::Ok(cli_operation_mode) | ResultFromCli::Cancel(Some(cli_operation_mode)) => {
                println!(
                    "Your console command:  {}",
                    shell_words::join(&cli_operation_mode.to_cli_args())
                );
                return Ok(());
            }
            ResultFromCli::Cancel(None)=> {
                println!("Goodbye!");
                return Ok(());
            }
            ResultFromCli::Back => {
                cli_operation_mode = Default::default();
            }
            ResultFromCli::Err(cli_operation_mode, err) => {
                if let Some(cli_operation_mode) = cli_operation_mode {
                    println!(
                        "Your console command:  {}",
                        shell_words::join(&cli_operation_mode.to_cli_args())
                    );
                }
                return Err(err);
            }
        }
    }
}
