use std::{fs, io};

use dialoguer::{Confirm, theme::ColorfulTheme};

use crate::{
    executor::options::ExecutorOptions,
    planner::{
        action::{Action, FsObjectKind},
        plan::Plan,
    },
};

pub struct Executor;

impl Executor {
    pub fn execute(plan: &Plan, options: &ExecutorOptions) -> io::Result<()> {
        Self::validate(plan)?;

        if !options.assume_yes {
            Self::confirm()?;
        }

        for action in plan.actions.iter() {
            Self::apply(action)?;
        }

        Ok(())
    }

    fn validate(plan: &Plan) -> io::Result<()> {
        if !plan.errors.is_empty() {
            return Err(io::Error::other("Cannot execute plan with errors"));
        }
        Ok(())
    }

    fn confirm() -> io::Result<()> {
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Confirm Execution")
            .default(true)
            .show_default(false)
            .interact()
            .unwrap()
        {
            Ok(())
        } else {
            Err(io::Error::other("User cancel command"))
        }
    }

    fn apply(action: &Action) -> io::Result<()> {
        match action {
            Action::Create { path, kind } => match kind {
                FsObjectKind::File => {
                    fs::File::create(path)?;
                }
                FsObjectKind::Directory => {
                    fs::create_dir(path)?;
                }
                FsObjectKind::Symlink => {
                    return Err(io::Error::new(
                        io::ErrorKind::Unsupported,
                        "Symlink creation not supported yet",
                    ));
                }
            },
            Action::Move {
                from,
                to,
                overwrite,
            } => {
                if *overwrite && to.exists() {
                    fs::remove_file(to)?;
                }
                fs::rename(from, to)?
            }
            Action::Delete { path, kind, .. } => match kind {
                FsObjectKind::File | FsObjectKind::Symlink => fs::remove_file(path)?,
                FsObjectKind::Directory => fs::remove_dir(path)?,
            },

            Action::Modify { .. } => {
                println!("To be implemented")
            }
        };

        Ok(())
    }
}
