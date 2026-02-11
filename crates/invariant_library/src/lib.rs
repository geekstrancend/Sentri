#![warn(missing_docs)]
#![deny(unsafe_code)]

//! Invariant library: Load invariants from TOML files.

pub mod library;
pub mod loader;

pub use library::InvariantLibrary;
pub use loader::LibraryLoader;
