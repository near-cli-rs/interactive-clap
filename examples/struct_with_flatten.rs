// This example shows additional functionality of the "interactive-clap" macro for parsing command-line data into a structure using the macro's flatten attributes.

// 1) build an example: cargo build --example struct_with_flatten
// 2) go to the `examples` folder: cd target/debug/examples
// 3) run an example: ./struct_with_flatten (without parameters) => entered interactive mode
//                    ./struct_with_flatten --no-docker --no-abi --out-dir /Users/Documents/Rust --color never test.testnet offline => contract: CliContract { build_command_args: Some(CliBuildCommand { no_docker: true, no_release: false, no_abi: true, no_embed_abi: false, no_doc: false, out_dir: Some("/Users/Documents/Rust"), manifest_path: None, color: Some(Never) }), contract_account_id: Some("test.testnet"), mode: Some(Offline) }
// To learn more about the parameters, use "help" flag: ./struct_with_flatten --help

// Note: currently there is no automatic generation of "interactive clap::From Cli"

use interactive_clap::{ResultFromCli, ToCliArgs};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = ())]
#[interactive_clap(output_context = ContractContext)]
#[interactive_clap(skip_default_from_cli)]
pub struct Contract {
    #[interactive_clap(flatten)]
    /// Specify a build command args:
    build_command_args: BuildCommand,
    /// What is the contract account ID?
    contract_account_id: String,
    #[interactive_clap(subcommand)]
    pub mode: Mode,
}

#[derive(Debug, Clone)]
pub struct ContractContext;

impl ContractContext {
    pub fn from_previous_context(
        previous_context: (),
        scope: &<Contract as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        // Your commands
        Ok(Self)
    }
}

impl interactive_clap::FromCli for Contract {
    type FromCliContext = ();
    type FromCliError = color_eyre::eyre::Error;
    fn from_cli(
        optional_clap_variant: Option<<Self as interactive_clap::ToCli>::CliVariant>,
        context: Self::FromCliContext,
    ) -> interactive_clap::ResultFromCli<
        <Self as interactive_clap::ToCli>::CliVariant,
        Self::FromCliError,
    >
    where
        Self: Sized + interactive_clap::ToCli,
    {
        let mut clap_variant = optional_clap_variant.unwrap_or_default();

        let build_command_args =
            if let Some(cli_build_command_args) = &clap_variant.build_command_args {
                BuildCommand {
                    no_docker: cli_build_command_args.no_docker,
                    no_release: cli_build_command_args.no_release,
                    no_abi: cli_build_command_args.no_abi,
                    no_embed_abi: cli_build_command_args.no_embed_abi,
                    no_doc: cli_build_command_args.no_doc,
                    out_dir: cli_build_command_args.out_dir.clone(),
                    manifest_path: cli_build_command_args.manifest_path.clone(),
                    color: cli_build_command_args.color.clone(),
                }
            } else {
                BuildCommand::default()
            };

        if clap_variant.contract_account_id.is_none() {
            clap_variant.contract_account_id = match Self::input_contract_account_id(&context) {
                Ok(Some(contract_account_id)) => Some(contract_account_id),
                Ok(None) => return interactive_clap::ResultFromCli::Cancel(Some(clap_variant)),
                Err(err) => return interactive_clap::ResultFromCli::Err(Some(clap_variant), err),
            };
        }
        let contract_account_id = clap_variant
            .contract_account_id
            .clone()
            .expect("Unexpected error");

        let new_context_scope = InteractiveClapContextScopeForContract {
            build_command_args,
            contract_account_id,
        };

        let output_context =
            match ContractContext::from_previous_context(context, &new_context_scope) {
                Ok(new_context) => new_context,
                Err(err) => return interactive_clap::ResultFromCli::Err(Some(clap_variant), err),
            };

        match <Mode as interactive_clap::FromCli>::from_cli(
            clap_variant.mode.take(),
            context.into(),
        ) {
            interactive_clap::ResultFromCli::Ok(cli_field) => {
                clap_variant.mode = Some(cli_field);
            }
            interactive_clap::ResultFromCli::Cancel(option_cli_field) => {
                clap_variant.mode = option_cli_field;
                return interactive_clap::ResultFromCli::Cancel(Some(clap_variant));
            }
            interactive_clap::ResultFromCli::Cancel(option_cli_field) => {
                clap_variant.mode = option_cli_field;
                return interactive_clap::ResultFromCli::Cancel(Some(clap_variant));
            }
            interactive_clap::ResultFromCli::Back => {
                return interactive_clap::ResultFromCli::Back;
            }
            interactive_clap::ResultFromCli::Err(option_cli_field, err) => {
                clap_variant.mode = option_cli_field;
                return interactive_clap::ResultFromCli::Err(Some(clap_variant), err);
            }
        };
        interactive_clap::ResultFromCli::Ok(clap_variant)
    }
}

