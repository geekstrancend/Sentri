#![deny(unsafe_code)]
#![allow(missing_docs)]

//! Invariant library: Load invariants from TOML files.

pub mod library;
pub mod loader;

pub use library::InvariantLibrary;
pub use loader::LibraryLoader;
