use assert_cmd::{Command, cargo};
use std::fs;
use tempfile::tempdir;

// TODO: Nice-to-have test cases for `rm`:
// - `rm_no_permissions`: Try to remove a file without permissions.
// - `rm_interactive_confirmation_no`: Run without `-y` and pipe "n" to stdin to test cancellation.

/// Rm_File_Success
///
/// `elvis -y rm <file>`
#[test]
fn rm_file_success() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("a.txt");
    fs::write(&file, "hello").unwrap();

    let mut cmd = Command::new(cargo::cargo_bin!());
    cmd.args(["-y", "rm", file.to_str().unwrap()])
        .assert()
        .success();

    assert!(!file.exists(), "File should be removed");
}

/// Rm_DirWithoutRecursiveFlag_Error
///
/// `elvis rm <dir>`
#[test]
fn rm_dir_without_recursive_flag_error() {
    let dir = tempdir().unwrap();
    let sub = dir.path().join("sub");
    fs::create_dir(&sub).unwrap();
    let file = sub.join("a.txt");
    fs::write(&file, "content").unwrap();

    let mut cmd = Command::new(cargo::cargo_bin!());
    cmd.args(["rm", sub.to_str().unwrap()]).assert().failure();
}

/// Rm_DirWithRecursiveFlag_Success
///
/// `elvis -y rm -r <dir>`
#[test]
fn rm_dir_recursive_success() {
    let dir = tempdir().unwrap();
    let sub = dir.path().join("sub");
    fs::create_dir(&sub).unwrap();

    let mut cmd = Command::new(cargo::cargo_bin!());
    cmd.args(["-y", "rm", "-r", dir.path().to_str().unwrap()])
        .assert()
        .success();

    assert!(!sub.exists(), "Subdirectory should be removed recursively");
    assert!(
        !dir.path().exists(),
        "Parent directory should be removed recursively"
    )
}

/// Rm_SourceNotFound_Error
#[test]
fn rm_source_not_found() {
    let dir = tempdir().unwrap();
    let non_existent_file = dir.path().join("non_existent.txt");

    let mut cmd = Command::new(cargo::cargo_bin!());
    cmd.args(["rm", non_existent_file.to_str().unwrap()])
        .assert()
        .failure();
}

/// Rm_MultipleFiles_Success
#[test]
fn rm_multiple_files() {
    let dir = tempdir().unwrap();
    let file1 = dir.path().join("a.txt");
    let file2 = dir.path().join("b.txt");
    fs::write(&file1, "a").unwrap();
    fs::write(&file2, "b").unwrap();

    let mut cmd = Command::new(cargo::cargo_bin!());
    cmd.args(["-y", "rm", file1.to_str().unwrap(), file2.to_str().unwrap()])
        .assert()
        .success();

    assert!(!file1.exists(), "File 1 should be removed");
    assert!(!file2.exists(), "File 2 should be removed");
}

/// Rm_EmptyDirWithoutRecursiveFlag_Success
#[test]
fn rm_empty_dir_without_recursive_flag_success() {
    let dir = tempdir().unwrap();
    let empty_dir = dir.path().join("empty");
    fs::create_dir(&empty_dir).unwrap();

    let mut cmd = Command::new(cargo::cargo_bin!());
    cmd.args(["-y", "rm", empty_dir.to_str().unwrap()])
        .assert()
        .success();

    assert!(!empty_dir.exists(), "Empty directory should be removed");
}
