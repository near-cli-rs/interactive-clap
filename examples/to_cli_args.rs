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
        let mut clap_variant = optional_clap_variant.unwrap_or_default();
        if clap_variant.network_name.is_none() {
            clap_variant.network_name = match Self::input_network_name(&context) {
                Ok(Some(network_name)) => Some(network_name),
                Ok(None) => return ResultFromCli::Ok(Some(clap_variant)),
                Err(err) => return ResultFromCli::Err(Some(clap_variant), err),
            };
        }

        let next_context = context.clone();

        match Submit::from_cli(clap_variant.submit, next_context) {
            ResultFromCli::Ok(submit) => {
                clap_variant.submit = submit;
            }
            ResultFromCli::Back => return ResultFromCli::Back,
            ResultFromCli::Err(submit, err) => {
                clap_variant.submit = submit;
                return ResultFromCli::Err(Some(clap_variant), err);
            }
        }
        ResultFromCli::Ok(Some(clap_variant))
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
            Some(submit) => ResultFromCli::Ok(Some(submit)),
            None => Self::choose_variant(context),
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
    ) -> ResultFromCli<
        <Self as interactive_clap::ToCli>::CliVariant,
        <Self as interactive_clap::FromCli>::FromCliError,
    > {
        match Select::new(
            "How would you like to proceed",
            SubmitDiscriminants::iter()
                .map(SelectVariantOrBack::Variant)
                .chain([SelectVariantOrBack::Back])
                .collect(),
        )
        .prompt()
        {
            Ok(SelectVariantOrBack::Variant(variant)) => ResultFromCli::Ok(Some(match variant {
                SubmitDiscriminants::Send => CliSubmit::Send,
                SubmitDiscriminants::Display => CliSubmit::Display,
            })),
            Ok(SelectVariantOrBack::Back) => ResultFromCli::Back,
            Err(
                inquire::error::InquireError::OperationCanceled
                | inquire::error::InquireError::OperationInterrupted,
            ) => ResultFromCli::Ok(None),
            Err(err) => ResultFromCli::Err(None, err.into()),
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

fn main() -> color_eyre::Result<()> {
    let mut cli_online_args = OnlineArgs::parse();
    let context = common::ConnectionConfig::Testnet; //#[interactive_clap(context = common::ConnectionConfig)]
    loop {
        match <OnlineArgs as interactive_clap::FromCli>::from_cli(
            Some(cli_online_args),
            context.clone(),
        ) {
            ResultFromCli::Ok(Some(cli_args)) => {
                println!(
                    "Your console command:  {}",
                    shell_words::join(&cli_args.to_cli_args())
                );
                return Ok(());
            }
            ResultFromCli::Ok(None) => {
                println!("Goodbye!");
                return Ok(());
            }
            ResultFromCli::Back => {
                cli_online_args = Default::default();
            }
            ResultFromCli::Err(cli_args, err) => {
                if let Some(cli_args) = cli_args {
                    println!(
                        "Your console command:  {}",
                        shell_words::join(&cli_args.to_cli_args())
                    );
                }
                return Err(err);
            }
        }
    }
}
