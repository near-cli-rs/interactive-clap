// cargo run --example simple_struct -- --age-full-years 30 --first-name QWE --second-name QWERTY =>
//                                    => args: Ok(Args { age: 30, first_name: "QWE", second_name: "QWERTY" })

// cargo run --example simple_struct  => entered interactive mode

#[derive(Debug, interactive_clap_derive::InteractiveClap)]
#[interactive_clap(skip_default_from_cli)] 
struct Args {
    #[interactive_clap(long = "age-full-years")] // hgfashdgfajdfsadajsdfh
    #[interactive_clap(skip_default_from_cli)] // указать для чего этот атрибут нужен
    #[interactive_clap(skip_default_input_arg)]
    age: u64,
    #[interactive_clap(long)]
    ///What is your first name?
    first_name: String,
    #[interactive_clap(long)]
    #[interactive_clap(skip_default_input_arg)]
    second_name: String
}

impl Args {
    fn input_age(_context: &()) -> color_eyre::eyre::Result<u64> {
        Ok(dialoguer::Input::new()
            .with_prompt("How old are you?")
            .interact_text()?)
    }

    fn input_second_name(_context: &()) -> color_eyre::eyre::Result<String> {
        Ok(dialoguer::Input::new()
            .with_prompt("What is your last name?")
            .interact_text()?)
    }

    fn from_cli_age(
        optional_cli_age: Option<u64>,
        context: &(), // default: input_context = ()
    ) -> color_eyre::eyre::Result<u64> {
        match optional_cli_age {
            Some(age) => Ok(age),
            None => Self::input_age(&context),
        }
    }
}

fn main() {
    let cli_args = Args::parse();
    let args = Args::from_cli(Some(cli_args), ());
    println!("args: {:?}", args)
}
