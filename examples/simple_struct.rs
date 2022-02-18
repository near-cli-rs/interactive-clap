// cargo run --example simple_struct -- --age-full-years 30 --first-name QWE --second-name QWERTY =>
//                                    => args: Ok(Args { age: 30, first_name: "QWE", second_name: "QWERTY" })

// cargo run --example simple_struct  => entered interactive mode

#[derive(Debug, interactive_clap_derive::InteractiveClap)]
struct Args {
    age: u64,
    first_name: String,
    second_name: String
}

fn main() {
    let cli_args = Args::parse();
    let context = (); // default: input_context = () 
    let args = Args::from_cli(Some(cli_args), context);
    println!("args: {:?}", args)
}
