//! Integration tests for complete workflows.
//!
//! These tests validate end-to-end workflows and cross-module interactions.

#[path = "integration/mod.rs"]
mod integration;

// Dummy test to make Cargo recognize this as a test binary
#[test]
fn _integration_tests_available() {
    // This test always passes - it exists only to make Cargo discover this test binary
}
