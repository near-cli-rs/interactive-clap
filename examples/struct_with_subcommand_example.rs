mod enum_example;

#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
#[interactive_clap(context = ())]
pub struct OperationMode {
    #[interactive_clap(subcommand)]
    pub mode: enum_example::Mode,
}

fn main() {
    let cli_operation_mode = CliOperationMode::default();
    println!("cli_operation_mode: {:?}", cli_operation_mode)
}
