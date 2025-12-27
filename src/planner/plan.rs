use std::path::PathBuf;
use std::time::SystemTime;

use crate::planner::action::Action;

#[doc = "Plan data"]
#[derive(Debug)]
pub struct Plan {
    pub metadata: PlanMetadata,
    pub actions: Vec<Action>,
    pub warnings: Vec<PlanWarning>,
    pub errors: Vec<PlanError>,
    pub summary: PlanSummary,
}

#[doc = "Command metadata"]
#[derive(Debug)]
pub struct PlanMetadata {
    pub command: CommandKind,
    pub working_dir: PathBuf,
    pub created_at: SystemTime,
}

#[doc = "Command kind simplified for metadata"]
#[derive(Debug)]
pub enum CommandKind {
    Touch,
    Mv,
    Rm,
}

#[doc = "Affected summary"]
#[derive(Debug, Default)]
pub struct PlanSummary {
    pub files_deleted: usize,
    pub dirs_deleted: usize,
    pub files_created: usize,
    pub files_moved: usize,
    pub warnings: usize,
    pub errors: usize,
}

#[doc = "Command Warning"]
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

#[doc = "Command Error"]
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
