//! Path utilities for cross-platform compatibility.

use std::path::{Path, PathBuf};

/// Normalize a path for the current platform.
pub fn normalize_path(path: &Path) -> PathBuf {
    path.to_path_buf()
}

/// Check if a path exists and is a file.
pub fn is_file(path: &Path) -> bool {
    path.is_file()
}

/// Check if a path exists and is a directory.
pub fn is_dir(path: &Path) -> bool {
    path.is_dir()
}

/// Ensure a directory exists, creating it if necessary.
pub fn ensure_dir(path: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(path)
}

/// Read a file to string safely.
pub fn read_file(path: &Path) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}

/// Write a string to a file safely.
pub fn write_file(path: &Path, content: &str) -> std::io::Result<()> {
    std::fs::write(path, content)
}
