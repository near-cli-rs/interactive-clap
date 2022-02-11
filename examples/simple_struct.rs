use clap::Clap;

#[derive(Debug, interactive_clap_derive::InteractiveClap)]
#[interactive_clap(context = ())]
struct Args {
    #[interactive_clap(long = "prepaid-gas")]
    gas: u64,
    #[interactive_clap(long)]
    first_first: String,
}

impl Args {
    fn input_gas(_context: &()) -> color_eyre::eyre::Result<u64> {
        Ok(1_000_000_000)
    }

    fn input_first_first(_context: &()) -> color_eyre::eyre::Result<String> {
        Ok("First".to_string())
    }
}

fn main() {
    let cli_args = CliArgs::parse();
    println!("cli: {:?}", &cli_args);
    let args = Args::from_cli(Some(cli_args), ());
    println!("args: {:#?}", args)
}
