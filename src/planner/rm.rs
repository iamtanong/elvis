use std::path::PathBuf;
use std::{fs, time::SystemTime};

use walkdir::WalkDir;

use crate::planner::{
    action::{Action, FsObjectKind},
    plan::{ErrorKind, Plan, PlanError, PlanMetadata, PlanSummary, PlanWarning, WarningKind},
};

pub struct RmPlanner {
    pub targets: Vec<PathBuf>,
    pub recursive: bool,
    pub force: bool,
    pub cwd: PathBuf,
}

impl RmPlanner {
    pub fn new(targets: Vec<PathBuf>, recursive: bool, force: bool, cwd: PathBuf) -> Self {
        Self {
            targets,
            recursive,
            force,
            cwd,
        }
    }
}

impl super::traits::Planner for RmPlanner {
    fn plan(&self) -> Plan {
        let mut actions = Vec::new();
        let mut warnings = Vec::new();
        let mut errors = Vec::new();
        let mut summary = PlanSummary::default();

        for target in self.targets.iter() {
            if !target.exists() {
                if !self.force {
                    errors.push(PlanError {
                        kind: ErrorKind::NotFound,
                        path: Some(target.clone()),
                        message: "No such file or directory".into(),
                    });
                }
                continue;
            }

            let metadata = match fs::symlink_metadata(target) {
                Ok(m) => m,
                Err(_) => {
                    errors.push(PlanError {
                        kind: ErrorKind::PermissionDenied,
                        path: Some(target.clone()),
                        message: "Permission denied".into(),
                    });
                    continue;
                }
            };

            if metadata.is_dir() {
                if !self.recursive {
                    errors.push(PlanError {
                        kind: ErrorKind::Unsupported,
                        path: Some(target.clone()),
                        message: "Is a directory (use -r)".into(),
                    });
                    continue;
                }

                warnings.push(PlanWarning {
                    kind: WarningKind::RecursiveDelete,
                    paths: vec![target.clone()],
                    message: "Recursive directory deletion".into(),
                });

                for entry in WalkDir::new(target)
                    .contents_first(true)
                    .into_iter()
                    .filter_map(Result::ok)
                {
                    let path = entry.path().to_path_buf();
                    let kind = if entry.file_type().is_dir() {
                        summary.dirs_deleted += 1;
                        FsObjectKind::Directory
                    } else {
                        summary.files_deleted += 1;
                        FsObjectKind::File
                    };

                    actions.push(Action::Delete {
                        path,
                        kind,
                        recursive: false,
                    });
                }
            } else {
                summary.files_deleted += 1;
                actions.push(Action::Delete {
                    path: target.clone(),
                    kind: FsObjectKind::File,
                    recursive: false,
                });
            }
        }

        summary.warnings = warnings.len();
        summary.errors = errors.len();

        Plan {
            metadata: PlanMetadata {
                command: "rm".into(),
                args: vec![],
                working_dir: self.cwd.clone(),
                created_at: SystemTime::now(),
            },
            actions,
            warnings,
            errors,
            summary,
        }
    }
}
