use std::{char, fmt::Display};
use color_eyre::Result;

use clap::Parser;

// TODO: Add backslash escaped characters support
const ALERT_BELL: char = '\x07';
const BACKSPACE: char = '\x08';
const ESCAPE: char = '\x1B';
const LINE_FEED: char = '\x0A';
const VERTICAL_TAB: char = '\x0B';

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

    #[arg(short = 'e', help = "enable interpretation of backslash escapes")]
    pub enable_escaping: bool,

    #[arg(value_name = "arg ...")]
    pub value: String,
}

impl EchoCommand {
    pub fn parse_octal(output_str: &mut String, octal_val: &mut String) -> Result<()> {
        let ch = u8::from_str_radix(&octal_val, 8)?;
        output_str.push(ch as char);
        octal_val.clear();

        Ok(())
    }

    pub fn parse_hex(output_str: &mut String, hex_val: &mut String) -> Result<()> {
        let ch = u8::from_str_radix(&hex_val, 16)?;
        output_str.push(ch as char);
        hex_val.clear();

        Ok(())
    }

    pub fn exec(&self) -> Result<EchoCommandResult> {
        let mut output_str = String::new();
        let mut previous_backslash = false;
        let mut octal = false;
        let mut octal_val = String::new();
        let mut hex = false;
        let mut hex_val = String::new();

        for ch in self.value.chars() {
            if octal {
                octal_val.push(ch);

                if octal_val.len() >= 3 {
                    Self::parse_octal(&mut output_str, &mut octal_val)?;
                }

                continue;
            }

            if hex {
                hex_val.push(ch);

                if hex_val.len() >= 2 {
                    Self::parse_hex(&mut output_str, &mut hex_val)?;
                }

                continue;
            }

            if previous_backslash {
                match ch {
                    'a' => output_str.push(ALERT_BELL),
                    'b' => output_str.push(BACKSPACE),
                    'c' => return Ok(EchoCommandResult::new(output_str)),
                    'e' => output_str.push(ESCAPE),
                    'f' => output_str.push(LINE_FEED),
                    'n' => output_str.push('\n'),
                    'r' => output_str.push('\r'),
                    't' => output_str.push('\t'),
                    'v' => output_str.push(VERTICAL_TAB),
                    '0' => octal = true,
                    'x' => hex = true,
                    _ => {
                        output_str.push('\\');
                        output_str.push(ch);
                    }
                }

                previous_backslash = false;
                continue;
            }

            previous_backslash = false;

            if ch == '\\' {
                previous_backslash = true;
                continue;
            }

            output_str.push(ch);
        }

        if octal {
            if octal_val.len() >= 1 {
                Self::parse_octal(&mut output_str, &mut octal_val)?;
            } else {
                output_str.push_str("\\0");
            }
        }

        if hex {
            if hex_val.len() >= 1 {
                Self::parse_hex(&mut output_str, &mut hex_val)?;
            } else {
                output_str.push_str("\\x");
            }
        }

        if !self.disable_new_line {
            output_str.push('\n');
        }

        Ok(EchoCommandResult::new(output_str))
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
