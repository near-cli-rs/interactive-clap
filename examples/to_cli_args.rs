// The "to_cli_args" method of the "interactive-clap" macro is designed to form and print the cli command using the interactive mode for entering parameters.

// 1) build an example: cargo build --example to_cli_args
// 2) go to the `examples` folder: cd target/debug/examples
// 3) run an example: ./to_cli_args (without parameters) => entered interactive mode
//                    ./to_cli_args send    => Your console command:  send
//                    ./to_cli_args display => Your console command:  display
// To learn more about the parameters, use "help" flag: ./to_cli_args --help

use inquire::Select;
use interactive_clap::{ResultFromCli, SelectVariantOrBack, ToCliArgs};
use strum::{EnumDiscriminants, EnumIter, EnumMessage, IntoEnumIterator};

mod common;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = common::ConnectionConfig)]
struct OnlineArgs {
    /// What is the name of the network
    #[interactive_clap(skip_default_input_arg)]
    network_name: String,
    #[interactive_clap(subcommand)]
    submit: Submit,
}

impl OnlineArgs {
    fn input_network_name(
        _context: &common::ConnectionConfig,
    ) -> color_eyre::eyre::Result<Option<String>> {
        match inquire::Text::new("Input network name").prompt() {
            Ok(value) => Ok(Some(value)),
            Err(
                inquire::error::InquireError::OperationCanceled
                | inquire::error::InquireError::OperationInterrupted,
            ) => Ok(None),
            Err(err) => Err(err.into()),
        }
    }
}

#[derive(Debug, EnumDiscriminants, Clone, clap::Parser)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
pub enum Submit {
    #[strum_discriminants(strum(message = "I want to send the transaction to the network"))]
    Send(Args),
    #[strum_discriminants(strum(
        message = "I only want to print base64-encoded transaction for JSON RPC input and exit"
    ))]
    Display,
}

#[derive(Debug, EnumDiscriminants, Clone, clap::Parser)]
pub enum CliSubmit {
    Send(CliArgs),
    Display,
}

impl From<Submit> for CliSubmit {
    fn from(command: Submit) -> Self {
        match command {
            Submit::Send(args) => Self::Send(args.into()),
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
            None => Self::choose_variant(context),
        }
    }
}

impl interactive_clap::ToCliArgs for CliSubmit {
    fn to_cli_args(&self) -> std::collections::VecDeque<String> {
        match self {
            Self::Send(cli_args) => {
                let mut args = cli_args.to_cli_args();
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
        context: common::ConnectionConfig,
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
            Ok(SelectVariantOrBack::Variant(variant)) => ResultFromCli::Ok(match variant {
                SubmitDiscriminants::Send => {
                    let cli_args =
                        match <Args as interactive_clap::FromCli>::from_cli(None, context) {
                            ResultFromCli::Ok(cli_args) => cli_args,
                            ResultFromCli::Cancel(optional_cli_args) => {
                                return ResultFromCli::Cancel(Some(CliSubmit::Send(
                                    optional_cli_args.unwrap_or_default(),
                                )));
                            }
                            ResultFromCli::Back => return ResultFromCli::Back,
                            ResultFromCli::Err(optional_cli_args, err) => {
                                return ResultFromCli::Err(
                                    Some(CliSubmit::Send(optional_cli_args.unwrap_or_default())),
                                    err,
                                );
                            }
                        };
                    CliSubmit::Send(cli_args)
                }
                SubmitDiscriminants::Display => CliSubmit::Display,
            }),
            Ok(SelectVariantOrBack::Back) => ResultFromCli::Back,
            Err(
                inquire::error::InquireError::OperationCanceled
                | inquire::error::InquireError::OperationInterrupted,
            ) => ResultFromCli::Cancel(None),
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

#[derive(Debug, Clone, interactive_clap::InteractiveClap, clap::Args)]
#[interactive_clap(context = common::ConnectionConfig)]
pub struct Args {
    age: u64,
    first_name: String,
    second_name: String,
}

fn main() -> color_eyre::Result<()> {
    let mut cli_online_args = OnlineArgs::parse();
    let context = common::ConnectionConfig::Testnet; //#[interactive_clap(context = common::ConnectionConfig)]
    let cli_args = loop {
        match <OnlineArgs as interactive_clap::FromCli>::from_cli(
            Some(cli_online_args),
            context.clone(),
        ) {
            ResultFromCli::Ok(cli_args) => break cli_args,
            ResultFromCli::Cancel(Some(cli_args)) => {
                println!(
                    "Your console command:  {}",
                    shell_words::join(&cli_args.to_cli_args())
                );
                return Ok(());
            }
            ResultFromCli::Cancel(None) => {
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
    };
    println!("cli_args: {:?}", cli_args);
    println!(
        "Your console command:  {}",
        shell_words::join(&cli_args.to_cli_args())
    );
    Ok(())
}
