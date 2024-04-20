// This example shows additional functionality of the "interactive-clap" macro for parsing command-line data into a structure using the macro's subargs attributes.

// 1) build an example: cargo build --example struct_with_subargs
// 2) go to the `examples` folder: cd target/debug/examples
// 3) run an example: ./struct_with_subargs (without parameters) => entered interactive mode
//                    ./struct_with_subargs QWERTY 18 => account: CliAccount { social_db_folder: None, account: Some(CliSender { sender_account_id: Some("QWERTY"), age: Some(18) }) }
// To learn more about the parameters, use "help" flag: ./struct_with_subargs --help

use interactive_clap::{ResultFromCli, ToCliArgs};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
struct Account {
    /// Change SocialDb prefix
    #[interactive_clap(long)]
    #[interactive_clap(skip_interactive_input)]
    social_db_folder: Option<String>,
    #[interactive_clap(subargs)]
    account: Sender,
}

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct Sender {
    /// What is the sender account ID?
    sender_account_id: String,
    /// How old is the sender?
    age: u64,
}

fn main() -> color_eyre::Result<()> {
    let mut cli_account = Account::parse();
    let context = (); // default: input_context = ()
    loop {
        let account = <Account as interactive_clap::FromCli>::from_cli(Some(cli_account), context);
        match account {
            ResultFromCli::Ok(cli_account) | ResultFromCli::Cancel(Some(cli_account)) => {
                println!("account: {cli_account:?}");
                println!(
                    "Your console command:  {}",
                    shell_words::join(&cli_account.to_cli_args())
                );
                return Ok(());
            }
            ResultFromCli::Cancel(None) => {
                println!("Goodbye!");
                return Ok(());
            }
            ResultFromCli::Back => {
                cli_account = Default::default();
            }
            ResultFromCli::Err(cli_account, err) => {
                if let Some(cli_account) = cli_account {
                    println!(
                        "Your console command:  {}",
                        shell_words::join(&cli_account.to_cli_args())
                    );
                }
                return Err(err);
            }
        }
    }
}
