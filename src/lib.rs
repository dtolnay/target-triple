include!(concat!(env!("OUT_DIR"), "/macros.rs"));

/// The target triple that is being compiled for.
pub const TARGET: &str = target!();

/// The host triple of the Rust compiler.
pub const HOST: &str = host!();
