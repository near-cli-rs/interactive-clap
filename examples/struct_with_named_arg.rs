// cargo run --example struct_with_named_arg account QWERTY => account: Ok(Account { account: Sender { sender_account_id: "QWERTY" } })
// cargo run --example struct_with_named_arg                => entered interactive mode

use clap::Clap;
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
    let context = ();  // default: input_context = ()
    let account = Account::from_cli(Some(cli_account), context);
    println!("account: {:?}", account)
}
