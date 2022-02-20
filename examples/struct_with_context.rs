// 1) build an example: cargo build --example struct_with_context
// 2) go to the `examples` folder: cd target/debug/examples
// 3) run an example: ./struct_with_context (without parameters) => entered interactive mode
//                    ./struct_with_context account QWERTY => offline_args: Ok(OfflineArgs { account: Sender { sender_account_id: "QWERTY" } })
// To learn more about the parameters, use "help" flag: ./struct_with_context --help


mod common;

#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
#[interactive_clap(input_context = ())]
#[interactive_clap(output_context = OfflineArgsContext)]
pub struct OfflineArgs {
    #[interactive_clap(named_arg)]
    ///Specify a sender
    account: Sender,
}

pub struct OfflineArgsContext {}

impl OfflineArgsContext {
    fn from_previous_context(
        _previous_context: (),
        _scope: &<OfflineArgs as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> Self {
        Self {}
    }
}

impl From<OfflineArgsContext> for NetworkContext {
    fn from(_: OfflineArgsContext) -> Self {
        Self {
            connection_config: None,
        }
    }
}

pub struct NetworkContext {
    pub connection_config: Option<common::ConnectionConfig>,
}

#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
#[interactive_clap(context = OfflineArgsContext)]
pub struct Sender {
    #[interactive_clap(skip_default_input_arg)]
    pub sender_account_id: String,
}

impl Sender {
    fn input_sender_account_id(context: &OfflineArgsContext) -> color_eyre::eyre::Result<String> {
        Ok(dialoguer::Input::new()
            .with_prompt("What is the account ID?")
            .interact_text()?)
    }
}

fn main() {
    let cli_offline_args = OfflineArgs::parse();
    let context = (); // #[interactive_clap(input_context = ())]
    let offline_args = OfflineArgs::from_cli(Some(cli_offline_args), context);
    println!("offline_args: {:?}", offline_args)
}
