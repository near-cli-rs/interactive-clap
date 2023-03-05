// This example shows additional functionality of the "interactive-clap" macro for parsing command-line data into a structure using the macro's named attributes.
// "named_arg" is a simplified version of the subcommand, consisting of a single enum element.

// 1) build an example: cargo build --example struct_with_named_arg
// 2) go to the `examples` folder: cd target/debug/examples
// 3) run an example: ./struct_with_named_arg (without parameters) => entered interactive mode
//                    ./struct_with_named_arg account QWERTY => account: Ok(Account { account: Sender { sender_account_id: "QWERTY" } })
// To learn more about the parameters, use "help" flag: ./struct_with_named_arg --help

use interactive_clap::{ResultFromCli, ToCliArgs};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
// #[interactive_clap(skip_default_from_cli)]
struct Account {
    #[interactive_clap(named_arg)]
    ///Specify a sender
    account: Sender,
}

// impl interactive_clap::FromCli for Account {
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
//         let new_context_scope = InteractiveClapContextScopeForAccount {};
//         let optional_account = match clap_variant.account.take() {
//             Some(ClapNamedArgSenderForAccount::Account(cli_arg)) => Some(cli_arg),
//             None => None,
//         };
//         match <Sender as interactive_clap::FromCli>::from_cli(optional_account, context.into()) {
//             ResultFromCli::Ok(cli_sender) => {
//                 clap_variant.account = Some(ClapNamedArgSenderForAccount::Account(cli_sender));
//             }
//             ResultFromCli::Cancel(optional_cli_sender) => {
//                 clap_variant.account =
//                     optional_cli_sender.map(ClapNamedArgSenderForAccount::Account);
//                 return ResultFromCli::Cancel(Some(clap_variant));
//             }
//             ResultFromCli::Back => {
//                 return ResultFromCli::Back;
//             }
//             ResultFromCli::Err(optional_cli_sender, err) => {
//                 clap_variant.account =
//                     optional_cli_sender.map(ClapNamedArgSenderForAccount::Account);
//                 return ResultFromCli::Err(Some(clap_variant), err);
//             }
//         };
//         ResultFromCli::Ok(clap_variant)
//     }
// }

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct Sender {
    ///What is the sender account ID?
    pub sender_account_id: String,
}

fn main() -> color_eyre::Result<()> {
    let mut cli_account = Account::parse();
    let context = (); // default: input_context = ()
    loop {
        let account = <Account as interactive_clap::FromCli>::from_cli(Some(cli_account), context);
        match account {
            ResultFromCli::Ok(cli_account) | ResultFromCli::Cancel(Some(cli_account)) => {
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
