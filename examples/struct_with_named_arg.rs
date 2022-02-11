use clap::Clap;
#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
#[interactive_clap(context = ())]
struct OfflineArgs {
    #[interactive_clap(named_arg)]
    ///Specify a sender
    account: Sender,
}

#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
#[interactive_clap(context = ())]
pub struct Sender {
    pub sender_account_id: String,
}

impl Sender {
    fn input_sender_account_id(context: &()) -> color_eyre::eyre::Result<String> {
        Ok("Volodymyr".to_string())
    }
}

fn main() {
    let cli_offline_args = CliOfflineArgs::parse();
    println!("cli_offline_args: {:?}", cli_offline_args);
    let offline_args = OfflineArgs::from_cli(Some(cli_offline_args), ());
    println!("offline_args: {:?}", offline_args)
}
