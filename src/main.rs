use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Sync(SyncArgs),
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct SyncArgs {
    #[arg(short, long)]
    repos: Vec<PathBuf>,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Sync(args) => {
            for repo in args.repos {
                println!("Fetching {}!", repo.display());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_cmd::Command;

    #[test]
    fn test_sync_command() {
        let mut cmd = Command::cargo_bin("work").unwrap();
        let assert = cmd.args(["sync", "--repos", "path/to/repo"]).assert();

        assert.success().code(0).stdout("Fetching path/to/repo!\n");
    }
}
