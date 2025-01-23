use std::{fmt::Display, fs, io::Result};

use clap::Parser;
use color_eyre::owo_colors::OwoColorize;

#[derive(Parser, Debug)]
pub struct GrepCommand {
    #[arg(value_name = "PATTERN")]
    pattern: String,

    #[arg(value_name = "FILE", value_delimiter = ' ', num_args = 1..)]
    files: Vec<String>,
}

impl GrepCommand {
    pub fn exec(&self) -> Result<GrepCommandResult> {
        let mut entries = Vec::new();
        for file in &self.files {
            let contents = fs::read_to_string(file)?;

            for line in contents.lines() {
                if line.contains(&self.pattern) {
                    entries.push(GrepCommandMatch::new(
                        line,
                        if self.files.len() > 1 {
                            Some(file)
                        } else {
                            None
                        },
                    ));
                }
            }
        }

        Ok(GrepCommandResult::new(entries, self.pattern.clone()))
    }
}

pub struct GrepCommandMatch {
    line: String,
    file: Option<String>,
}

impl GrepCommandMatch {
    pub fn new(line: &str, file: Option<&str>) -> Self {
        let file = match file {
            Some(file_path) => Some(String::from(file_path)),
            None => None,
        };

        Self {
            line: String::from(line),
            file,
        }
    }
}

pub struct GrepCommandResult {
    pattern: String,
    matches: Vec<GrepCommandMatch>,
}

impl GrepCommandResult {
    pub fn new(matches: Vec<GrepCommandMatch>, pattern: String) -> Self {
        Self { matches, pattern }
    }
}

impl Display for GrepCommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, m) in self.matches.iter().enumerate() {
            let GrepCommandMatch { file, line } = m;

            if let Some(file) = file {
                write!(f, "{}{}", file.purple(), ":".cyan())?;
            }

            let byte_index = line.find(&self.pattern).unwrap();
            write!(
                f,
                "{}{}{}",
                &line[..byte_index],
                self.pattern.red(),
                &line[(byte_index) + self.pattern.len()..]
            )?;

            if idx != self.matches.len() - 1 {
                write!(f, "\n")?;
            }
        }

        Ok(())
    }
}
