mod cli;
mod commands;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Start(_) => {
            println!("starting");
        }
        Commands::Sync(args) => {
            commands::sync::handle(args);
        }
    }
}
