#![warn(missing_docs)]
#![deny(unsafe_code)]

//! Utilities for Invar: logging, path handling, and version management.

pub mod logging;
pub mod path_utils;
pub mod release;
pub mod version;

pub use logging::setup_tracing;
pub use release::ReleaseManager;
pub use version::{Platform, ReleaseArtifact, ReproducibleBuildConfig, SemanticVersion};
