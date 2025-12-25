use std::{collections::HashMap, path::Path};

use console::style;

use crate::{
    planner::{
        action::{Action, FsObjectKind},
        plan::Plan,
    },
    printer::options::PrinterOptions,
};

#[derive(Debug)]
pub struct PrettyPrinter;

impl PrettyPrinter {
    pub fn print(plan: &Plan, options: &PrinterOptions) {
        Self::print_errors(plan);
        Self::print_summary(plan);

        if options.summary_only {
            return;
        }

        Self::print_warnings(plan);
        Self::print_actions(plan, options);
    }

    fn print_errors(plan: &Plan) {
        if plan.errors.is_empty() {
            return;
        }

        println!("{}", style("Errors").red());
        for err in plan.errors.iter() {
            match &err.path {
                Some(p) => println!("  {} - {}", p.display(), err.message),
                None => println!("  - {}", err.message),
            }
        }
        println!();
    }

    fn print_warnings(plan: &Plan) {
        if plan.warnings.is_empty() {
            return;
        }

        println!("{}", style("Warnings").yellow());
        for warn in plan.warnings.iter() {
            println!("  - {}", warn.message);
        }
        println!();
    }

    fn print_summary(plan: &Plan) {
        let s = &plan.summary;

        println!("Plan summary:");
        if s.files_deleted > 0 || s.dirs_deleted > 0 {
            println!(
                "  Delete: {} files, {} directories",
                s.files_deleted, s.dirs_deleted
            );
        }

        if s.files_created > 0 {
            println!("  Create: {} files", s.files_created);
        }

        if s.files_moved > 0 {
            println!("  Move: {} files", s.files_moved);
        }

        println!("Warnings: {}", s.warnings);
        println!("Errors: {}", s.errors);
        println!();
    }

    fn print_actions(plan: &Plan, options: &PrinterOptions) {
        let mut groups = HashMap::<&'static str, Vec<&Action>>::new();

        for action in &plan.actions {
            let key = match action {
                Action::Create { .. } => "Create",
                Action::Modify { .. } => "Modify",
                Action::Move { .. } => "Move",
                Action::Delete { .. } => "Delete",
            };

            groups.entry(key).or_default().push(action);
        }

        for (group, action) in groups {
            println!("{}:", group);

            let total = action.len();
            let limit = options.max_entries.min(total);

            for action in action.iter().take(limit) {
                Self::print_action(action, options);
            }

            if total > limit {
                println!("  ... ({} more)", total - limit);
            }

            println!();
        }
    }

    fn print_action(action: &Action, options: &PrinterOptions) {
        match action {
            Action::Create { path, kind, .. } => {
                println!(
                    "{}  {}{}",
                    style("C").green(),
                    Self::rel_path(path, options),
                    Self::kind_suffix(*kind)
                )
            }
            Action::Modify { path, description } => {
                println!(
                    "{}  {} ({})",
                    style("M").yellow(),
                    Self::rel_path(path, options),
                    description
                )
            }
            Action::Move { from, to, .. } => {
                println!(
                    "{}  {} -> {}",
                    style("M").yellow(),
                    Self::rel_path(from, options),
                    Self::rel_path(to, options)
                )
            }
            Action::Delete { path, kind, .. } => {
                println!(
                    "{}  {}{}",
                    style("D").red(),
                    Self::rel_path(path, options),
                    Self::kind_suffix(*kind)
                )
            }
        }
    }

    fn rel_path(path: &Path, options: &PrinterOptions) -> String {
        path.strip_prefix(&options.cwd)
            .unwrap_or(path)
            .display()
            .to_string()
    }

    fn kind_suffix(kind: FsObjectKind) -> &'static str {
        match kind {
            FsObjectKind::Directory => "/",
            _ => "",
        }
    }
}
