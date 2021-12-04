pub trait ToCli {
    type CliVariant;
}

impl ToCli for String {
    type CliVariant = String;
}

impl ToCli for u128 {
    type CliVariant = u128;
}

impl ToCli for u64 {
    type CliVariant = u64;
}

pub trait ToInteractiveClapContextScope {
    type InteractiveClapContextScope;
}
