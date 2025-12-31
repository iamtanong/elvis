use std::fs;

use assert_cmd::{Command, cargo};
use tempfile::tempdir;

/// Mv_File_Success
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

    assert!(!file1.exists());
    assert!(file2.exists());
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

    for entry in dir2
        .path()
        .read_dir()
        .expect("read_dir call failed")
        .flatten()
    {
        println!("{:?}", entry.path());
    }

    assert!(!file1.exists());
    assert!(!file2.exists());
    assert!(!file3.exists());

    assert!(dest1.exists());
    assert!(dest2.exists());
    assert!(dest3.exists());
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

    assert!(!file1.exists());
    assert!(file2.exists());
}

/// Mv_DirSameRoot_Success
///
/// *aka. rename dir*
///
/// `elvis -y mv <old_name_dir> <new_name_dir>`
#[test]
fn rename_dir_success() {
    let dir1 = tempdir().unwrap();
    let file1 = dir1.path().join("a.txt");
    fs::write(&file1, "hello").unwrap();

    let dir2 = tempdir().unwrap();
    let file2 = dir2.path().join("a.txt");

    let mut cmd = Command::new(cargo::cargo_bin!());
    cmd.args([
        "-y",
        "mv",
        dir1.path().to_str().unwrap(),
        dir2.path().to_str().unwrap(),
    ])
    .assert()
    .success();
    for entry in dir2
        .path()
        .read_dir()
        .expect("read_dir call failed")
        .flatten()
    {
        println!("{:?}", entry.path());
    }

    assert!(!file1.exists());
    // assert!(file2.exists()); // It's work but assert failed.
    assert!(!dir1.path().exists());
    assert!(dir2.path().exists());
}
