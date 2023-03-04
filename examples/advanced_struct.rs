// This example shows additional functionality of the "interactive-clap" macro for parsing command-line data into a structure using macro attributes.

// 1) build an example: cargo build --example advanced_struct
// 2) go to the `examples` folder: cd target/debug/examples
// 3) run an example: ./advanced_struct (without parameters) => entered interactive mode
//                    ./advanced_struct --age-full-years 30 --first-name QWE --second-name QWERTY =>
//                                    => args: Ok(Args { age: 30, first_name: "QWE", second_name: "QWERTY" })
// To learn more about the parameters, use "help" flag: ./advanced_struct --help

use interactive_clap::{ResultFromCli, ToCliArgs};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(skip_default_from_cli)]
struct Args {
    #[interactive_clap(long = "age-full-years")] // hgfashdgfajdfsadajsdfh
    #[interactive_clap(skip_default_input_arg)]
    age: u64,
    #[interactive_clap(long)]
    ///What is your first name?
    first_name: String,
    #[interactive_clap(long)]
    #[interactive_clap(skip_default_input_arg)]
    second_name: String,
}

impl interactive_clap::FromCli for Args {
    type FromCliContext = ();
    type FromCliError = color_eyre::eyre::Error;
    fn from_cli(
        optional_clap_variant: Option<<Self as interactive_clap::ToCli>::CliVariant>,
        context: Self::FromCliContext,
    ) -> ResultFromCli<<Self as interactive_clap::ToCli>::CliVariant, Self::FromCliError>
    where
        Self: Sized + interactive_clap::ToCli,
    {
        let mut clap_variant = optional_clap_variant.unwrap_or_default();
        if clap_variant.age.is_none() {
            clap_variant.age = match Self::input_age(&context) {
                Ok(Some(age)) => Some(age),
                Ok(None) => return ResultFromCli::Ok(Some(clap_variant)),
                Err(err) => return ResultFromCli::Err(Some(clap_variant), err),
            };
        }
        let age = clap_variant.age.clone().expect("Unexpected error");
        if clap_variant.first_name.is_none() {
            clap_variant.first_name = match Self::input_first_name(&context) {
                Ok(Some(first_name)) => Some(first_name),
                Ok(None) => return ResultFromCli::Ok(Some(clap_variant)),
                Err(err) => return ResultFromCli::Err(Some(clap_variant), err),
            };
        }
        let first_name = clap_variant.first_name.clone().expect("Unexpected error");
        if clap_variant.second_name.is_none() {
            clap_variant.second_name = match Self::input_second_name(&context) {
                Ok(Some(second_name)) => Some(second_name),
                Ok(None) => return ResultFromCli::Ok(Some(clap_variant)),
                Err(err) => return ResultFromCli::Err(Some(clap_variant), err),
            };
        }
        let second_name = clap_variant.second_name.clone().expect("Unexpected error");
        ResultFromCli::Ok(Some(clap_variant))
    }
}

impl Args {
    fn input_age(_context: &()) -> color_eyre::eyre::Result<Option<u64>> {
        match inquire::CustomType::new("Input age full years".to_string().as_str()).prompt() {
            Ok(value) => Ok(Some(value)),
            Err(
                inquire::error::InquireError::OperationCanceled
                | inquire::error::InquireError::OperationInterrupted,
            ) => Ok(None),
            Err(err) => Err(err.into()),
        }
    }

    fn input_second_name(_context: &()) -> color_eyre::eyre::Result<Option<String>> {
        match inquire::Text::new("Input second name".to_string().as_str()).prompt() {
            Ok(value) => Ok(Some(value)),
            Err(
                inquire::error::InquireError::OperationCanceled
                | inquire::error::InquireError::OperationInterrupted,
            ) => Ok(None),
            Err(err) => Err(err.into()),
        }
    }
}

fn main() -> color_eyre::Result<()> {
    let mut cli_args = Args::parse();
    let context = (); // default: input_context = ()
    loop {
        let args = <Args as interactive_clap::FromCli>::from_cli(Some(cli_args), context);
        match args {
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
                cli_args = Default::default();
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
