use chrono::{DateTime, Local};
use colored::{ColoredString, Colorize};
use numfmt::{Formatter, Numeric, Precision, Scales};
use std::{
    fmt::Debug,
    fs::{self, DirEntry, Metadata},
    io::Result,
    os::{
        linux::fs::MetadataExt,
        unix::fs::{FileTypeExt, PermissionsExt},
    },
    time::{SystemTime, UNIX_EPOCH},
};

use clap::Parser;
use users::{get_group_by_gid, get_user_by_uid, Group, User};

#[derive(Parser, Debug)]
pub struct LsCommand {
    #[arg(
        short = 'a',
        long = "all",
        help = "do not ignore entries starting with ."
    )]
    all: bool,

    #[arg(
        short = 'A',
        long = "almost-all",
        help = "do not list implied . and .."
    )]
    almost_all: bool,

    #[arg(short = 'l', help = "use a long listing format")]
    long_format: bool,

    #[arg(
        short = 'i',
        long = "human-readable",
        help = "with -l and -s, print sizes like 1K 234M 2G etc."
    )]
    human_readable: bool,

    #[arg(value_name = "PATH", default_value = ".")]
    path: String,
}

impl LsCommand {
    pub fn human_readable(&self) -> bool {
        self.human_readable
    }

    fn almost_all(&self) -> bool {
        self.all || self.almost_all
    }

    fn process_blocks(total_blocks: Option<u64>, blocks: u64) -> Option<u64> {
        let total_blocks_so_far = match total_blocks {
            Some(v) => v,
            None => 0,
        };

        // block count is returned in 512 byte blocks but linux counts 1024 size blocks, so divided by 2
        Some(total_blocks_so_far + blocks / 2)
    }

    pub fn exec(&self) -> Result<LsOuptut> {
        let mut dir: Vec<_> = fs::read_dir(&self.path)?.collect();
        dir.sort_by(|a, b| {
            a.as_ref()
                .clone()
                .unwrap()
                .file_name()
                .cmp(&b.as_ref().clone().unwrap().file_name())
        });

        let separator = if self.long_format { "\n" } else { "  " };

        let mut total_blocks = None;
        let mut output = Vec::new();

        if self.all {
            let current_dir_extra = if self.long_format {
                let metadata = fs::metadata(".")?;
                total_blocks = Self::process_blocks(total_blocks, metadata.st_blocks());

                Some(LsEntryExtra::from(metadata, self.human_readable)?)
            } else {
                None
            };

            let parent_dir_extra = if self.long_format {
                let metadata = fs::metadata("..")?;
                total_blocks = Self::process_blocks(total_blocks, metadata.st_blocks());

                Some(LsEntryExtra::from(metadata, self.human_readable)?)
            } else {
                None
            };

            output.push(LsEntry::new(".".blue(), current_dir_extra));
            output.push(LsEntry::new("..".blue(), parent_dir_extra));
        }

        for entry in dir {
            let entry = entry?;

            let file_or_dir_name = get_colorize_file_name(&entry)?;

            if !self.almost_all() && file_or_dir_name.starts_with(".") {
                continue;
            }

            let extra = if self.long_format {
                let metadata = entry.metadata()?;
                total_blocks = Self::process_blocks(total_blocks, metadata.st_blocks());

                Some(LsEntryExtra::from(metadata, self.human_readable)?)
            } else {
                None
            };

            output.push(LsEntry::new(file_or_dir_name, extra));
        }

        Ok(LsOuptut::new(
            total_blocks,
            output,
            separator,
            self.human_readable,
        ))
    }
}

pub struct LsOuptut<'a> {
    total_blocks: Option<u64>,
    entries: Vec<LsEntry>,
    separator: &'a str,
    human_readable: bool,
    step_1_formatter: Formatter,
    step_2_formatter: Formatter,
}

impl<'a> LsOuptut<'a> {
    fn new(
        total_blocks: Option<u64>,
        entries: Vec<LsEntry>,
        separator: &'a str,
        human_readable: bool,
    ) -> Self {
        Self {
            total_blocks,
            entries,
            separator,
            human_readable,
            step_1_formatter: Formatter::new()
                .scales(Scales::new(1024, vec![" K", " M", " G", " T", " P"]).unwrap())
                .precision(Precision::Significance(4)),
            step_2_formatter: Formatter::new().precision(Precision::Significance(2)),
        }
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

    pub fn total_blocks_str(&self) -> Option<String> {
        if let Some(total_blocks) = self.total_blocks() {
            return Some(format!(
                "total {}",
                if self.human_readable {
                    format_value(
                        self.step_1_formatter.clone(),
                        self.step_2_formatter.clone(),
                        total_blocks,
                    )
                } else {
                    format!("{}", total_blocks)
                }
            ));
        }

        None
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
            return Some(&extra.metadata);
        }

        None
    }

