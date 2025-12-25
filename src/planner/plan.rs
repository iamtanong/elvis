use std::path::PathBuf;
use std::time::SystemTime;

use crate::planner::action::Action;

#[derive(Debug)]
pub struct Plan {
    pub metadata: PlanMetadata,
    pub actions: Vec<Action>,
    pub warnings: Vec<PlanWarning>,
    pub errors: Vec<PlanError>,
    pub summary: PlanSummary,
}

#[derive(Debug)]
pub struct PlanMetadata {
    pub command: String,
    pub args: Vec<String>,
    pub working_dir: PathBuf,
    pub created_at: SystemTime,
}

#[derive(Debug, Default)]
pub struct PlanSummary {
    pub files_deleted: usize,
    pub dirs_deleted: usize,
    pub files_created: usize,
    pub files_moved: usize,
    pub warnings: usize,
    pub errors: usize,
}

#[derive(Debug)]
pub struct PlanWarning {
    pub kind: WarningKind,
    pub paths: Vec<PathBuf>,
    pub message: String,
}

#[derive(Debug)]
pub enum WarningKind {
    Overwrite,
    RecursiveDelete,
    LargeOperation,
    PermissionRisk,
}

#[derive(Debug)]
pub struct PlanError {
    pub kind: ErrorKind,
    pub path: Option<PathBuf>,
    pub message: String,
}

#[derive(Debug)]
pub enum ErrorKind {
    NotFound,
    PermissionDenied,
    InvalidPath,
    Unsupported,
}
