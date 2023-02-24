// This example shows additional functionality of the "interactive-clap" macro for parsing command line data into a structure using the context attributes of the macro.

// 1) build an example: cargo build --example struct_with_context
// 2) go to the `examples` folder: cd target/debug/examples
// 3) run an example: ./struct_with_context (without parameters) => entered interactive mode
//                    ./struct_with_context account QWERTY => offline_args: Ok(OfflineArgs { account: Sender { sender_account_id: "QWERTY" } })
// To learn more about the parameters, use "help" flag: ./struct_with_context --help

use inquire::Text;

mod common;

#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
#[interactive_clap(input_context = ())]
#[interactive_clap(output_context = NetworkContext)]
pub struct OfflineArgs {
    #[interactive_clap(named_arg)]
    ///Specify a sender
    account: Sender,
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

#[derive(Debug)]
pub struct NetworkContext {
    pub connection_config: Option<common::ConnectionConfig>,
}

#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
#[interactive_clap(context = NetworkContext)]
pub struct Sender {
    #[interactive_clap(skip_default_input_arg)]
    pub sender_account_id: String,
}

impl Sender {
    fn input_sender_account_id(context: &NetworkContext) -> color_eyre::eyre::Result<String> {
        println!("Let's use context: {:?}", context);
        Ok(Text::new("What is the account ID?").prompt()?)
    }
}

fn main() {
    let cli_offline_args = OfflineArgs::parse();
    let context = (); // #[interactive_clap(input_context = ())]
    let offline_args =
        <OfflineArgs as interactive_clap::FromCli>::from_cli(Some(cli_offline_args), context);
    println!("offline_args: {:?}", offline_args)
}
