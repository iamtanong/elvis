use std::fs;
use std::thread;
use std::time::Duration;

use assert_cmd::{Command, cargo};
use tempfile::tempdir;

// TODO: Nice-to-have test cases for `touch`:
// - `touch_no_permissions`: Try to create a file in a read-only directory.

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

    assert!(file.exists(), "File should be created");
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

    assert!(file1.exists(), "File 1 should be created");
    assert!(file2.exists(), "File 2 should be created");
    assert!(file3.exists(), "File 3 should be created");
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

    assert!(
        file.exists(),
        "Existing file should still exist after touch"
    );
}

/// Touch_Updates_Modification_Time
#[test]
fn touch_updates_modification_time() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("a.txt");
    fs::write(&file, "hello").unwrap();

    let initial_metadata = fs::metadata(&file).unwrap();
    let initial_mod_time = initial_metadata.modified().unwrap();

    // Wait a bit to ensure the modification time will be different
    thread::sleep(Duration::from_secs(1));

    let mut cmd = Command::new(cargo::cargo_bin!());
    cmd.args(["-y", "touch", file.to_str().unwrap()])
        .assert()
        .success();

    let new_metadata = fs::metadata(&file).unwrap();
    let new_mod_time = new_metadata.modified().unwrap();

    assert!(
        new_mod_time > initial_mod_time,
        "Modification time should be updated"
    );
}
