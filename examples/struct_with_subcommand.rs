// cargo run --example struct_with_subcommand offline => operation_mode: Ok(OperationMode { mode: Offline })
// cargo run --example struct_with_subcommand network => operation_mode: Ok(OperationMode { mode: Network })
// cargo run --example struct_with_subcommand         => entered interactive mode

mod simple_enum;

#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
pub struct OperationMode {
    #[interactive_clap(subcommand)]
    pub mode: simple_enum::Mode,
}

fn main() {
    let cli_operation_mode = OperationMode::parse();
    let context = (); // default: input_context = ()
    let operation_mode = OperationMode::from_cli(Some(cli_operation_mode), context);
    println!("operation_mode: {:?}", &operation_mode);
}
