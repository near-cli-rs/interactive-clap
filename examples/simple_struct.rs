// This example shows how to parse data from the command line to a structure using the "interactive-clap" macro.

// 1) build an example: cargo build --example simple_struct
// 2) go to the `examples` folder: cd target/debug/examples
// 3) run an example: ./simple_struct (without parameters) => entered interactive mode
//                    ./simple_struct 30 QWE QWERTY => args: Ok(Args { age: 30, first_name: "QWE", second_name: "QWERTY" })
// To learn more about the parameters, use "help" flag: ./simple_struct --help

use interactive_clap::{ResultFromCli, ToCliArgs};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
struct Args {
    age: u64,
    first_name: String,
    second_name: String,
}

fn main() -> color_eyre::Result<()> {
    let mut cli_args = Args::parse();
    let context = (); // default: input_context = ()
    loop {
        let args = <Args as interactive_clap::FromCli>::from_cli(Some(cli_args), context);
        match args {
            ResultFromCli::Ok(cli_args) | ResultFromCli::Cancel(Some(cli_args)) => {
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
