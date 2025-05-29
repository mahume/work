use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;

    #[test]
    fn test_command() {
        let mut cmd = Command::cargo_bin("work").unwrap();
        let assert = cmd.args(["start"]).assert();

        assert.success().code(0);
    }
}
