// This example shows additional functionality of the "interactive-clap" macro for parsing command line data into a structure using the context attributes of the macro.

// 1) build an example: cargo build --example struct_with_context
// 2) go to the `examples` folder: cd target/debug/examples
// 3) run an example: ./struct_with_context (without parameters) => entered interactive mode
//                    ./struct_with_context account QWERTY => offline_args: Ok(OfflineArgs { account: Sender { sender_account_id: "QWERTY" } })
// To learn more about the parameters, use "help" flag: ./struct_with_context --help

use interactive_clap::{ResultFromCli, ToCliArgs};

mod common;
mod simple_enum;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = ())]
#[interactive_clap(output_context = NetworkContext)]
pub struct OfflineArgs {
    #[interactive_clap(named_arg)]
    ///Specify a sender
    sender: Sender,
}

#[derive(Debug)]
pub struct OfflineArgsContext {
    pub some_context_field: i64,
}

impl OfflineArgsContext {
    fn from_previous_context(
        _previous_context: (),
        scope: &<OfflineArgs as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        Ok(Self {
            some_context_field: 42,
        })
    }
}

impl From<OfflineArgsContext> for NetworkContext {
    fn from(_: OfflineArgsContext) -> Self {
        Self {
            connection_config: None,
        }
    }
}

impl From<()> for NetworkContext {
    fn from(_: ()) -> Self {
        Self {
            connection_config: None,
        }
    }
}

impl From<NetworkContext> for () {
    fn from(_: NetworkContext) -> Self {
        ()
    }
}

#[derive(Debug)]
pub struct NetworkContext {
    pub connection_config: Option<common::ConnectionConfig>,
}

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = NetworkContext)]
pub struct Sender {
    #[interactive_clap(skip_default_input_arg)]
    sender_account_id: String,
    #[interactive_clap(subcommand)]
    network: simple_enum::Mode,
}

impl Sender {
    fn input_sender_account_id(
        context: &NetworkContext,
    ) -> color_eyre::eyre::Result<Option<String>> {
        println!("Let's use context: {:?}", context);
        match inquire::CustomType::new("What is the account ID?").prompt() {
            Ok(value) => Ok(Some(value)),
            Err(
                inquire::error::InquireError::OperationCanceled
                | inquire::error::InquireError::OperationInterrupted,
            ) => Ok(None),
            Err(err) => Err(err.into()),
        }
    }
}

fn main() -> color_eyre::Result<()> {
    let mut cli_offline_args = OfflineArgs::parse();
    let context = (); // #[interactive_clap(input_context = ())]
    loop {
        let offline_args = <OfflineArgs as interactive_clap::FromCli>::from_cli(
            Some(cli_offline_args.clone()),
            context,
        );
        match offline_args {
            ResultFromCli::Ok(cli_offline_args) | ResultFromCli::Cancel(Some(cli_offline_args)) => {
                println!(
                    "Your console command:  {}",
                    shell_words::join(&cli_offline_args.to_cli_args())
                );
                return Ok(());
            }
            ResultFromCli::Cancel(None) => {
                println!("Goodbye!");
                return Ok(());
            }
            ResultFromCli::Back => {
                cli_offline_args = Default::default();
            }
            ResultFromCli::Err(cli_offline_args, err) => {
                if let Some(cli_offline_args) = cli_offline_args {
                    println!(
                        "Your console command:  {}",
                        shell_words::join(&cli_offline_args.to_cli_args())
                    );
                }
                return Err(err);
            }
        }
    }
}
