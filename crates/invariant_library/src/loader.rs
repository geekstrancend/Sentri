//! Library loader for TOML-based invariants.

use invar_core::model::Invariant;
use invar_core::Result;
use std::path::Path;
use tracing::info;

/// Loads invariants from TOML files.
pub struct LibraryLoader;

impl LibraryLoader {
    /// Load invariants from a TOML file.
    pub fn load_from_toml(path: &Path) -> Result<Vec<Invariant>> {
        info!("Loading invariants from {:?}", path);

        let content = std::fs::read_to_string(path).map_err(invar_core::InvarError::IoError)?;

        // Parse TOML
        let _table: toml::Table = toml::from_str(&content)
            .map_err(|e| invar_core::InvarError::ConfigError(e.to_string()))?;

        let invariants = Vec::new();

        // TODO: Extract invariants from table
        // For now, just return empty vector
        info!("Loaded {} invariants", invariants.len());
        Ok(invariants)
    }

    /// Load all invariants from a directory.
    pub fn load_from_dir(dir: &Path) -> Result<Vec<Invariant>> {
        let mut all_invariants = Vec::new();

        // Read all .toml files in directory
        let entries = std::fs::read_dir(dir).map_err(invar_core::InvarError::IoError)?;

        for entry in entries {
            let entry = entry.map_err(invar_core::InvarError::IoError)?;
            let path = entry.path();

            if path.extension().is_some_and(|ext| ext == "toml") {
                let invariants = Self::load_from_toml(&path)?;
                all_invariants.extend(invariants);
            }
        }

        Ok(all_invariants)
    }
}
