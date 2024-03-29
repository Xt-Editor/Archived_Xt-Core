//! Crate for the core functions and modules of Xt.
#![deny(
    warnings,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::all,
    clippy::cargo,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    variant_size_differences
)]
#![allow(dead_code)]

mod buffer;
pub mod logging;
mod modes;
mod workspace;

pub use buffer::Buffer;
pub use workspace::Workspace;
