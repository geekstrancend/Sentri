#![warn(missing_docs)]
#![deny(unsafe_code)]

//! Utilities for Invar: logging, path handling, and version management.

pub mod logging;
pub mod path_utils;
pub mod version;
pub mod release;

pub use logging::setup_tracing;
pub use version::{SemanticVersion, ReleaseArtifact, ReproducibleBuildConfig, Platform};
pub use release::ReleaseManager;
