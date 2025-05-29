use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct SyncArgs {
    #[arg(short, long)]
    pub repos: Vec<PathBuf>,
}

pub fn handle(args: Args) {
    for repo in args.repos {
        println!("Fetching {}!", repo.display());
    }
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;

    #[test]
    fn test_command() {
        let mut cmd = Command::cargo_bin("work").unwrap();
        let assert = cmd.args(["sync", "--repos", "path/to/repo"]).assert();

        assert.success().code(0).stdout("Fetching path/to/repo!\n");
    }
}
