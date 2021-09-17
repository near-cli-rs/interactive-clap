pub trait ToCli {
    type CliVariant;
}

impl ToCli for String {
    type CliVariant = String;
}

impl ToCli for u64 {
    type CliVariant = u64;
}
