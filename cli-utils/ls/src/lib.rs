use std::{fs::{self, Metadata}, io::Result, os::{linux::fs::MetadataExt, unix::fs::PermissionsExt}, time::{SystemTime, UNIX_EPOCH}};
use chrono::{DateTime, Local};
use colored::{ColoredString, Colorize};

use clap::Parser;
use users::{get_group_by_gid, get_user_by_uid, Group, User};

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

    fn get_permissions_string(&self, metadata: &Metadata) -> String {
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
        let mut dir: Vec<_> = fs::read_dir(&self.path)?.collect();
        dir.sort_by(|a, b| {
            a.as_ref().clone().unwrap().file_name().cmp(&b.as_ref().clone().unwrap().file_name())
        });

        let separator = if self.long_format { "\n" } else { "  " };

        let mut total_blocks = None;
        let mut output = Vec::new();
        if self.all {
            output.push(LsEntry::new(".".blue(), None));
            output.push(LsEntry::new("..".blue(), None));
        }

        for entry in dir {
            let entry = entry?;

            let file_or_dir_name = entry.file_name().into_string().unwrap();

            if !self.almost_all() && file_or_dir_name.starts_with(".") {
                continue;
            }

            let extra = if self.long_format {
                let metadata = entry.metadata()?;
                let total_blocks_so_far = match total_blocks { Some(v) => v, None => 0 };

                // block count is returned in 512 byte blocks but linux counts 1024 size blocks, so divided by 2
                total_blocks = Some(total_blocks_so_far + (metadata.st_blocks() / 2));

                Some(LsEntryExtra::new(
                    self.get_permissions_string(&metadata),
                    metadata.st_nlink(),
                    get_user_by_uid(metadata.st_uid()),
                    get_group_by_gid(metadata.st_gid()),
                    metadata.st_size(),
                    metadata.modified()?,
                ))
            } else {
                None
            };

            let file_or_dir_name = if entry.file_type()?.is_dir() {
                file_or_dir_name.blue()
            } else {
                file_or_dir_name.white()
            };

            output.push(LsEntry::new(file_or_dir_name, extra));
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

    pub fn max_size(&self) -> u64 {
        let mut max_size = 0;

        for entry in &self.entries {
            if let Some(extra) = entry.extra() {
                if extra.size() > max_size {
                    max_size = extra.size();
                }
            }
        }

        max_size
    }
}

pub struct LsEntry {
    file_or_dir_name: ColoredString,
    extra: Option<LsEntryExtra>,
}

impl LsEntry {
    pub fn new(file_or_dir_name: ColoredString, extra: Option<LsEntryExtra>) -> Self {
        LsEntry {
            file_or_dir_name,
            extra,
        }
    }

    pub fn extra(&self) -> Option<&LsEntryExtra> {
        self.extra.as_ref()
    }

    pub fn metadata(&self) -> Option<&str> {
        if let Some(extra) = &self.extra {
            return Some(&extra.metadata)
        }

        None
    }

    pub fn file_or_dir_name(&self) -> &ColoredString {
        &self.file_or_dir_name
    }

    pub fn links(&self) -> Option<u64> {
        if let Some(extra) = &self.extra {
            return Some(extra.links)
        }

        None
    }

    pub fn user(&self) -> Option<&User> {
        if let Some(extra) = &self.extra {
            return extra.user.as_ref()
        }

        None
    }
}

pub struct LsEntryExtra {
    metadata: String,
    links: u64,
    user: Option<User>,
    group: Option<Group>,
    size: u64,
    modified: SystemTime,
}

impl LsEntryExtra {
    pub fn new(metadata: String, links: u64, user: Option<User>, group: Option<Group>, size: u64, modified: SystemTime) -> Self {
        LsEntryExtra { metadata, links, user, group, size, modified }
    }

    pub fn metadata(&self) -> &str {
        &self.metadata
    }

    pub fn links(&self) -> u64 {
        self.links
    }

    pub fn user(&self) -> Option<&User> {
        self.user.as_ref()
    }

    pub fn group(&self) -> Option<&Group> {
        self.group.as_ref()
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn modified(&self) -> SystemTime {
        self.modified
    }

    pub fn size_str(&self, max_str_len: usize) -> String {
        let size_str_val = format!("{}", self.size());
        let num_spaces_needed = max_str_len - size_str_val.len();
        let mut size_str = String::new();
        for _ in 0..num_spaces_needed {
            size_str.push(' ');
        }
        size_str.push_str(&size_str_val);

        size_str
    }

    pub fn modified_str(&self) -> String {
        let mut modified_str = String::new();

        let seconds = self.modified().duration_since(UNIX_EPOCH).unwrap();
        let datetime = DateTime::from_timestamp(seconds.as_secs().try_into().unwrap(), 0).unwrap().naive_utc();
        let datetime = DateTime::<Local>::from_naive_utc_and_offset(datetime, Local::now().offset().clone());
        let datetime = datetime.format("%b %e %H:%M");
        modified_str.push_str(&format!("{}", datetime));

        modified_str
    }
}
