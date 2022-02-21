// The "to_cli_args" method of the "interactive-clap" macro is designed to form and print the cli command using the interactive mode for entering parameters.

// 1) build an example: cargo build --example to_cli_args
// 2) go to the `examples` folder: cd target/debug/examples
// 3) run an example: ./to_cli_args (without parameters) => entered interactive mode
//                    ./to_cli_args send    => Your console command:  send
//                    ./to_cli_args display => Your console command:  display
// To learn more about the parameters, use "help" flag: ./to_cli_args --help

use dialoguer::{theme::ColorfulTheme, Select};
use strum::{EnumDiscriminants, EnumIter, EnumMessage, IntoEnumIterator};

mod common;

#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
#[interactive_clap(context = common::ConnectionConfig)]
struct OnlineArgs {
    #[interactive_clap(subcommand)]
    submit: Submit,
}

#[derive(Debug, EnumDiscriminants, Clone, clap::Clap, interactive_clap_derive::ToCliArgs)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
pub enum Submit {
    #[strum_discriminants(strum(message = "I want to send the transaction to the network"))]
    Send,
    #[strum_discriminants(strum(
        message = "I only want to print base64-encoded transaction for JSON RPC input and exit"
    ))]
    Display,
}

impl Submit {
    fn choose_variant(
        connection_config: common::ConnectionConfig,
    ) -> color_eyre::eyre::Result<Self> {
        let variants = SubmitDiscriminants::iter().collect::<Vec<_>>();
        let submits = variants
            .iter()
            .map(|p| p.get_message().unwrap().to_owned())
            .collect::<Vec<_>>();
        let select_submit = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("How would you like to proceed")
            .items(&submits)
            .default(0)
            .interact()
            .unwrap();
        match variants[select_submit] {
            SubmitDiscriminants::Send => Ok(Submit::Send),
            SubmitDiscriminants::Display => Ok(Submit::Display),
        }
    }

    fn from_cli(
        optional_clap_variant: Option<<Submit as interactive_clap::ToCli>::CliVariant>,
        context: common::ConnectionConfig,
    ) -> color_eyre::eyre::Result<Self> {
        let submit: Option<Submit> = optional_clap_variant.clone();
        match submit {
            Some(submit) => Ok(submit),
            None => Ok(Submit::Display),
        }
    }
}

impl interactive_clap::ToCli for Submit {
    type CliVariant = Submit;
}

fn main() {
    let mut cli_online_args = OnlineArgs::parse();
    let context = common::ConnectionConfig::Testnet; //#[interactive_clap(context = common::ConnectionConfig)]
    let online_args = OnlineArgs::from_cli(Some(cli_online_args), context).unwrap();
    cli_online_args = online_args.into();
    let completed_cli = cli_online_args.to_cli_args();
    println!(
        "Your console command:  {}",
        shell_words::join(&completed_cli)
    );
}
