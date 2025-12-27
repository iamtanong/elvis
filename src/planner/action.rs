use std::path::PathBuf;

#[doc = "Plan Action"]
#[derive(Debug)]
pub enum Action {
    Create {
        path: PathBuf,
        kind: FsObjectKind,
    },
    Move {
        from: PathBuf,
        to: PathBuf,
        overwrite: bool,
    },
    Modify {
        path: PathBuf,
        description: String,
    },
    Delete {
        path: PathBuf,
        kind: FsObjectKind,
        recursive: bool,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum FsObjectKind {
    File,
    Directory,
    Symlink,
}
