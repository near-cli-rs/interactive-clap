use strum::{EnumDiscriminants, EnumIter, EnumMessage};

#[derive(Debug, Clone, EnumDiscriminants, interactive_clap_derive::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(context = ())]
pub enum Mode {
    #[strum_discriminants(strum(message = "Yes, I keep it simple"))]
    Network,
    #[strum_discriminants(strum(
        message = "No, I want to work in no-network (air-gapped) environment"
    ))]
    Offline,
}

fn main() {
    let cli_mode = CliMode::Network;
    println!("cli_mode: {:?}", cli_mode);
    let variant = Mode::choose_variant(());
    println!("variant: {:?}", variant)
}
