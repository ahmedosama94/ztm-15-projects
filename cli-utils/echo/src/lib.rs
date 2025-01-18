use clap::Parser;

const LONG_ABOUT: &str = "
Write arguments to the standard output.

Display the ARGs, separated by a single space character and followed by a
newline, on the standard output.";

const ABOUT: &str = "echo: echo [-neE] [arg ...]";

#[derive(Parser, Debug)]
#[command(version = "0.0.0", about = ABOUT, long_about = LONG_ABOUT)]
pub struct EchoArgs {
    #[arg(short = 'n', help = "do not append a newline")]
    pub disable_new_line: bool,

    #[arg(value_name = "arg ...")]
    pub value: String,
}
