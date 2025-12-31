use std::fs;

use assert_cmd::{Command, cargo};
use tempfile::tempdir;

// TODO: Nice-to-have test cases for `mv`:
// - `mv_no_permissions_source`: Test moving a file from a read-only directory.
// - `mv_no_permissions_target`: Test moving a file to a read-only directory.
// - `mv_interactive_confirmation_no`: Run without `-y` and pipe "n" to stdin to test cancellation.
// - `mv_hidden_files`: Test moving hidden files (dotfiles).

/// Mv_File_Success
///
/// This test moves a file into a directory.
///
/// `elvis -y mv <from> <to>`
#[test]
fn mv_file_success() {
    let dir1 = tempdir().unwrap();
    let file1 = dir1.path().join("a.txt");
    fs::write(&file1, "hello").unwrap();

    let dir2 = tempdir().unwrap();
    let file2 = dir2.path().join("a.txt");

    let mut cmd = Command::new(cargo::cargo_bin!());
    cmd.args([
        "-y",
        "mv",
        file1.to_str().unwrap(),
        dir2.path().to_str().unwrap(),
    ])
    .assert()
    .success();

    assert!(!file1.exists(), "Source file should not exist after move");
    assert!(
        file2.exists(),
        "File should exist in the destination directory"
    );
}

/// Mv_MultipleFile_Success
///
/// `elvis -y mv <from1> <from2> ... <to>`
#[test]
fn mv_files_success() {
    let dir1 = tempdir().unwrap();
    let file1 = dir1.path().join("a.txt");
    let file2 = dir1.path().join("b.txt");
    let file3 = dir1.path().join("c.txt");
    fs::write(&file1, "hello1").unwrap();
    fs::write(&file2, "hello2").unwrap();
    fs::write(&file3, "hello3").unwrap();

    let dir2 = tempdir().unwrap();
    let dest1 = dir2.path().join("a.txt");
    let dest2 = dir2.path().join("b.txt");
    let dest3 = dir2.path().join("c.txt");

    let mut cmd = Command::new(cargo::cargo_bin!());
    cmd.args([
        "-y",
        "mv",
        file1.to_str().unwrap(),
        file2.to_str().unwrap(),
        file3.to_str().unwrap(),
        dir2.path().to_str().unwrap(),
    ])
    .assert()
    .success();

    assert!(!file1.exists(), "Source file 1 should not exist");
    assert!(!file2.exists(), "Source file 2 should not exist");
    assert!(!file3.exists(), "Source file 3 should not exist");

    assert!(dest1.exists(), "Destination file 1 should exist");
    assert!(dest2.exists(), "Destination file 2 should exist");
    assert!(dest3.exists(), "Destination file 3 should exist");
}

/// Mv_FileSameRoot_Success
///
/// *aka. rename file*
///
/// `elvis -y mv <old_name_file> <new_name_file>`
#[test]
fn rename_file_success() {
    let dir = tempdir().unwrap();
    let file1 = dir.path().join("a.txt");
    fs::write(&file1, "hello").unwrap();

    let file2 = dir.path().join("rename.txt");

    let mut cmd = Command::new(cargo::cargo_bin!());
    cmd.args(["-y", "mv", file1.to_str().unwrap(), file2.to_str().unwrap()])
        .assert()
        .success();

    assert!(
        !file1.exists(),
        "Original file should not exist after rename"
    );
    assert!(file2.exists(), "Renamed file should exist");
}

/// Mv_DirSameRoot_Success
///
/// This test moves a directory into another directory, not a rename.
///
/// `elvis -y mv <old_name_dir> <new_name_dir>`
#[test]
fn rename_dir_success() {
    let dir1 = tempdir().unwrap();
    let file1 = dir1.path().join("a.txt");
    fs::write(&file1, "hello").unwrap();

    let dir2 = tempdir().unwrap();
    let dir1_name = dir1.path().file_name().unwrap();
    let new_dir1_path = dir2.path().join(dir1_name);
    let new_file1_path = new_dir1_path.join("a.txt");

    let mut cmd = Command::new(cargo::cargo_bin!());
    cmd.args([
        "-y",
        "mv",
        dir1.path().to_str().unwrap(),
        dir2.path().to_str().unwrap(),
    ])
    .assert()
    .success();

    assert!(!dir1.path().exists(), "Source directory should be removed");
    assert!(!file1.exists(), "File in source directory should be moved");
    assert!(dir2.path().exists(), "Destination directory should exist");
    assert!(
        new_dir1_path.exists(),
        "Source directory should be moved into destination"
    );
    assert!(
        new_file1_path.exists(),
        "File should be in the new directory location"
    );
}

/// Mv_SourceNotFound_Error
#[test]
fn mv_source_not_found() {
    let dir = tempdir().unwrap();
    let non_existent_file = dir.path().join("non_existent.txt");
    let target_dir = dir.path().join("target");
    fs::create_dir(&target_dir).unwrap();

    let mut cmd = Command::new(cargo::cargo_bin!());
    cmd.args([
        "mv",
        non_existent_file.to_str().unwrap(),
        target_dir.to_str().unwrap(),
    ])
    .assert()
    .failure();
}

/// Mv_TargetIsFileWithMultipleSources_Error
#[test]
fn mv_target_is_file_with_multiple_sources() {
    let dir = tempdir().unwrap();
    let file1 = dir.path().join("a.txt");
    let file2 = dir.path().join("b.txt");
    let target_file = dir.path().join("c.txt");
    fs::write(&file1, "a").unwrap();
    fs::write(&file2, "b").unwrap();
    fs::write(&target_file, "c").unwrap();

    let mut cmd = Command::new(cargo::cargo_bin!());
    cmd.args([
        "mv",
        file1.to_str().unwrap(),
        file2.to_str().unwrap(),
        target_file.to_str().unwrap(),
    ])
    .assert()
    .failure();
}

/// Mv_SourceAndDestinationAreSame_Error
#[test]
fn mv_source_and_destination_are_same() {
    let dir = tempdir().unwrap();
    let file1 = dir.path().join("a.txt");
    fs::write(&file1, "a").unwrap();

    let mut cmd = Command::new(cargo::cargo_bin!());
    cmd.args(["mv", file1.to_str().unwrap(), file1.to_str().unwrap()])
        .assert()
        .failure();
}
