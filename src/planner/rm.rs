use std::path::{Path, PathBuf};
use std::{fs, time::SystemTime};

use walkdir::WalkDir;

use crate::planner::plan::CommandKind;
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

    fn handle_directory(
        &self,
        target: &Path,
        actions: &mut Vec<Action>,
        warnings: &mut Vec<PlanWarning>,
        errors: &mut Vec<PlanError>,
        summary: &mut PlanSummary,
    ) {
        if self.recursive {
            warnings.push(PlanWarning {
                kind: WarningKind::RecursiveDelete,
                paths: vec![target.to_path_buf()],
                message: "Recursive directory deletion".into(),
            });


            for entry in WalkDir::new(target)
                .contents_first(true)
                .follow_links(false)
                .into_iter()
                .filter_map(|e| e.ok())

            {
                let kind = if entry.file_type().is_dir() {
                    summary.dirs_deleted += 1;
                    FsObjectKind::Directory
                } else {
                    summary.files_deleted += 1;
                    FsObjectKind::File
                };

                actions.push(Action::Delete {
                    path: entry.into_path(),
                    kind,
                    recursive: false,
                });
            }
        } else {
            match fs::read_dir(target) {
                Ok(mut dir) => match dir.next() {
                    Some(Ok(_)) => {
                        errors.push(PlanError {
                            kind: ErrorKind::Unsupported,
                            path: Some(target.to_path_buf()),
                            message: "Is a directory (use -r)".into(),
                        });
                        return;
                    }
                    Some(Err(_)) => {

                    }
                    None => (),
                },
                Err(_) => {

                    if !self.force {
                        errors.push(PlanError {
                            kind: ErrorKind::PermissionDenied,
                            path: Some(target.to_path_buf()),
                            message: "Cannot read directory".into(),
                        });
                        return;
                    }
                }
            }

            summary.dirs_deleted += 1;
            actions.push(Action::Delete {
                path: target.to_path_buf(),
                kind: FsObjectKind::Directory,
                recursive: false,
            });
        }
    }
}

impl super::traits::Planner for RmPlanner {
    fn plan(&self) -> Plan {
        let estimated_actions = self.targets.len() * (if self.recursive { 10 } else { 1 });
        let mut actions = Vec::with_capacity(estimated_actions);
        let mut warnings = Vec::new();
        let mut errors = Vec::new();
        let mut summary = PlanSummary::default();

        for target in &self.targets {
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
                Err(e) => {
                    if self.force {

                        warnings.push(PlanWarning {
                            kind: WarningKind::RecursiveDelete,
                            paths: vec![target.clone()],
                            message: format!("Cannot stat, attempting to remove anyway: {}", e)
                                .into(),
                        });

                        summary.files_deleted += 1;
                        actions.push(Action::Delete {
                            path: target.clone(),
                            kind: FsObjectKind::File,
                            recursive: false,
                        });
                    } else {
                        errors.push(PlanError {
                            kind: ErrorKind::PermissionDenied,
                            path: Some(target.clone()),
                            message: format!("Permission denied: {}", e).into(),
                        });
                    }
                    continue;
                }
            };

            if metadata.is_dir() {
                self.handle_directory(
                    target,
                    &mut actions,
                    &mut warnings,
                    &mut errors,
                    &mut summary,
                );
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
                command: CommandKind::Rm,
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
