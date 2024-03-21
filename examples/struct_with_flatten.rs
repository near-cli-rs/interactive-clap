// This example shows additional functionality of the "interactive-clap" macro for parsing command-line data into a structure using the macro's flatten attributes.

// 1) build an example: cargo build --example struct_with_flatten
// 2) go to the `examples` folder: cd target/debug/examples
// 3) run an example: ./struct_with_flatten (without parameters) => entered interactive mode
//                    ./struct_with_flatten QWERTY 18 => account: CliAccount { social_db_folder: None, account: Some(CliSender { sender_account_id: Some("QWERTY"), age: Some(18) }) }
// To learn more about the parameters, use "help" flag: ./struct_with_flatten --help

use interactive_clap::{ResultFromCli, ToCliArgs};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(skip_default_from_cli)]
struct Account {
    /// Change SocialDb prefix
    #[interactive_clap(long)]
    #[interactive_clap(skip_interactive_input)]
    social_db_folder: Option<String>,
    #[interactive_clap(skip_default_input_arg)]
    #[interactive_clap(flatten)]
    account: Sender,
}

impl interactive_clap::FromCli for Account {
    type FromCliContext = ();
    type FromCliError = color_eyre::eyre::Error;
    fn from_cli(
        optional_clap_variant: Option<<Self as interactive_clap::ToCli>::CliVariant>,
        _context: Self::FromCliContext,
    ) -> interactive_clap::ResultFromCli<
        <Self as interactive_clap::ToCli>::CliVariant,
        Self::FromCliError,
    >
    where
        Self: Sized + interactive_clap::ToCli,
    {
        let mut clap_variant = optional_clap_variant.clone().unwrap_or_default();

        let social_db_folder = clap_variant.clone().social_db_folder;

        let output_context = ();

        let account = match Sender::from_cli(
            optional_clap_variant.unwrap_or_default().account,
            output_context,
        ) {
            interactive_clap::ResultFromCli::Ok(cli_sender) => cli_sender,
            interactive_clap::ResultFromCli::Cancel(optional_cli_sender) => {
                clap_variant.account = optional_cli_sender;
                return interactive_clap::ResultFromCli::Cancel(Some(clap_variant));
            }
            interactive_clap::ResultFromCli::Back => return interactive_clap::ResultFromCli::Back,
            interactive_clap::ResultFromCli::Err(optional_cli_sender, err) => {
                clap_variant.account = optional_cli_sender;
                return interactive_clap::ResultFromCli::Err(Some(clap_variant), err);
            }
        };
        interactive_clap::ResultFromCli::Ok(CliAccount {
            social_db_folder,
            account: Some(account),
        })
    }
}

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct Sender {
    /// What is the sender account ID?
    pub sender_account_id: String,
    /// How old is the sender?
    pub age: u64,
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
