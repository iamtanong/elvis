use std::fs;

use assert_cmd::{Command, cargo};
use tempfile::tempdir;

/// Touch_File_Success
///
/// `elvis -y touch <file>`
#[test]
fn touch_file_success() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("a.txt");

    let mut cmd = Command::new(cargo::cargo_bin!());
    cmd.args(["-y", "touch", file.to_str().unwrap()])
        .assert()
        .success();

    assert!(file.exists());
}

/// Touch_MultipleFiles_Success
///
/// `elvis -y touch <file1> <file2> ...`
#[test]
fn touch_files_success() {
    let dir = tempdir().unwrap();
    let file1 = dir.path().join("a.txt");
    let file2 = dir.path().join("b.txt");
    let file3 = dir.path().join("c.txt");

    let mut cmd = Command::new(cargo::cargo_bin!());
    cmd.args([
        "-y",
        "touch",
        file1.to_str().unwrap(),
        file2.to_str().unwrap(),
        file3.to_str().unwrap(),
    ])
    .assert()
    .success();

    assert!(file1.exists());
    assert!(file2.exists());
    assert!(file3.exists());
}

/// Touch_ExistedFile_Success
///
/// *Should get warning, but able to run successfully*
///
/// `elvis -y touch <existed_file>`
#[test]
fn touch_existed_file_success() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("a.txt");
    fs::write(&file, "hello").unwrap();

    let mut cmd = Command::new(cargo::cargo_bin!());
    cmd.args(["-y", "touch", file.to_str().unwrap()])
        .assert()
        .success();
}
