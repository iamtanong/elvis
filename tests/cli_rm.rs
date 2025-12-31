use assert_cmd::{Command, cargo};
use std::fs;
use tempfile::tempdir;

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

    assert!(!file.exists());
}

/// Rm_DirWithoutRecursiveFlag_Error
///
/// `elvis rm <dir>`
#[test]
fn rm_dir_without_recursive_flag_error() {
    let dir = tempdir().unwrap();
    let sub = dir.path().join("sub");
    fs::create_dir(&sub).unwrap();

    let mut cmd = Command::new(cargo::cargo_bin!());
    cmd.args(["rm", sub.to_str().unwrap()]).assert().failure();
}

/// Rm_DirWithRecursiveFlag_Success
///
/// `elvis -y rm -r <dir>`
#[test]
fn rm_remove_dir_after_confirmation() {
    let dir = tempdir().unwrap();
    let sub = dir.path().join("sub");
    fs::create_dir(&sub).unwrap();

    let mut cmd = Command::new(cargo::cargo_bin!());
    cmd.args(["-y", "rm", "-r", dir.path().to_str().unwrap()])
        .assert()
        .success();

    assert!(!sub.exists());
    assert!(!dir.path().exists())
}
