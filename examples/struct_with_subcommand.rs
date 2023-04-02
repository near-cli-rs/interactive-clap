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
pub struct OperationMode {
    #[interactive_clap(subcommand)]
    pub mode: simple_enum::Mode,
}

fn main() -> color_eyre::Result<()> {
    let mut cli_operation_mode = OperationMode::parse();
    let context = (); // default: input_context = ()
    loop {
        let operation_mode = <OperationMode as interactive_clap::FromCli>::from_cli(
            Some(cli_operation_mode),
            context,
        );
        match operation_mode {
            ResultFromCli::Ok(cli_operation_mode)
            | ResultFromCli::Cancel(Some(cli_operation_mode)) => {
                println!(
                    "Your console command:  {}",
                    shell_words::join(&cli_operation_mode.to_cli_args())
                );
                return Ok(());
            }
            ResultFromCli::Cancel(None) => {
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
