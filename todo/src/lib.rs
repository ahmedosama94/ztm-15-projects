use std::{fmt::Display, iter};

use clap::{Args, Parser, Subcommand};
use color_eyre::{
    eyre::{ContextCompat, Result},
    owo_colors::OwoColorize,
};
use db::models::TodoItemRow;
use derive_more::Display;
use sqlx::types::chrono::Local;
use std::error::Error;

mod db;

#[derive(Debug, Parser)]
pub struct Todo {
    #[command(subcommand)]
    subcommand: SubCommand,
}

#[derive(Clone, Debug, Subcommand)]
enum SubCommand {
    #[command()]
    Add(AddArgs),
    Clear(ClearArgs),
    Done(DoneArgs),
    Edit(EditArgs),
    List(ListArgs),
    Remove(RemoveArgs),
}

#[derive(Args, Clone, Debug)]
#[command()]
struct AddArgs {
    #[arg(required = true, num_args = 1..)]
    items: Vec<String>,
}

#[derive(Args, Clone, Debug)]
struct ClearArgs {}

#[derive(Args, Clone, Debug)]
#[command()]
struct DoneArgs {
    #[arg()]
    ids: Vec<u32>,
}

#[derive(Args, Clone, Debug)]
struct EditArgs {
    #[arg(required = true, num_args = 1..)]
    items: Vec<String>,
}

#[derive(Args, Clone, Debug)]
struct ListArgs {}

#[derive(Args, Clone, Debug)]
struct RemoveArgs {
    #[arg()]
    ids: Vec<u32>,
}

#[derive(Debug, Display)]
enum CliError {
    ArgsError(&'static str),
}

impl Error for CliError {}

impl Todo {
    pub async fn exec(&self) -> Result<()> {
        match &self.subcommand {
            SubCommand::Add(AddArgs { items }) => {
                if items.is_empty() {
                    return Err(CliError::ArgsError("No items passed to be added").into());
                }

                db::add_todos(items, false).await?;
            }
            SubCommand::Clear(_) => {
                db::clear_todos().await?;
                println!("{}", TodoOutput::new_empty());

                return Ok(());
            }
            SubCommand::Done(DoneArgs { ids }) => {
                if ids.is_empty() {
                    return Err(CliError::ArgsError("No ids passed to be set to done").into());
                }

                db::set_todos_done(ids).await?;
            }
            SubCommand::Edit(EditArgs { items }) => {
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

                db::edit_todos(&id_and_item_pairs).await?;
            }
            SubCommand::List(ListArgs {}) => {}
            SubCommand::Remove(RemoveArgs { ids }) => {
                if ids.is_empty() {
                    return Err(CliError::ArgsError("No ids passed to remove").into());
                }

                db::remove_todos(ids).await?;
            }
        };

        let rows = db::get_todos(None).await?;
        println!("{}", TodoOutput::new(rows));

        Ok(())
    }
}

pub struct TodoOutput {
    todos: Vec<TodoItemRow>,
}

impl TodoOutput {
    fn new(todos: Vec<TodoItemRow>) -> Self {
        Self { todos }
    }

    fn new_empty() -> Self {
        Self::new(Vec::new())
    }
}

impl Display for TodoOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.todos.is_empty() {
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
        for todo in &self.todos {
            let id = todo.id();
            let item = todo.item();
            let created_at = todo.created_at().with_timezone(&Local).to_rfc2822();
            let done_at: Box<dyn Display> = if let Some(done_at) = todo.done_at() {
                Box::new(done_at.with_timezone(&Local).to_rfc2822())
            } else {
                Box::new("")
            };

            let (id, item): (Box<dyn Display>, Box<dyn Display>) = if todo.is_done() {
                (Box::new(id.strikethrough()), Box::new(item.strikethrough()))
            } else {
                (Box::new(id), Box::new(item))
            };

            write!(
                f,
                "\n {:<5}| {:<30}| {:<30}| {:<30}",
                id, item, created_at, done_at,
            )?;
        }

        Ok(())
    }
}
