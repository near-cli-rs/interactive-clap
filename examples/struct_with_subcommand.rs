// 1) build an example: cargo build --example struct_with_subcommand
// 2) go to the `examples` folder: cd target/debug/examples
// 3) run an example: ./struct_with_subcommand (without parameters) => entered interactive mode
//                    ./struct_with_subcommand network => operation_mode: Ok(OperationMode { mode: Network })
//                    ./struct_with_subcommand offline => operation_mode: Ok(OperationMode { mode: Offline })
// To learn more about the parameters, use "help" flag: ./struct_with_subcommand --help


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
