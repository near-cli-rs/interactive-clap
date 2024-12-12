#[cfg(feature = "introspect")]
macro_rules! dbg_cond {
    ($($val:expr),* ) => {
        dbg!($($val),*)
    };
}

#[cfg(not(feature = "introspect"))]
macro_rules! dbg_cond {
    ($($val:expr),*) => {
        // no-op
    };
}
