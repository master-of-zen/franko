//! CLI integration tests for Franko
//!
//! Tests basic CLI functionality without requiring actual book files

use assert_cmd::Command;
use predicates::prelude::*;

#[allow(deprecated)]
fn franko() -> Command {
    Command::cargo_bin("franko").unwrap()
}

#[test]
fn test_help_command() {
    franko()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("book reader"))
        .stdout(predicate::str::contains("read"))
        .stdout(predicate::str::contains("library"))
        .stdout(predicate::str::contains("serve"));
}

#[test]
fn test_version_command() {
    franko()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("franko"));
}

#[test]
fn test_library_help() {
    franko()
        .args(["library", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("add"))
        .stdout(predicate::str::contains("search"));
}

#[test]
fn test_config_help() {
    franko()
        .args(["config", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("show"))
        .stdout(predicate::str::contains("edit"));
}

#[test]
fn test_read_missing_file() {
    franko()
        .args(["read", "nonexistent.epub"])
        .assert()
        .failure();
}

#[test]
fn test_init_creates_config() {
    use tempfile::tempdir;

    let temp = tempdir().unwrap();

    // Set HOME to temp directory so config is created there
    franko()
        .arg("init")
        .env("HOME", temp.path())
        .env("XDG_CONFIG_HOME", temp.path().join(".config"))
        .assert()
        .success()
        .stdout(predicate::str::contains("Configuration initialized"));
}
