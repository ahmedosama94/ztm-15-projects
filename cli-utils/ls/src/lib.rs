use std::{fs::{self, Metadata}, io::Result, os::{linux::fs::MetadataExt, unix::fs::PermissionsExt}};
use colored::{ColoredString, Colorize};

use clap::Parser;

#[derive(Parser, Debug)]
pub struct LsCommand {
    #[arg(short = 'a', long = "all", help = "do not ignore entries starting with .")]
    all: bool,

    #[arg(short = 'A', long = "almost-all", help = "do not list implied . and ..")]
    almost_all: bool,

    #[arg(short = 'l', help = "use a long listing format")]
    long_format: bool,

    #[arg(value_name = "PATH", default_value = ".")]
    path: String,
}

impl LsCommand {
    fn almost_all(&self) -> bool {
        self.all || self.almost_all
    }

    fn get_permissions_string(&self, metadata: Metadata) -> String {
        let mut string = String::new();

        string.push(if metadata.is_dir() { 'd' } else { '-' });

        let mode = metadata.permissions().mode() % 512;
        let first_part = mode / 64;
        let second_part = mode % 64 / 8;
        let third_part = mode % 8;

        string.push_str(&self.get_permissions_substring(first_part));
        string.push_str(&self.get_permissions_substring(second_part));
        string.push_str(&self.get_permissions_substring(third_part));

        string
    }

    fn get_permissions_substring(&self, mode_part: u32) -> String {
        let mut output = String::new();

        if mode_part & (1 << 2) > 0 {
            output.push('r');
        } else {
            output.push('-');
        }

        if mode_part & (1 << 1) > 0 {
            output.push('w');
        } else {
            output.push('-');
        }

        if mode_part & 1 > 0 {
            output.push('x');
        } else {
            output.push('-');
        }

        output
    }

    pub fn exec(&self) -> Result<LsOuptut> {
        let mut dir: Vec<_> = fs::read_dir(&self.path).unwrap().collect();
        dir.sort_by(|a, b| {
            a.as_ref().clone().unwrap().file_name().cmp(&b.as_ref().clone().unwrap().file_name())
        });

        let separator = if self.long_format { "\n" } else { "  " };

        let mut total_blocks = None;
        let mut output = Vec::new();
        if self.all {
            output.push(LsEntry::new(None, ".".blue()));
            output.push(LsEntry::new(None, "..".blue()));
        }

        for entry in dir {
            let entry = entry.unwrap();

            let file_or_dir_name = entry.file_name().into_string().unwrap();

            if !self.almost_all() && file_or_dir_name.starts_with(".") {
                continue;
            }

            let metadata = if self.long_format {
                let metadata = entry.metadata()?;
                let total_blocks_so_far = match total_blocks { Some(v) => v, None => 0 };

                // block count is returned in 512 byte blocks but linux counts 1024 size blocks, so divided by 2
                total_blocks = Some(total_blocks_so_far + (metadata.st_blocks() / 2));

                Some(self.get_permissions_string(metadata))
            } else {
                None
            };

            let file_or_dir_name = if entry.file_type().unwrap().is_dir() {
                file_or_dir_name.blue()
            } else {
                file_or_dir_name.white()
            };

            output.push(LsEntry::new(metadata, file_or_dir_name));
        }

        Ok(LsOuptut::new(total_blocks, output, separator))
    }
}

pub struct LsOuptut<'a> {
    total_blocks: Option<u64>,
    entries: Vec<LsEntry>,
    separator: &'a str,
}

impl<'a> LsOuptut<'a> {
    fn new(total_blocks: Option<u64>, entries: Vec<LsEntry>, separator: &'a str) -> Self {
        Self { total_blocks, entries, separator }
    }

    pub fn total_blocks(&self) -> Option<u64> {
        self.total_blocks
    }

    pub fn entries(&self) -> &Vec<LsEntry> {
        &self.entries
    }

    pub fn separator(&self) -> &str {
        &self.separator
    }
}

pub struct LsEntry {
    metadata: Option<String>,
    file_or_dir_name: ColoredString,
}

impl LsEntry {
    pub fn new(metadata: Option<String>, file_or_dir_name: ColoredString) -> Self {
        LsEntry { metadata, file_or_dir_name }
    }

    pub fn metadata(&self) -> &Option<String> {
        &self.metadata
    }

    pub fn file_or_dir_name(&self) -> &ColoredString {
        &self.file_or_dir_name
    }
}
