use std::fmt::Display;

use clap::Parser;

// TODO: Add backslash escaped characters support
// const ALERT_BELL: char = '\x07';
// const BACKSPACE: char = '\x08';

const LONG_ABOUT: &str = "
Write arguments to the standard output.

Display the ARGs, separated by a single space character and followed by a
newline, on the standard output.";

const ABOUT: &str = "echo: echo [-neE] [arg ...]";

#[derive(Parser, Debug)]
#[command(version = "0.0.0", about = ABOUT, long_about = LONG_ABOUT)]
pub struct EchoCommand {
    #[arg(short = 'n', help = "do not append a newline")]
    pub disable_new_line: bool,

    #[arg(value_name = "arg ...")]
    pub value: String,
}

impl EchoCommand {
    pub fn exec(&self) -> EchoCommandResult {
        let mut output_str = String::from(&self.value);
        if !self.disable_new_line {
            output_str.push('\n');
        }

        EchoCommandResult::new(output_str)
    }
}

pub struct EchoCommandResult {
    echo_val: String,
}

impl EchoCommandResult {
    pub fn new(echo_val: String) -> Self {
        Self { echo_val }
    }
}

impl Display for EchoCommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.echo_val)
    }
}
