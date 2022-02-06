use dialoguer::{theme::ColorfulTheme, Select};
use strum::{EnumMessage, IntoEnumIterator};
pub fn prompt_variant<T>(prompt: &str) -> T
where
    T: IntoEnumIterator + EnumMessage,
    T: Copy + Clone,
{
    let variants = T::iter().collect::<Vec<_>>();
    let actions = variants
        .iter()
        .map(|p| {
            p.get_message()
                .unwrap_or_else(|| "error[This entry does not have an option message!!]")
                .to_owned()
        })
        .collect::<Vec<_>>();

    let selected = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(&actions)
        .default(0)
        .interact()
        .unwrap();

    variants[selected]
}

#[derive(Debug, Clone)]
pub enum ConnectionConfig {
    Testnet,
    Mainnet,
    Betanet,
}
