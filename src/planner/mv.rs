use std::{path::PathBuf, time::SystemTime};
use walkdir::WalkDir;

use crate::planner::{
    action::{Action, FsObjectKind},
    plan::{
        CommandKind, ErrorKind, Plan, PlanError, PlanMetadata, PlanSummary, PlanWarning,
        WarningKind,
    },
};

#[doc = "Planner for `mv`"]
pub struct MvPlanner {
    pub sources: Vec<PathBuf>,
    pub target: PathBuf,
    pub force: bool,
    pub cwd: PathBuf,
}

impl MvPlanner {
    pub fn new(sources: Vec<PathBuf>, target: PathBuf, force: bool, cwd: PathBuf) -> Self {
        Self {
            sources,
            target,
            force,
            cwd,
        }
    }
}

impl super::traits::Planner for MvPlanner {
    fn plan(&self) -> Plan {
        let mut actions = Vec::new();
        let mut warnings = Vec::new();
        let mut errors = Vec::new();
        let mut summary = PlanSummary::default();

        let target_exist = self.target.exists();
        let target_is_dir = target_exist && self.target.is_dir();

        if self.sources.len() > 1 {
            if !target_exist {
                errors.push(PlanError {
                    kind: ErrorKind::NotFound,
                    path: Some(self.target.clone()),
                    message: "Target does not exist".into(),
                });
            } else if !target_is_dir {
                errors.push(PlanError {
                    kind: ErrorKind::InvalidPath,
                    path: Some(self.target.clone()),
                    message: "Target must be a directory".into(),
                });
            }
        }

        for src in self.sources.iter() {
            if !src.exists() {
                errors.push(PlanError {
                    kind: ErrorKind::NotFound,
                    path: Some(src.clone()),
                    message: "Source does not exist".into(),
                });
                continue;
            }

            if src.is_dir() {
                let dest_dir = if target_is_dir {
                    self.target.join(src.file_name().unwrap())
                } else {
                    self.target.clone()
                };

                let mut dirs_to_delete = vec![];
                for entry in WalkDir::new(src).into_iter().filter_map(|e| e.ok()) {
                    let entry_path = entry.path();
                    let relative_path = entry_path.strip_prefix(src).unwrap();
                    let dest_path = dest_dir.join(relative_path);

                    if entry.file_type().is_dir() {
                        actions.push(Action::Create {
                            path: dest_path.clone(),
                            kind: FsObjectKind::Directory,
                        });
                        if !dest_path.exists() {
                            summary.dirs_created += 1;
                        }
                        dirs_to_delete.push(entry_path.to_path_buf());
                    } else {
                        let overwrite = dest_path.exists();
                        if overwrite {
                            warnings.push(PlanWarning {
                                kind: WarningKind::Overwrite,
                                paths: vec![dest_path.clone()],
                                message: "Dest will be overwrite".into(),
                            });
                        }
                        actions.push(Action::Move {
                            from: entry_path.to_path_buf(),
                            to: dest_path,
                            overwrite,
                        });
                        summary.files_moved += 1;
                    }
                }
                for dir in dirs_to_delete.iter().rev() {
                    actions.push(Action::Delete {
                        path: dir.clone(),
                        kind: FsObjectKind::Directory,
                        recursive: false,
                    });
                    summary.dirs_deleted += 1;
                }
            } else {
                let dest = if self.sources.len() > 1 || target_is_dir {
                    self.target.join(src.file_name().unwrap())
                } else {
                    self.target.clone()
                };

                if src == &dest {
                    errors.push(PlanError {
                        kind: ErrorKind::InvalidPath,
                        path: Some(src.clone()),
                        message: "Source and destination are the same".into(),
                    });
                    continue;
                }

                let overwrite = dest.exists();
                if overwrite {
                    warnings.push(PlanWarning {
                        kind: WarningKind::Overwrite,
                        paths: vec![dest.clone()],
                        message: "Dest will be overwrite".into(),
                    });
                }

                actions.push(Action::Move {
                    from: src.clone(),
                    to: dest,
                    overwrite,
                });

                summary.files_moved += 1;
            }
        }

        summary.warnings = warnings.len();
        summary.errors = errors.len();

        Plan {
            metadata: PlanMetadata {
                command: CommandKind::Mv,
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
