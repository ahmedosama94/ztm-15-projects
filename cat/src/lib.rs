use std::{error::Error, fs, io};

use clap::Parser;
use thiserror::Error;

const ABOUT: &str = "Concatenate FILE(s) to standard output.";

#[derive(Parser, Debug)]
#[command(version = "0.0.0", about = ABOUT, long_about = None)]
pub struct CatCommand {
    #[arg(short = 'b', long = "number-nonblank", help = "number nonempty output lines, overrides -n")]
    number_nonblank: bool,

    #[arg(short = 'n', long = "number", help = "number all output lines")]
    number: bool,

    #[arg(short = 'E', long = "show-ends", help = "display $ at the end of each line")]
    show_ends: bool,

    #[arg(short = 's', long = "squeeze-blank", help = "suppress repeated empty output lines")]
    squeeze_blank: bool,

    #[arg(short = 'T', long = "show-tabs", help = "display TAB characters as ^I")]
    show_tabs: bool,

    #[arg(value_name = "FILE(S)", value_delimiter = ' ', num_args = 1..)]
    file_paths: Vec<String>,
}

impl CatCommand {
    pub fn exec(&self) -> Result<String, Box<dyn Error>> {
        let mut line_number = 1;
        let mut output = String::new();

        for file_path in &self.file_paths {
            let contents = fs::read_to_string(file_path);

            if let Err(err) = contents {
                return Err(
                    Box::new(
                        CatError::NoSuchFile{
                            path: String::from(file_path),
                            io_err: err,
                        }
                    )
                );
            }

            let contents = contents.unwrap();

            let mut previous_line_empty = false;
            for line in contents.lines() {
                if self.number || (self.number_nonblank && line.len() > 0) {
                    let line_number_str = format!("{: >6}", line_number);
                    output.push_str(&format!("{}\t", line_number_str));
                    line_number += 1;
                }

                if line.len() == 0 && self.squeeze_blank && previous_line_empty {
                    previous_line_empty = line.len() == 0;
                    continue;
                }
                previous_line_empty = line.len() == 0;

                let line = if self.show_tabs && line.contains('\t') {
                    &line.replace('\t', "^I")
                } else {
                    line
                };

                output.push_str(&format!("{}", line));

                if self.show_ends {
                    output.push('$');
                }

                output.push('\n');
            }
        }

        Ok(output)
    }
}

#[derive(Error, Debug)]
pub enum CatError<String> {
    #[error("No such file \"{path}\"")]
    NoSuchFile {
        path: String,
        io_err: io::Error,
    }
}
