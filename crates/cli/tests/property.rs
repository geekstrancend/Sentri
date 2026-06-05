//! Property-based tests using proptest.
//!
//! These tests use generative techniques to validate invariants
//! across large input spaces, ensuring robustness and stability.

#[path = "property/mod.rs"]
mod property;

// Dummy test to make Cargo recognize this as a test binary
#[test]
fn _property_tests_available() {
    // This test always passes - it exists only to make Cargo discover this test binary
}
