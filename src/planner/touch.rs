use crate::planner::{
    action::{Action, FsObjectKind},
    plan::{CommandKind, Plan, PlanMetadata, PlanSummary, PlanWarning, WarningKind},
};
use std::{path::PathBuf, time::SystemTime};

#[doc = "Planner for `touch`"]
pub struct TouchPlanner {
    pub targets: Vec<PathBuf>,
    pub cwd: PathBuf,
}

impl TouchPlanner {
    pub fn new(targets: Vec<PathBuf>, cwd: PathBuf) -> Self {
        Self { targets, cwd }
    }
}

impl super::traits::Planner for TouchPlanner {
    fn plan(&self) -> Plan {
        let mut actions = Vec::new();
        let mut warnings = Vec::new();
        let errors = Vec::new();
        let mut summary = PlanSummary::default();

        for target in self.targets.iter() {
            if target.exists() {
                warnings.push(PlanWarning {
                    kind: WarningKind::Overwrite,
                    paths: vec![target.clone()],
                    message: "Already existed".into(),
                });
                continue;
            }

            actions.push(Action::Create {
                path: target.clone(),
                kind: FsObjectKind::File,
            });

            summary.files_created += 1;
        }

        summary.warnings = warnings.len();
        summary.errors = errors.len();

        Plan {
            metadata: PlanMetadata {
                command: CommandKind::Touch,
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
