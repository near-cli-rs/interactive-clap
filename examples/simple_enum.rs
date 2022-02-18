// 1) собрать пример: cargo build --example simple_enum
// 2) cd target/debug/examples
// 3) запустить пример: ./simple_enum (без параметров) => entered interactive mode
//                      ./simple_enum network          => mode: Ok(Network)
//                      ./simple_enum offline          => mode: Ok(Offline)


use dialoguer::{theme::ColorfulTheme, Select};
use strum::{EnumDiscriminants, EnumIter, EnumMessage, IntoEnumIterator};

#[derive(Debug, Clone, EnumDiscriminants, interactive_clap_derive::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
///To construct a transaction you will need to provide information about sender (signer) and receiver accounts, and actions that needs to be performed.
///Do you want to derive some information required for transaction construction automatically querying it online?
pub enum Mode {
    /// Prepare and, optionally, submit a new transaction with online mode
    #[strum_discriminants(strum(message = "Yes, I keep it simple"))]
    Network,
    /// Prepare and, optionally, submit a new transaction with offline mode
    #[strum_discriminants(strum(
        message = "No, I want to work in no-network (air-gapped) environment"
    ))]
    Offline,
}

fn main() {
    let cli_mode = Mode::try_parse().ok();
    let context = (); // default: input_context = ()
    let mode = Mode::from_cli(cli_mode, context);
    println!("mode: {:?}", mode)
}
