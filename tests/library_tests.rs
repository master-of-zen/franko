//! Library integration tests for Franko
//!
//! Tests library database operations

use tempfile::tempdir;

#[test]
fn test_library_list_empty() {
    use assert_cmd::Command;
    use predicates::prelude::*;

    let temp = tempdir().unwrap();

    #[allow(deprecated)]
    Command::cargo_bin("franko")
        .unwrap()
        .args(["library", "list"])
        .env("HOME", temp.path())
        .env("XDG_CONFIG_HOME", temp.path().join(".config"))
        .env("XDG_DATA_HOME", temp.path().join(".local/share"))
        .assert()
        .success()
        .stdout(
            predicate::str::contains("No books found").or(predicate::str::contains("Total: 0")),
        );
}

#[test]
fn test_library_search_empty() {
    use assert_cmd::Command;
    use predicates::prelude::*;

    let temp = tempdir().unwrap();

    #[allow(deprecated)]
    Command::cargo_bin("franko")
        .unwrap()
        .args(["library", "search", "nonexistent"])
        .env("HOME", temp.path())
        .env("XDG_CONFIG_HOME", temp.path().join(".config"))
        .env("XDG_DATA_HOME", temp.path().join(".local/share"))
        .assert()
        .success()
        .stdout(
            predicate::str::contains("No results found")
                .or(predicate::str::contains("No books found")),
        );
}

#[test]
fn test_library_info_nonexistent() {
    use assert_cmd::Command;
    use predicates::prelude::*;

    let temp = tempdir().unwrap();

    #[allow(deprecated)]
    Command::cargo_bin("franko")
        .unwrap()
        .args(["library", "info", "nonexistent-id"])
        .env("HOME", temp.path())
        .env("XDG_CONFIG_HOME", temp.path().join(".config"))
        .env("XDG_DATA_HOME", temp.path().join(".local/share"))
        .assert()
        .success()
        .stdout(predicate::str::contains("not found"));
}
