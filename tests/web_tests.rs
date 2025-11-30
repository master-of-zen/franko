//! Web server tests for Franko
//!
//! Tests the web interface basic functionality

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

/// Test that the serve command accepts correct arguments
#[test]
fn test_serve_help() {
    #[allow(deprecated)]
    Command::cargo_bin("franko")
        .unwrap()
        .args(["serve", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("port"))
        .stdout(predicate::str::contains("bind"));
}

/// Test that serve validates port numbers
#[test]
fn test_serve_invalid_port() {
    let temp = tempdir().unwrap();

    // Port 0 might work or fail depending on OS
    // But we can test that the argument is accepted
    #[allow(deprecated)]
    let result = Command::cargo_bin("franko")
        .unwrap()
        .args(["serve", "--port", "abc"])
        .env("HOME", temp.path())
        .env("XDG_CONFIG_HOME", temp.path().join(".config"))
        .env("XDG_DATA_HOME", temp.path().join(".local/share"))
        .assert();

    // Should fail because "abc" is not a valid port number
    result.failure();
}
