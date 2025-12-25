use clap::Parser;
use elvis::cli::{self, args::Cli};

fn main() {
    let cli = Cli::parse();

    if let Err(err) = cli::run(cli) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
