use std::{fmt::Display, fs, io::Result};

use clap::Parser;

#[derive(Parser, Debug)]
pub struct FindCommand {
    #[arg(value_name = "[path...]", default_value = ".")]
    path: String,
}

impl FindCommand {
    pub fn exec(&self) -> Result<FindCommandResult> {
        let mut entries = Vec::new();
        Self::traverse_and_add(&mut entries, self.path.clone())?;

        Ok(FindCommandResult::new(entries))
    }

    fn traverse_and_add(entries: &mut Vec<String>, path: String) -> Result<()> {
        entries.push(path.clone());

        for dir_entry in fs::read_dir(path)? {
            let dir_entry = dir_entry?;
            let file_type = dir_entry.file_type()?;
            let entry_path = dir_entry
                .path()
                .into_os_string()
                .into_string()
                .expect("Should parse to string");

            if file_type.is_dir() {
                Self::traverse_and_add(entries, entry_path)?;
            } else {
                entries.push(entry_path);
            }
        }

        Ok(())
    }
}

const SEPARATOR: &str = "\n";
pub struct FindCommandResult {
    entries: Vec<String>,
}

impl FindCommandResult {
    pub fn new(entries: Vec<String>) -> Self {
        FindCommandResult { entries }
    }
}

impl Display for FindCommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.entries.join(SEPARATOR))
    }
}
