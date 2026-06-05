//! CLI integration tests.
//!
//! These tests validate the command-line interface functionality.

#[path = "cli/mod.rs"]
mod cli;

// Dummy test to make Cargo recognize this as a test binary
#[test]
fn _cli_tests_available() {
    // This test always passes - it exists only to make Cargo discover this test binary
}
