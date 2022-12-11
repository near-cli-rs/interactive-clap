// The "to_cli_args" method of the "interactive-clap" macro is designed to form and print the cli command using the interactive mode for entering parameters.

// 1) build an example: cargo build --example to_cli_args
// 2) go to the `examples` folder: cd target/debug/examples
// 3) run an example: ./to_cli_args (without parameters) => entered interactive mode
//                    ./to_cli_args send    => Your console command:  send
//                    ./to_cli_args display => Your console command:  display
// To learn more about the parameters, use "help" flag: ./to_cli_args --help

use inquire::Select;
use strum::{EnumDiscriminants, EnumIter, EnumMessage, IntoEnumIterator};

use interactive_clap::ToCliArgs;

mod common;

#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
#[interactive_clap(context = common::ConnectionConfig)]
struct OnlineArgs {
    #[interactive_clap(subcommand)]
    submit: Submit,
}

#[derive(Debug, EnumDiscriminants, Clone, clap::Parser, interactive_clap_derive::ToCliArgs)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
pub enum Submit {
    #[strum_discriminants(strum(message = "I want to send the transaction to the network"))]
    Send,
    #[strum_discriminants(strum(
        message = "I only want to print base64-encoded transaction for JSON RPC input and exit"
    ))]
    Display,
}

impl interactive_clap::FromCli for Submit {
    type FromCliContext = common::ConnectionConfig;
    type FromCliError = color_eyre::eyre::Error;

    fn from_cli(
        optional_clap_variant: Option<<Self as interactive_clap::ToCli>::CliVariant>,
        _context: Self::FromCliContext,
    ) -> Result<Option<Self>, Self::FromCliError>
    where
        Self: Sized + interactive_clap::ToCli,
    {
        let submit: Option<Submit> = optional_clap_variant.clone();
        match submit {
            Some(submit) => Ok(Some(submit)),
            None => Ok(Some(Submit::Display)),
        }
    }
}

impl Submit {
    fn choose_variant(
        _context: common::ConnectionConfig,
    ) -> color_eyre::eyre::Result<Option<Self>> {
        let variants = SubmitDiscriminants::iter().collect::<Vec<_>>();
        let mut submits = variants
            .iter()
            .map(|p| p.get_message().unwrap().to_owned())
            .collect::<Vec<_>>();
        submits.push("back".to_string());
        let select_submit = Select::new("How would you like to proceed", submits.clone())
            .prompt()
            .unwrap();
        let mut selected: usize = 0;
        for (i, item) in submits.iter().enumerate() {
            if item == &select_submit {
                selected = i;
                break;
            }
        }
        match variants.get(selected) {
            Some(SubmitDiscriminants::Send) => Ok(Some(Submit::Send)),
            Some(SubmitDiscriminants::Display) => Ok(Some(Submit::Display)),
            None => Ok(None),
        }
    }
}

impl interactive_clap::ToCli for Submit {
    type CliVariant = Submit;
}

impl std::fmt::Display for SubmitDiscriminants {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Send => write!(f, "send"),
            Self::Display => write!(f, "display"),
        }
    }
}

fn main() {
    let mut cli_online_args = OnlineArgs::parse();
    let context = common::ConnectionConfig::Testnet; //#[interactive_clap(context = common::ConnectionConfig)]
    let online_args = loop {
        if let Some(args) = <OnlineArgs as interactive_clap::FromCli>::from_cli(
            Some(cli_online_args.clone()),
            context.clone(),
        )
        .unwrap()
        {
            break args;
        }
    };
    cli_online_args = online_args.into();
    let completed_cli = cli_online_args.to_cli_args();
    println!(
        "Your console command:  {}",
        shell_words::join(&completed_cli)
    );
}
