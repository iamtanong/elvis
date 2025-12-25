use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "elvis")]
#[command(author, version, about="File-system command preview tool", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    #[arg(long)]
    pub no_color: bool,

    #[arg(long)]
    pub summary_only: bool,

    #[arg(short, long)]
    pub max_entries: Option<usize>,

    #[arg(short = 'y', long)]
    pub yes: bool,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Touch {
        #[arg(required = true)]
        targets: Vec<PathBuf>,
    },

    Mv {
        #[arg(required = true)]
        sources: Vec<PathBuf>,

        #[arg(required = true)]
        target: PathBuf,

        #[arg(short, long)]
        force: bool,
    },

    Rm {
        #[arg(required = true)]
        targets: Vec<PathBuf>,

        #[arg(short, long)]
        recursive: bool,

        #[arg(short, long)]
        force: bool,
    },
}
