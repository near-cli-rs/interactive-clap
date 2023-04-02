//This example shows how to parse data from the command line to an enum using the "interactive-clap" macro.

// 1) build an example: cargo build --example advanced_enum
// 2) go to the `examples` folder: cd target/debug/examples
// 3) run an example: ./advanced_enum (without parameters) => entered interactive mode
//                    ./advanced_enum network              => mode: Ok(Network)
//                    ./advanced_enum offline              => mode: Ok(Offline)
// To learn more about the parameters, use "help" flag: ./advanced_enum --help

use interactive_clap::{ResultFromCli, ToCliArgs};
use strum::{EnumDiscriminants, EnumIter, EnumMessage};

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
///To construct a transaction you will need to provide information about sender (signer) and receiver accounts, and actions that needs to be performed.
///Do you want to derive some information required for transaction construction automatically querying it online?
pub enum Mode {
    /// Prepare and, optionally, submit a new transaction with online mode
    #[strum_discriminants(strum(message = "Yes, I keep it simple"))]
    Network(Args),
    /// Prepare and, optionally, submit a new transaction with offline mode
    #[strum_discriminants(strum(
        message = "No, I want to work in no-network (air-gapped) environment"
    ))]
    Offline,
}

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct Args {
    age: u64,
    first_name: String,
    second_name: String,
}

fn main() -> color_eyre::Result<()> {
    let cli_mode = Mode::try_parse().ok();
    let context = (); // default: input_context = ()
    let mode = loop {
        let mode = <Mode as interactive_clap::FromCli>::from_cli(cli_mode.clone(), context);
        match mode {
            ResultFromCli::Ok(cli_mode) => break cli_mode,
            ResultFromCli::Cancel(Some(cli_mode)) => {
                println!(
                    "Your console command:  {}",
                    shell_words::join(&cli_mode.to_cli_args())
                );
                return Ok(());
            }
            ResultFromCli::Cancel(None) => {
                println!("Goodbye!");
                return Ok(());
            }
            ResultFromCli::Back => {}
            ResultFromCli::Err(optional_cli_mode, err) => {
                if let Some(cli_mode) = optional_cli_mode {
                    println!(
                        "Your console command:  {}",
                        shell_words::join(&cli_mode.to_cli_args())
                    );
                }
                return Err(err);
            }
        }
    };
    println!("cli_mode: {:?}", mode);
    println!(
        "Your console command:  {}",
        shell_words::join(&mode.to_cli_args())
    );
    Ok(())
}
