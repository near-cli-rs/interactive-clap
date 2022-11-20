// This example shows how to parse data from the command line to a structure using the "interactive-clap" macro.

// 1) build an example: cargo build --example simple_struct
// 2) go to the `examples` folder: cd target/debug/examples
// 3) run an example: ./simple_struct (without parameters) => entered interactive mode
//                    ./simple_struct 30 QWE QWERTY => args: Ok(Args { age: 30, first_name: "QWE", second_name: "QWERTY" })
// To learn more about the parameters, use "help" flag: ./simple_struct --help

#[derive(Debug, interactive_clap::InteractiveClap)]
struct Args {
    age: u64,
    first_name: String,
    second_name: String,
}

fn main() {
    let cli_args = Args::parse();
    let context = (); // default: input_context = ()
    let args = <Args as interactive_clap::FromCli>::from_cli(Some(cli_args), context);
    println!("args: {:?}", args)
}
