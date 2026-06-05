//! Common test utilities and fixtures.
//!
//! This module provides shared utilities for all test categories.

use std::path::PathBuf;
use std::fs;

/// Initialize test environment.
pub fn init_test_env() {
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .try_init();
}

/// Create a temporary test directory with a given structure.
pub fn create_test_project(name: &str, content: &[(String, String)]) -> tempfile::TempDir {
    let temp_dir = tempfile::TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path();

    for (file_path, file_content) in content {
        let full_path = base_path.join(file_path);
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create parent directory");
        }
        fs::write(&full_path, file_content).expect("Failed to write test file");
    }

    temp_dir
}

/// Assert that a value matches a pattern without panicking on mismatch.
#[macro_export]
macro_rules! assert_matches {
    ($value:expr, $pattern:pat) => {
        match &$value {
            $pattern => {}
            _ => panic!("Value did not match pattern: {:?}", $value),
        }
    };
}

/// Capture string output for validation.
pub struct OutputCapture {
    output: String,
}

impl OutputCapture {
    /// Create a new output capture.
    pub fn new() -> Self {
        Self {
            output: String::new(),
        }
    }

    /// Add a line to the capture.
    pub fn add_line(&mut self, line: &str) {
        self.output.push_str(line);
        self.output.push('\n');
    }

    /// Get the captured output.
    pub fn output(&self) -> &str {
        &self.output
    }

    /// Assert the output contains a substring.
    pub fn assert_contains(&self, substring: &str) {
        assert!(
            self.output.contains(substring),
            "Output does not contain '{}'. Output: {}",
            substring,
            self.output
        );
    }

    /// Assert the output matches a regex.
    pub fn assert_matches_regex(&self, pattern: &str) {
        let re = regex::Regex::new(pattern)
            .expect("Invalid regex pattern");
        assert!(
            re.is_match(&self.output),
            "Output does not match regex '{}'. Output: {}",
            pattern,
            self.output
        );
    }
}

impl Default for OutputCapture {
    fn default() -> Self {
        Self::new()
    }
}
