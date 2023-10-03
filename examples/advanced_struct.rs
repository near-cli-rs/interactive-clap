// This example shows additional functionality of the "interactive-clap" macro for parsing command-line data into a structure using macro attributes.

// 1) build an example: cargo build --example advanced_struct
// 2) go to the `examples` folder: cd target/debug/examples
// 3) run an example: ./advanced_struct (without parameters) => entered interactive mode
//                    ./advanced_struct --age-full-years 30 --first-name QWE --second-name QWERTY --favorite-color red =>
//                                    => cli_args: CliArgs { age: Some(30), first_name: Some("QWE"), second_name: Some("QWERTY"), favorite_color: Some(Red) }
// To learn more about the parameters, use "help" flag: ./advanced_struct --help

use inquire::Select;
use interactive_clap::{ResultFromCli, ToCliArgs};
use strum::{EnumDiscriminants, EnumIter, EnumMessage, IntoEnumIterator};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(skip_default_from_cli)]
struct Args {
    #[interactive_clap(long = "age-full-years")]
    #[interactive_clap(skip_default_input_arg)]
    age: Option<u64>,
    #[interactive_clap(long)]
    ///What is your first name?
    first_name: String,
    #[interactive_clap(long)]
    #[interactive_clap(skip_default_input_arg)]
    second_name: String,
    #[interactive_clap(long)]
    #[interactive_clap(value_enum)]
    #[interactive_clap(skip_default_input_arg)]
    favorite_color: ColorPalette,
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
                Ok(optional_age) => optional_age,
                Err(err) => return ResultFromCli::Err(Some(clap_variant), err),
            };
        }
        let _age = clap_variant.age;
        if clap_variant.first_name.is_none() {
            clap_variant.first_name = match Self::input_first_name(&context) {
                Ok(Some(first_name)) => Some(first_name),
                Ok(None) => return ResultFromCli::Cancel(Some(clap_variant)),
                Err(err) => return ResultFromCli::Err(Some(clap_variant), err),
            };
        }
        let _first_name = clap_variant.first_name.clone().expect("Unexpected error");
        if clap_variant.second_name.is_none() {
            clap_variant.second_name = match Self::input_second_name(&context) {
                Ok(Some(second_name)) => Some(second_name),
                Ok(None) => return ResultFromCli::Cancel(Some(clap_variant)),
                Err(err) => return ResultFromCli::Err(Some(clap_variant), err),
            };
        }
        let _second_name = clap_variant.second_name.clone().expect("Unexpected error");
        if clap_variant.favorite_color.is_none() {
            clap_variant.favorite_color = match Self::input_favorite_color(&context) {
                Ok(Some(favorite_color)) => Some(favorite_color),
                Ok(None) => return ResultFromCli::Cancel(Some(clap_variant)),
                Err(err) => return ResultFromCli::Err(Some(clap_variant), err),
            };
        }
        let _favorite_color = clap_variant
            .favorite_color
            .clone()
            .expect("Unexpected error");
        ResultFromCli::Ok(clap_variant)
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

    fn input_favorite_color(_context: &()) -> color_eyre::eyre::Result<Option<ColorPalette>> {
        let variants = ColorPaletteDiscriminants::iter().collect::<Vec<_>>();
        let selected = Select::new("What color is your favorite?", variants).prompt()?;
        match selected {
            ColorPaletteDiscriminants::Red => Ok(Some(ColorPalette::Red)),
            ColorPaletteDiscriminants::Orange => Ok(Some(ColorPalette::Orange)),
            ColorPaletteDiscriminants::Yellow => Ok(Some(ColorPalette::Yellow)),
            ColorPaletteDiscriminants::Green => Ok(Some(ColorPalette::Green)),
            ColorPaletteDiscriminants::Blue => Ok(Some(ColorPalette::Blue)),
            ColorPaletteDiscriminants::Indigo => Ok(Some(ColorPalette::Indigo)),
            ColorPaletteDiscriminants::Violet => Ok(Some(ColorPalette::Violet)),
        }
    }
}

#[derive(Debug, EnumDiscriminants, Clone, clap::ValueEnum)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
pub enum ColorPalette {
    #[strum_discriminants(strum(message = "red"))]
    /// Red
    Red,
    #[strum_discriminants(strum(message = "orange"))]
    /// Orange
    Orange,
    #[strum_discriminants(strum(message = "yellow"))]
    /// Yellow
    Yellow,
    #[strum_discriminants(strum(message = "green"))]
    /// Green
    Green,
    #[strum_discriminants(strum(message = "blue"))]
    /// Blue
    Blue,
    #[strum_discriminants(strum(message = "indigo"))]
    /// Indigo
    Indigo,
    #[strum_discriminants(strum(message = "violet"))]
    /// Violet
    Violet,
}

impl interactive_clap::ToCli for ColorPalette {
    type CliVariant = ColorPalette;
}

impl std::str::FromStr for ColorPalette {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Self::Red),
            "orange" => Ok(Self::Orange),
            "yellow" => Ok(Self::Yellow),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            "indigo" => Ok(Self::Indigo),
            "violet" => Ok(Self::Violet),
            _ => Err("ColorPalette: incorrect value entered".to_string()),
        }
    }
}

impl std::fmt::Display for ColorPalette {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Red => write!(f, "red"),
            Self::Orange => write!(f, "orange"),
            Self::Yellow => write!(f, "yellow"),
            Self::Green => write!(f, "green"),
            Self::Blue => write!(f, "blue"),
            Self::Indigo => write!(f, "indigo"),
            Self::Violet => write!(f, "violet"),
        }
    }
}

impl std::fmt::Display for ColorPaletteDiscriminants {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Red => write!(f, "red"),
            Self::Orange => write!(f, "orange"),
            Self::Yellow => write!(f, "yellow"),
            Self::Green => write!(f, "green"),
            Self::Blue => write!(f, "blue"),
            Self::Indigo => write!(f, "indigo"),
            Self::Violet => write!(f, "violet"),
        }
    }
}

fn main() -> color_eyre::Result<()> {
    let mut cli_args = Args::parse();
    let context = (); // default: input_context = ()
    loop {
        let args = <Args as interactive_clap::FromCli>::from_cli(Some(cli_args), context);
        match args {
            ResultFromCli::Ok(cli_args) | ResultFromCli::Cancel(Some(cli_args)) => {
                println!("cli_args: {cli_args:?}");
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
