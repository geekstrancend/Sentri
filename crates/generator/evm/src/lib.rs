#![deny(unsafe_code)]
#![allow(missing_docs)]

//! EVM code generator.

pub mod generator;

pub use generator::EvmGenerator;
