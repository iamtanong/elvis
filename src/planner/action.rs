use std::path::PathBuf;

#[derive(Debug)]
pub enum Action {
    Delete {
        path: PathBuf,
        kind: FsObjectKind,
        recursive: bool,
    },
    Move {
        from: PathBuf,
        to: PathBuf,
        overwrite: bool,
    },
    Create {
        path: PathBuf,
        kind: FsObjectKind,
    },
    Modify {
        path: PathBuf,
        description: String,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum FsObjectKind {
    File,
    Directory,
    Symlink,
}
