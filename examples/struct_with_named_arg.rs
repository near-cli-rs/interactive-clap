// This example shows additional functionality of the "interactive-clap" macro for parsing command-line data into a structure using the macro's named attributes.
// "named_arg" is a simplified version of the subcommand, consisting of a single enum element.

// 1) build an example: cargo build --example struct_with_named_arg
// 2) go to the `examples` folder: cd target/debug/examples
// 3) run an example: ./struct_with_named_arg (without parameters) => entered interactive mode
//                    ./struct_with_named_arg account QWERTY => account: Ok(Account { account: Sender { sender_account_id: "QWERTY" } })
// To learn more about the parameters, use "help" flag: ./struct_with_named_arg --help

use clap::Parser;
#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
struct Account {
    #[interactive_clap(named_arg)]
    ///Specify a sender
    account: Sender,
}

#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
pub struct Sender {
    ///What is the account ID?
    pub sender_account_id: String,
}

fn main() {
    let cli_account = Account::parse();
    let context = (); // default: input_context = ()
    let account = Account::from_cli(Some(cli_account), context);
    println!("account: {:?}", account)
}
