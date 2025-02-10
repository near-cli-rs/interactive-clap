#[cfg(feature = "introspect")]
macro_rules! dbg_cond {
    ($val:expr) => {
        dbg!($val)
    };
}

/// this macro under `introspect` feature can be used to debug how derive proc macros
/// ([`crate::InteractiveClap`], [`crate::ToCliArgs`]) work
///
/// ```bash
/// # interactive-clap-derive folder
/// cargo test test_doc_comments_propagate --features introspect -- --nocapture
/// # from repo root
/// cargo run --example struct_with_subargs --features interactive-clap-derive/introspect
/// ```
#[cfg(not(feature = "introspect"))]
macro_rules! dbg_cond {
    ($val:expr) => {
        #[allow(unused)]
        $val
    };
}
