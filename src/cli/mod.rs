use std::{env, io};

use crate::{
    executor::{execution::Executor, options::ExecutorOptions},
    planner::{mv::MvPlanner, rm::RmPlanner, touch::TouchPlanner, traits::Planner},
    printer::{options::PrinterOptions, pretty::PrettyPrinter},
};

use self::args::{Cli, Command};

pub mod args;

pub fn run(cli: Cli) -> io::Result<()> {
    let cwd = env::current_dir()?;

    let planner: Box<dyn Planner> = match cli.command {
        Command::Touch { targets } => Box::new(TouchPlanner::new(targets, cwd.clone())),
        Command::Mv {
            sources,
            target,
            force,
        } => Box::new(MvPlanner::new(sources, target, force, cwd.clone())),
        Command::Rm {
            targets,
            recursive,
            force,
        } => Box::new(RmPlanner::new(targets, recursive, force, cwd.clone())),
    };

    let plan = planner.plan();

    let printer_opts = PrinterOptions {
        summary_only: cli.summary_only,
        cwd: cwd.clone(),
        ..Default::default()
    };

    PrettyPrinter::print(&plan, &printer_opts);

    let exec_opts = ExecutorOptions {
        assume_yes: cli.yes,
    };

    Executor::execute(&plan, &exec_opts)
}
