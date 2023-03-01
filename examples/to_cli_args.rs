// The "to_cli_args" method of the "interactive-clap" macro is designed to form and print the cli command using the interactive mode for entering parameters.

// 1) build an example: cargo build --example to_cli_args
// 2) go to the `examples` folder: cd target/debug/examples
// 3) run an example: ./to_cli_args (without parameters) => entered interactive mode
//                    ./to_cli_args send    => Your console command:  send
//                    ./to_cli_args display => Your console command:  display
// To learn more about the parameters, use "help" flag: ./to_cli_args --help

use inquire::Select;
use strum::{EnumDiscriminants, EnumIter, EnumMessage, IntoEnumIterator};

use interactive_clap::{ResultFromCli, SelectVariantOrBack, ToCliArgs};

mod common;

#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
#[interactive_clap(context = common::ConnectionConfig)]
#[interactive_clap(skip_default_from_cli)]
struct OnlineArgs {
    /// What is the name of the network
    // #[interactive_clap(skip_default_input_arg)]
    network_name: String,
    #[interactive_clap(subcommand)]
    submit: Submit,
}

impl interactive_clap::FromCli for OnlineArgs {
    type FromCliContext = common::ConnectionConfig;
    type FromCliError = color_eyre::eyre::Error;

    fn from_cli(
        optional_clap_variant: Option<<Self as interactive_clap::ToCli>::CliVariant>,
        context: Self::FromCliContext,
    ) -> ResultFromCli<<Self as interactive_clap::ToCli>::CliVariant, Self::FromCliError>
    where
        Self: Sized + interactive_clap::ToCli,
    {
        let optional_network_name = match optional_clap_variant
            .clone()
            .and_then(|clap_variant| clap_variant.network_name)
        {
            Some(network_name) => Some(network_name),
            None => match Self::input_network_name(&context) {
                Ok(network_name) => Some(network_name),
                Err(_) => {
                    return ResultFromCli::Exit;
                }
            },
        };

        let optional_submit =
            match optional_clap_variant.and_then(|clap_variant| clap_variant.submit) {
                Some(submit) => Some(submit),
                None => match Submit::choose_variant(context) {
                    Ok(Some(submit)) => Some(submit.into()),
                    Ok(None) => {
                        return ResultFromCli::Back;
                    }
                    Err(_) => {
                        return ResultFromCli::Exit;
                    }
                },
            };
        ResultFromCli::Ok(CliOnlineArgs {
            network_name: optional_network_name,
            submit: optional_submit,
        })
    }
}

#[derive(Debug, EnumDiscriminants, Clone, clap::Parser)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
// #[interactive_clap(context = common::ConnectionConfig)]
// #[interactive_clap(skip_default_from_cli)]
pub enum Submit {
    #[strum_discriminants(strum(message = "I want to send the transaction to the network"))]
    Send,
    #[strum_discriminants(strum(
        message = "I only want to print base64-encoded transaction for JSON RPC input and exit"
    ))]
    Display,
}

#[derive(Debug, EnumDiscriminants, Clone, clap::Parser)]
pub enum CliSubmit {
    Send,
    Display,
}

impl From<Submit> for CliSubmit {
    fn from(command: Submit) -> Self {
        match command {
            Submit::Send => Self::Send,
            Submit::Display => Self::Display,
        }
    }
}

impl interactive_clap::FromCli for Submit {
    type FromCliContext = common::ConnectionConfig;
    type FromCliError = color_eyre::eyre::Error;

    fn from_cli(
        optional_clap_variant: Option<<Self as interactive_clap::ToCli>::CliVariant>,
        context: Self::FromCliContext,
    ) -> ResultFromCli<<Self as interactive_clap::ToCli>::CliVariant, Self::FromCliError>
    where
        Self: Sized + interactive_clap::ToCli,
    {
        match optional_clap_variant {
            Some(submit) => ResultFromCli::Ok(submit),
            None => match Self::choose_variant(context) {
                Ok(Some(submit)) => ResultFromCli::Ok(submit.into()),
                Ok(None) => ResultFromCli::Back,
                Err(_) => ResultFromCli::Exit,
            },
        }
    }
}

impl interactive_clap::ToCliArgs for CliSubmit {
    fn to_cli_args(&self) -> std::collections::VecDeque<String> {
        match self {
            Self::Send => {
                let mut args = std::collections::VecDeque::new();
                args.push_front("send".to_owned());
                args
            }
            Self::Display => {
                let mut args = std::collections::VecDeque::new();
                args.push_front("display".to_owned());
                args
            }
        }
    }
}

impl Submit {
    fn choose_variant(
        _context: common::ConnectionConfig,
    ) -> color_eyre::eyre::Result<Option<Self>> {
        let selected_variant = Select::new(
            "How would you like to proceed",
            SubmitDiscriminants::iter()
                .map(SelectVariantOrBack::Variant)
                .chain([SelectVariantOrBack::Back])
                .collect(),
        )
        .prompt()
        .unwrap();
        match selected_variant {
            SelectVariantOrBack::Variant(SubmitDiscriminants::Send) => Ok(Some(Submit::Send)),
            SelectVariantOrBack::Variant(SubmitDiscriminants::Display) => Ok(Some(Submit::Display)),
            SelectVariantOrBack::Back => Ok(None),
        }
    }
}
impl interactive_clap::ToCli for Submit {
    type CliVariant = CliSubmit;
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
        match <OnlineArgs as interactive_clap::FromCli>::from_cli(
            Some(cli_online_args.clone()),
            context.clone(),
        ) {
            ResultFromCli::Ok(args) => break args,
            ResultFromCli::Back => (),
            ResultFromCli::Exit => todo!(),
            ResultFromCli::Err(cli_args, err) => todo!(),
        }
    };
    cli_online_args = online_args.into();
    let completed_cli = cli_online_args.to_cli_args();
    println!(
        "Your console command:  {}",
        shell_words::join(&completed_cli)
    );
}
