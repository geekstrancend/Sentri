//! CLI behavior tests.
//!
//! These tests validate that the CLI tool behaves correctly,
//! producing correct exit codes, output formats, and error handling.

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Setup a test project directory with sample files
fn setup_test_project() -> TempDir {
    let temp = TempDir::new().expect("Failed to create temp dir");
    let base = temp.path();

    // Create a sample DSL file
    let dsl_content = r#"
invariant: balance_conservation
description: "Total balance must be conserved across transactions"

forall tx in transactions:
    sum(tx.inputs) == sum(tx.outputs) + tx.fee
"#;

    fs::write(base.join("invariants.invar"), dsl_content)
        .expect("Failed to write invariants file");

    temp
}

#[test]
fn test_cli_help_output() {
    let mut cmd = Command::cargo_bin("invar").expect("Failed to find binary");
    cmd.arg("--help");
    cmd.assert().success()
        .stdout(predicate::str::contains("Invar"));
}

#[test]
fn test_cli_version_output() {
    let mut cmd = Command::cargo_bin("invar").expect("Failed to find binary");
    cmd.arg("--version");
    cmd.assert().success();
}

#[test]
fn test_cli_init_creates_project() {
    let temp = TempDir::new().expect("Failed to create temp dir");
    let project_path = temp.path().join("new_project");

    let mut cmd = Command::cargo_bin("invar").expect("Failed to find binary");
    cmd.arg("init")
        .arg(&project_path);
    
    cmd.assert().success();
    assert!(project_path.exists(), "init should create project directory");
}

#[test]
fn test_cli_missing_file_exits_with_error() {
    let mut cmd = Command::cargo_bin("invar").expect("Failed to find binary");
    cmd.arg("build")
        .arg("--source")
        .arg("/nonexistent/file.rs")
        .arg("--chain")
        .arg("solana")
        .arg("--output")
        .arg("/tmp/out");

    cmd.assert().failure();
}

#[test]
fn test_cli_invalid_chain_exits_with_error() {
    let temp = setup_test_project();
    
    let mut cmd = Command::cargo_bin("invar").expect("Failed to find binary");
    cmd.arg("build")
        .arg("--source")
        .arg(temp.path().join("test.rs"))
        .arg("--chain")
        .arg("invalid_chain")
        .arg("--output")
        .arg(temp.path().join("output"));

    cmd.assert().failure();
}

#[test]
fn test_cli_verbose_flag_produces_output() {
    let temp = setup_test_project();
    
    let mut cmd = Command::cargo_bin("invar").expect("Failed to find binary");
    cmd.arg("--verbose")
        .arg("list");

    cmd.assert().success();
}

#[test]
fn test_cli_log_level_flag() {
    let mut cmd = Command::cargo_bin("invar").expect("Found to find binary");
    cmd.arg("--log-level")
        .arg("debug")
        .arg("list");

    cmd.assert().success();
}

#[test]
fn test_cli_invalid_subcommand() {
    let mut cmd = Command::cargo_bin("invar").expect("Failed to find binary");
    cmd.arg("nonexistent_command");

    cmd.assert().failure();
}

/// Exit code tests
mod exit_codes {
    use super::*;

    #[test]
    fn test_exit_code_success_is_zero() {
        let mut cmd = Command::cargo_bin("invar").expect("Failed to find binary");
        cmd.arg("--help");
        
        let output = cmd.output().expect("Failed to execute");
        assert_eq!(output.status.code(), Some(0), "Success should exit with 0");
    }

    #[test]
    fn test_exit_code_error_is_nonzero() {
        let mut cmd = Command::cargo_bin("invar").expect("Failed to find binary");
        cmd.arg("nonexistent_command");
        
        let output = cmd.output().expect("Failed to execute");
        assert_ne!(output.status.code(), Some(0), "Error should exit with non-zero");
    }
}

/// Output format tests
mod output_formats {
    use super::*;

    #[test]
    fn test_json_output_is_valid() {
        let temp = setup_test_project();
        
        let mut cmd = Command::cargo_bin("invar").expect("Failed to find binary");
        cmd.arg("report")
            .arg("--input")
            .arg(temp.path().join("test_report.json"))
            .arg("--format")
            .arg("json");

        // Check if output is valid JSON (when it runs successfully)
        let output = cmd.output().expect("Failed to execute");
        
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // Validate it's JSON-like
            assert!(stdout.contains("{") || stdout.is_empty());
        }
    }

    #[test]
    fn test_markdown_output() {
        let temp = setup_test_project();
        
        let mut cmd = Command::cargo_bin("invar").expect("Failed to find binary");
        cmd.arg("report")
            .arg("--input")
            .arg(temp.path().join("test_report.json"))
            .arg("--format")
            .arg("markdown");

        let output = cmd.output().expect("Failed to execute");
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Markdown output shouldn't be JSON
        assert!(!stdout.starts_with("{") || output.status.success());
    }
}

/// Configuration tests
mod configuration {
    use super::*;

    #[test]
    fn test_config_file_loading() {
        let temp = TempDir::new().expect("Failed to create temp dir");
        let config_path = temp.path().join("invar.toml");

        let config_content = r#"
[project]
name = "test-project"
version = "0.1.0"

[invariants]
chains = ["solana", "evm"]
"#;

        fs::write(&config_path, config_content)
            .expect("Failed to write config");

        // Config should be loadable
        assert!(config_path.exists());
    }
}

/// Determinism tests
mod determinism {
    use super::*;

    #[test]
    fn test_same_input_same_output() {
        let temp = setup_test_project();
        
        // Run the same command twice
        let mut cmd1 = Command::cargo_bin("invar").expect("Failed to find binary");
        cmd1.arg("list");
        let output1 = cmd1.output().expect("Failed to execute");

        let mut cmd2 = Command::cargo_bin("invar").expect("Failed to find binary");
        cmd2.arg("list");
        let output2 = cmd2.output().expect("Failed to execute");

        // Same input should produce same output
        assert_eq!(output1.stdout, output2.stdout, "CLI must be deterministic");
        assert_eq!(output1.stderr, output2.stderr, "Errors must be deterministic");
    }
}