#[derive(Debug, Default, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = ())]
#[interactive_clap(output_context = BuildCommandlContext)]
pub struct BuildCommand {
    /// Build contract without SourceScan verification
    #[interactive_clap(long)]
    pub no_docker: bool,
    /// Build contract in debug mode, without optimizations and bigger is size
    #[interactive_clap(long)]
    pub no_release: bool,
    /// Do not generate ABI for the contract
    #[interactive_clap(long)]
    pub no_abi: bool,
    /// Do not embed the ABI in the contract binary
    #[interactive_clap(long)]
    pub no_embed_abi: bool,
    /// Do not include rustdocs in the embedded ABI
    #[interactive_clap(long)]
    pub no_doc: bool,
    /// Copy final artifacts to this directory
    #[interactive_clap(long)]
    #[interactive_clap(skip_interactive_input)]
    pub out_dir: Option<String>,
    /// Path to the `Cargo.toml` of the contract to build
    #[interactive_clap(long)]
    #[interactive_clap(skip_interactive_input)]
    pub manifest_path: Option<String>,
    /// Coloring: auto, always, never
    #[interactive_clap(long)]
    #[interactive_clap(value_enum)]
    #[interactive_clap(skip_interactive_input)]
    pub color: Option<ColorPreference>,
}

#[derive(Debug, Clone)]
pub struct BuildCommandlContext {
    build_command_args: BuildCommand,
}

impl BuildCommandlContext {
    pub fn from_previous_context(
        _previous_context: (),
        scope: &<BuildCommand as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let build_command_args = BuildCommand {
            no_docker: scope.no_docker,
            no_release: scope.no_release,
            no_abi: scope.no_abi,
            no_embed_abi: scope.no_embed_abi,
            no_doc: scope.no_doc,
            out_dir: scope.out_dir.clone(),
            manifest_path: scope.manifest_path.clone(),
            color: scope.color.clone(),
        };
        Ok(Self { build_command_args })
    }
}

use strum::{EnumDiscriminants, EnumIter, EnumMessage};

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
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

use std::{env, str::FromStr};

#[derive(Debug, EnumDiscriminants, Clone, clap::ValueEnum)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
pub enum ColorPreference {
    Auto,
    Always,
    Never,
}

impl interactive_clap::ToCli for ColorPreference {
    type CliVariant = ColorPreference;
}

impl std::fmt::Display for ColorPreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Always => write!(f, "always"),
            Self::Never => write!(f, "never"),
        }
    }
}

impl FromStr for ColorPreference {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(default_mode()),
            "always" => Ok(ColorPreference::Always),
            "never" => Ok(ColorPreference::Never),
            _ => Err(format!("invalid color preference: {}", s)),
        }
    }
}

fn default_mode() -> ColorPreference {
    ColorPreference::Never
}

impl ColorPreference {
    pub fn as_str(&self) -> &str {
        match self {
            ColorPreference::Auto => "auto",
            ColorPreference::Always => "always",
            ColorPreference::Never => "never",
        }
    }
}

fn main() -> color_eyre::Result<()> {
    let mut cli_contract = Contract::parse();
    let context = (); // default: input_context = ()
    loop {
        let contract =
            <Contract as interactive_clap::FromCli>::from_cli(Some(cli_contract), context);
        match contract {
            ResultFromCli::Ok(cli_contract) | ResultFromCli::Cancel(Some(cli_contract)) => {
                println!("contract: {cli_contract:#?}");
                println!(
                    "Your console command:  {}",
                    shell_words::join(&cli_contract.to_cli_args())
                );
                return Ok(());
            }
            ResultFromCli::Cancel(None) => {
                println!("Goodbye!");
                return Ok(());
            }
            ResultFromCli::Back => {
                cli_contract = Default::default();
            }
            ResultFromCli::Err(cli_contract, err) => {
                if let Some(cli_contract) = cli_contract {
                    println!(
                        "Your console command:  {}",
                        shell_words::join(&cli_contract.to_cli_args())
                    );
                }
                return Err(err);
            }
        }
    }
}
