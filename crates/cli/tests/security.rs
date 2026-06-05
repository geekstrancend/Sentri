//! Security-focused tests for invariant validation.
//!
//! These tests ensure the analysis and validation pipeline correctly
//! identifies security issues and produces accurate security reports.

#[path = "security/mod.rs"]
mod security;

// Dummy test to make Cargo recognize this as a test binary
#[test]
fn _security_tests_available() {
    // This test always passes - it exists only to make Cargo discover this test binary
}
