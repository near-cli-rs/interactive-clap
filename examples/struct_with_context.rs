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
    pub connection_config: Option<crate::common::ConnectionConfig>,
}

#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
#[interactive_clap(context = OfflineArgsContext)]
pub struct Sender {
    pub sender_account_id: String,
}

impl Sender {
    fn input_sender_account_id(context: &OfflineArgsContext) -> color_eyre::eyre::Result<String> {
        Ok("Volodymyr".to_string())
    }
}

fn main() {
    let cli_offline_args = CliOfflineArgs::default();
    println!("cli_offline_args: {:?}", cli_offline_args);
    let offline_args = OfflineArgs::from_cli(Some(cli_offline_args), ());
    println!("offline_args: {:?}", offline_args)
}
