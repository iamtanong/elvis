use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "elvis")]
#[command(author, version, about="File-system command preview tool", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    #[doc = "Disable colored output"]
    #[arg(long)]
    pub no_color: bool,

    #[doc = "Only show a summary of changes, hiding specific paths"]
    #[arg(long)]
    pub summary_only: bool,

    #[doc = "Limit the preview to N entries [default: 50]"]
    #[arg(short, long)]
    pub max_entries: Option<usize>,

    #[doc = "Skip confirmation and execute immediately after preview"]
    #[arg(short = 'y', long)]
    pub yes: bool,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[doc = "Preview file creation or timestamp updates"]
    Touch {
        #[arg(required = true)]
        targets: Vec<PathBuf>,
    },

    #[doc = "Preview moving or renaming files/directories"]
    Mv {
        #[arg(required = true)]
        sources: Vec<PathBuf>,

        #[arg(required = true)]
        target: PathBuf,

        #[arg(short, long)]
        force: bool,
    },

    #[doc = "Preview the deletion of files/directories"]
    Rm {
        #[arg(required = true)]
        targets: Vec<PathBuf>,

        #[arg(short, long)]
        recursive: bool,

        #[arg(short, long)]
        force: bool,
    },
}
