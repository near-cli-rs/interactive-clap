// This example shows additional functionality of the "interactive-clap" macro for parsing command-line data into a structure using macro attributes.

// 1) build an example: cargo build --example advanced_struct
// 2) go to the `examples` folder: cd target/debug/examples
// 3) run an example: ./advanced_struct (without parameters) => entered interactive mode
//                    ./advanced_struct --age-full-years 30 --first-name QWE --second-name QWERTY =>
//                                    => args: Ok(Args { age: 30, first_name: "QWE", second_name: "QWERTY" })
// To learn more about the parameters, use "help" flag: ./advanced_struct --help

#[derive(Debug, interactive_clap_derive::InteractiveClap)]
#[interactive_clap(skip_default_from_cli)]
struct Args {
    #[interactive_clap(long = "age-full-years")] // hgfashdgfajdfsadajsdfh
    #[interactive_clap(skip_default_from_cli_arg)] // указать для чего этот атрибут нужен
    #[interactive_clap(skip_default_input_arg)]
    age: u64,
    #[interactive_clap(long)]
    ///What is your first name?
    first_name: String,
    #[interactive_clap(long)]
    #[interactive_clap(skip_default_input_arg)]
    second_name: String,
}

impl Args {
    pub fn from_cli(
        optional_clap_variant: Option<CliArgs>,
        context: (),
    ) -> color_eyre::eyre::Result<Option<Self>> {
        let age = Self::from_cli_age(
            optional_clap_variant
                .clone()
                .and_then(|clap_variant| clap_variant.age),
            &context,
        )?;
        let first_name = Self::from_cli_first_name(
            optional_clap_variant
                .clone()
                .and_then(|clap_variant| clap_variant.first_name),
            &context,
        )?;
        let second_name = Self::from_cli_second_name(
            optional_clap_variant
                .clone()
                .and_then(|clap_variant| clap_variant.second_name),
            &context,
        )?;
        let new_context_scope = InteractiveClapContextScopeForArgs {
            age,
            first_name,
            second_name,
        };
        Ok(Some(Self {
            age: new_context_scope.age,
            first_name: new_context_scope.first_name,
            second_name: new_context_scope.second_name,
        }))
    }

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
