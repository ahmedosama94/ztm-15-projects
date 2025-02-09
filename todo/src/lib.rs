use std::fmt::Display;

use clap::{Args, Parser, Subcommand};
use color_eyre::{eyre::Result, owo_colors::OwoColorize};
use db::{
    create_todo_list,
    models::{TodoListItemRow, TodoListRow},
};
use derive_more::Display;
use sqlx::types::chrono::Local;
use std::error::Error;

mod db;

#[derive(Debug, Parser)]
pub struct TodoCli {
    #[command(subcommand)]
    subcommand: SubCommand,
}

#[derive(Clone, Debug, Subcommand)]
enum SubCommand {
    #[command()]
    Add(AddArgs),
    CreateTodoList(CreateTodoListArgs),
    Clear(ClearArgs),
    Done(DoneArgs),
    Edit(EditArgs),
    List(ListArgs),
    Remove(RemoveArgs),
    ShowLists(ShowListsArgs),
}

#[derive(Args, Clone, Debug)]
#[command()]
struct AddArgs {
    #[arg(required = true)]
    todo_list_title: String,

    #[arg(required = true, num_args = 1..)]
    items: Vec<String>,
}

#[derive(Args, Clone, Debug)]
#[command()]
struct CreateTodoListArgs {
    #[arg(required = true)]
    title: String,
}

#[derive(Args, Clone, Debug)]
#[command()]
struct ClearArgs {
    #[arg(required = true)]
    todo_list_title: String,
}

#[derive(Args, Clone, Debug)]
#[command()]
struct DoneArgs {
    #[arg(required = true)]
    todo_list_title: String,

    #[arg(required = true, num_args = 1..)]
    ids: Vec<u32>,
}

#[derive(Args, Clone, Debug)]
#[command()]
struct EditArgs {
    #[arg(required = true)]
    todo_list_title: String,

    #[arg(required = true, num_args = 1..)]
    items: Vec<String>,
}

#[derive(Args, Clone, Debug)]
#[command()]
struct ListArgs {
    #[arg(required = true)]
    todo_list_title: String,
}

#[derive(Args, Clone, Debug)]
#[command()]
struct RemoveArgs {
    #[arg(required = true)]
    todo_list_title: String,

    #[arg(required = true, num_args = 1..)]
    ids: Vec<u32>,
}

#[derive(Args, Clone, Debug)]
#[command()]
struct ShowListsArgs {
    #[arg(short = 'd', default_value = "false")]
    with_deleted: bool,
}

