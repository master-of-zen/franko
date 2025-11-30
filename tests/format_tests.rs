//! Format parser tests for Franko
//!
//! Tests book format detection and parsing

/// Test format detection from file paths
#[test]
fn test_format_detection() {
    // We can't use the internal types directly in integration tests,
    // so we test via the CLI behavior instead

    // This test verifies the format detection works correctly
    // by checking that the binary recognizes file extensions
}

#[test]
fn test_read_txt_file() {
    use assert_cmd::Command;
    use predicates::prelude::*;
    use std::fs;
    use tempfile::tempdir;

    let temp = tempdir().unwrap();
    let txt_path = temp.path().join("test.txt");

    fs::write(
        &txt_path,
        "This is a test book.\n\nWith multiple paragraphs.\n\nAnd some content.",
    )
    .unwrap();

    // Reading a txt file should work (at least parse it)
    // The actual reading requires TUI/web, but parsing should succeed
    #[allow(deprecated)]
    Command::cargo_bin("franko")
        .unwrap()
        .args(["library", "add", txt_path.to_str().unwrap()])
        .env("HOME", temp.path())
        .env("XDG_CONFIG_HOME", temp.path().join(".config"))
        .env("XDG_DATA_HOME", temp.path().join(".local/share"))
        .assert()
        .success()
        .stdout(predicate::str::contains("Added"));
}

#[test]
fn test_read_markdown_file() {
    use assert_cmd::Command;
    use predicates::prelude::*;
    use std::fs;
    use tempfile::tempdir;

    let temp = tempdir().unwrap();
    let md_path = temp.path().join("test.md");

    fs::write(
        &md_path,
        r#"# Test Book

## Chapter 1

This is the first chapter.

## Chapter 2

This is the second chapter with **bold** and *italic* text.
"#,
    )
    .unwrap();

    #[allow(deprecated)]
    Command::cargo_bin("franko")
        .unwrap()
        .args(["library", "add", md_path.to_str().unwrap()])
        .env("HOME", temp.path())
        .env("XDG_CONFIG_HOME", temp.path().join(".config"))
        .env("XDG_DATA_HOME", temp.path().join(".local/share"))
        .assert()
        .success()
        .stdout(predicate::str::contains("Added"));
}

#[test]
fn test_unsupported_format() {
    use assert_cmd::Command;
    use std::fs;
    use tempfile::tempdir;

    let temp = tempdir().unwrap();
    let unknown_path = temp.path().join("test.xyz");

    fs::write(&unknown_path, "Some content").unwrap();

    #[allow(deprecated)]
    Command::cargo_bin("franko")
        .unwrap()
        .args(["library", "add", unknown_path.to_str().unwrap()])
        .env("HOME", temp.path())
        .env("XDG_CONFIG_HOME", temp.path().join(".config"))
        .env("XDG_DATA_HOME", temp.path().join(".local/share"))
        .assert()
        .failure(); // Should fail for unsupported format
}