    pub fn file_or_dir_name(&self) -> &ColoredString {
        &self.file_or_dir_name
    }

    pub fn links(&self) -> Option<u64> {
        if let Some(extra) = &self.extra {
            return Some(extra.links);
        }

        None
    }

    pub fn user(&self) -> Option<&User> {
        if let Some(extra) = &self.extra {
            return extra.user.as_ref();
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
    human_readable: bool,
    step_1_formatter: Formatter,
    step_2_formatter: Formatter,
}

impl LsEntryExtra {
    pub fn new(
        metadata: String,
        links: u64,
        user: Option<User>,
        group: Option<Group>,
        size: u64,
        modified: SystemTime,
        human_readable: bool,
    ) -> Self {
        LsEntryExtra {
            metadata,
            links,
            user,
            group,
            size,
            modified,
            human_readable,
            step_1_formatter: Formatter::new()
                .scales(Scales::new(1024, vec!["", " K", " M", " G", " T", " P"]).unwrap())
                .precision(Precision::Significance(4)),
            step_2_formatter: Formatter::new().precision(Precision::Significance(2)),
        }
    }

    pub fn from(metadata: Metadata, human_readable: bool) -> Result<Self> {
        Ok(LsEntryExtra::new(
            Self::get_permissions_string(&metadata),
            metadata.st_nlink(),
            get_user_by_uid(metadata.st_uid()),
            get_group_by_gid(metadata.st_gid()),
            metadata.st_size(),
            metadata.modified()?,
            human_readable,
        ))
    }
    fn get_permissions_string(metadata: &Metadata) -> String {
        let mut string = String::new();

        string.push(if metadata.is_dir() { 'd' } else { '-' });

        let mode = metadata.permissions().mode() % 512;
        let first_part = mode / 64;
        let second_part = mode % 64 / 8;
        let third_part = mode % 8;

        string.push_str(&Self::get_permissions_substring(first_part));
        string.push_str(&Self::get_permissions_substring(second_part));
        string.push_str(&Self::get_permissions_substring(third_part));

        string
    }

    fn get_permissions_substring(mode_part: u32) -> String {
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
        let mut size_str = String::new();

        let size_str_val = if self.human_readable && self.size() > 1024 {
            format_value(
                self.step_1_formatter.clone(),
                self.step_2_formatter.clone(),
                self.size(),
            )
        } else {
            format!("{}", self.size())
        };

        let num_spaces_needed = max_str_len.saturating_sub(size_str_val.len());
        for _ in 0..num_spaces_needed {
            size_str.push(' ');
        }
        size_str.push_str(&size_str_val);

        size_str
    }

    pub fn modified_str(&self) -> String {
        let mut modified_str = String::new();

        let seconds = self.modified().duration_since(UNIX_EPOCH).unwrap();
        let datetime = DateTime::from_timestamp(seconds.as_secs().try_into().unwrap(), 0)
            .unwrap()
            .naive_utc();
        let datetime =
            DateTime::<Local>::from_naive_utc_and_offset(datetime, Local::now().offset().clone());
        let datetime = datetime.format("%b %e %H:%M");
        modified_str.push_str(&format!("{}", datetime));

        modified_str
    }
}

fn format_value<T: Numeric>(
    mut step_1_formatter: Formatter,
    mut step_2_formatter: Formatter,
    value: T,
) -> String {
    let formatted_size = step_1_formatter.fmt2(value);

    let split = formatted_size.split(" ");
    let mut num_str = "";
    let mut unit_str = "";
    for (idx, part) in split.into_iter().enumerate() {
        if idx == 0 {
            num_str = part;
        } else {
            unit_str = part;
        }
    }
    let size: f64 = num_str.parse().unwrap();
    let size = if size < 10.0 {
        (size * 10.0).ceil() / 10.0
    } else {
        size.ceil()
    };

    format!("{}{}", step_2_formatter.fmt2(size), unit_str,)
}

fn get_colorize_file_name(entry: &DirEntry) -> Result<ColoredString> {
    let mut file_name = entry.file_name().into_string().unwrap();
    if file_name.contains(" ") {
        file_name = format!("'{}'", file_name);
    }
    let file_type = entry.file_type()?;

    if file_type.is_dir() {
        Ok(file_name.blue())
    } else if file_type.is_symlink() {
        Ok(file_name.cyan())
    } else if file_type.is_block_device() || file_type.is_char_device() {
        Ok(file_name.yellow().on_black())
    } else {
        Ok(file_name.white())
    }
}