#[derive(Debug, Display)]
enum CliError {
    ArgsError(&'static str),
    LogicError(&'static str),
}

impl Error for CliError {}

impl TodoCli {
    pub async fn exec(&self) -> Result<CliOutput> {
        match &self.subcommand {
            SubCommand::Add(AddArgs {
                todo_list_title,
                items,
            }) => {
                let todo_list = Self::try_get_todo_list_by_title(todo_list_title).await?;
                if items.is_empty() {
                    return Err(CliError::ArgsError("No items passed to be added").into());
                }

                let added_rows = db::add_todo_list_items_returning(todo_list.id(), items).await?;

                Ok(CliOutput::TodoItemsOutput(added_rows))
            }
            SubCommand::Clear(ClearArgs { todo_list_title }) => {
                let todo_list = Self::try_get_todo_list_by_title(todo_list_title).await?;

                db::clear_todo_list_items(todo_list.id()).await?;

                Ok(CliOutput::TodoListClearedOutput(todo_list_title))
            }
            SubCommand::CreateTodoList(CreateTodoListArgs { title }) => {
                if title.is_empty() {
                    return Err(CliError::ArgsError("Todo list title cannot be ''").into());
                }

                let todo_list_option = db::get_todo_list_by_title(title).await?;

                if todo_list_option.is_some() {
                    return Err(CliError::LogicError(
                        "A todo list with the same title already exists!",
                    )
                    .into());
                }

                create_todo_list(title).await?;

                Ok(CliOutput::TodoListCreatedOutput(title))
            }
            SubCommand::Done(DoneArgs {
                todo_list_title,
                ids,
            }) => {
                let todo_list = Self::try_get_todo_list_by_title(todo_list_title).await?;

                if ids.is_empty() {
                    return Err(CliError::ArgsError("No ids passed to be set to done").into());
                }

                db::set_todo_list_items_done(todo_list.id(), ids).await?;

                let rows = db::get_todo_list_items(todo_list.id(), false).await?;

                Ok(CliOutput::TodoItemsOutput(rows))
            }
            SubCommand::Edit(EditArgs {
                todo_list_title,
                items,
            }) => {
                let todo_list = Self::try_get_todo_list_by_title(todo_list_title).await?;

                if items.len() % 2 == 1 {
                    return Err(CliError::ArgsError(
                        "The input has to be pairs of ids and todo items",
                    )
                    .into());
                }

                let mut id_and_item_pairs = Vec::new();

                for half_idx in 0..(items.iter().len() / 2) {
                    let idx = half_idx * 2;

                    let id = &items[idx];
                    let id: u32 = match id.parse() {
                        Ok(id) => id,
                        Err(_) => {
                            return Err(CliError::ArgsError(
                                "The input has to be pairs of ids and todo items",
                            )
                            .into());
                        }
                    };
                    let item = items[idx + 1].clone();

                    id_and_item_pairs.push((id, item));
                }

                let edited_rows =
                    db::edit_todo_list_items(todo_list.id(), &id_and_item_pairs).await?;

                Ok(CliOutput::TodoItemsOutput(edited_rows))
            }
            SubCommand::List(ListArgs { todo_list_title }) => {
                let todo_list = Self::try_get_todo_list_by_title(todo_list_title).await?;
                let rows = db::get_todo_list_items(todo_list.id(), false).await?;

                Ok(CliOutput::TodoItemsOutput(rows))
            }
            SubCommand::Remove(RemoveArgs {
                todo_list_title,
                ids,
            }) => {
                let todo_list = Self::try_get_todo_list_by_title(todo_list_title).await?;

                if ids.is_empty() {
                    return Err(CliError::ArgsError("No ids passed to remove").into());
                }

                db::remove_todo_list_items(todo_list.id(), ids).await?;

                let rows = db::get_todo_list_items(todo_list.id(), false).await?;

                Ok(CliOutput::TodoItemsOutput(rows))
            }
            SubCommand::ShowLists(ShowListsArgs { with_deleted }) => {
                let todo_lists = db::get_todo_lists(*with_deleted).await?;

                Ok(CliOutput::TodoListsOutput(todo_lists))
            }
        }
    }

    async fn try_get_todo_list_by_title(todo_list_title: &str) -> Result<TodoListRow> {
        let todo_list = db::get_todo_list_by_title(todo_list_title).await?;

        match todo_list {
            Some(todo_list) => Ok(todo_list),
            None => Err(CliError::ArgsError("Could not find todo list with the given name").into()),
        }
    }
}

pub enum CliOutput<'a> {
    TodoItemsOutput(Vec<TodoListItemRow>),
    TodoListsOutput(Vec<TodoListRow>),
    TodoListCreatedOutput(&'a str),
    TodoListClearedOutput(&'a str),
}

impl Display for CliOutput<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TodoItemsOutput(todo_list_items) => fmt_todo_list_items(todo_list_items, f),
            Self::TodoListsOutput(todo_lists) => fmt_todo_lists(todo_lists, f),
            Self::TodoListCreatedOutput(title) => {
                write!(f, "Created a new list '{}'", title)
            }
            Self::TodoListClearedOutput(title) => {
                write!(f, "'{}' list has been cleared", title)
            }
        }
    }
}

fn fmt_todo_lists(todo_lists: &[TodoListRow], f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if todo_lists.is_empty() {
        return write!(f, "No lists created");
    }

    writeln!(f, " {:<30}| {:<30}", "list", "created at")?;
    write!(f, "{}|{}", "=".repeat(31), "=".repeat(31))?;
    for todo_list in todo_lists {
        let created_at = todo_list.created_at().with_timezone(&Local).to_rfc2822();
        write!(f, "\n {:<30}| {:<30}", todo_list.title(), created_at)?;
    }

    Ok(())
}

fn fmt_todo_list_items(
    todo_list_items: &[TodoListItemRow],
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    if todo_list_items.is_empty() {
        return write!(f, "list is empty");
    }

    writeln!(
        f,
        " {:<5}| {:<30}| {:<30}| {:<30}",
        "id", "item", "created at", "done at"
    )?;
    write!(
        f,
        "{}|{}|{}|{}",
        "=".repeat(6),
        "=".repeat(31),
        "=".repeat(31),
        "=".repeat(31)
    )?;
    for todo in todo_list_items {
        let id = todo.id();
        let title = todo.title();
        let created_at = todo.created_at().with_timezone(&Local).to_rfc2822();
        let done_at: Box<dyn Display> = if let Some(done_at) = todo.done_at() {
            Box::new(done_at.with_timezone(&Local).to_rfc2822())
        } else {
            Box::new("")
        };

        let (id, title): (Box<dyn Display>, Box<dyn Display>) = if todo.is_done() {
            (
                Box::new(id.strikethrough()),
                Box::new(title.strikethrough()),
            )
        } else {
            (Box::new(id), Box::new(title))
        };

        write!(
            f,
            "\n {:<5}| {:<30}| {:<30}| {:<30}",
            id, title, created_at, done_at,
        )?;
    }

    Ok(())
}
