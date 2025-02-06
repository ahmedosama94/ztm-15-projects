use std::fmt::Display;

use clap::{Args, Parser, Subcommand};
use color_eyre::eyre::Result;
use db::models::TodoItemRow;
use derive_more::Display;
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
    Done(DoneArgs),
    Edit(EditArgs),
    List(ListArgs),
    Remove(RemoveArgs),
    Clear(ClearArgs),
}

#[derive(Args, Clone, Debug)]
#[command()]
struct AddArgs {
    #[arg(required = true, num_args = 1..)]
    items: Vec<String>,
}

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
struct RemoveArgs {}

#[derive(Args, Clone, Debug)]
struct ClearArgs {}

#[derive(Debug, Display)]
enum CliError {
    ParseError(&'static str),
}

impl Error for CliError {}

impl Todo {
    pub async fn exec(&self) -> Result<TodoOutput> {
        match &self.subcommand {
            SubCommand::Add(AddArgs { items }) => {
                if items.is_empty() {
                    panic!("No items passed to be added");
                }

                db::add_todos(items).await?;
            }
            SubCommand::Clear(_) => {
                todo!()
            }
            SubCommand::Done(DoneArgs { ids }) => {
                if ids.is_empty() {
                    panic!("No ids passed to set to done");
                }
            }
            SubCommand::Edit(EditArgs { items }) => {
                if items.len() % 2 == 1 {
                    panic!("The input has to be pairs of ids and todo items");
                }

                let mut id_and_item_pairs = Vec::new();

                for half_idx in 0..(items.iter().len() / 2) {
                    let idx = half_idx * 2;

                    let id = &items[idx];
                    let id: u32 = match id.parse() {
                        Ok(id) => id,
                        Err(_) => panic!("Could not parse id '{}'", id),
                    };
                    let item = items[idx + 1].clone();

                    id_and_item_pairs.push((id, item));
                }

                db::edit_todos(&id_and_item_pairs).await?;
            }
            SubCommand::List(_) => {}
            SubCommand::Remove(_) => {
                todo!()
            }
            _ => {
                todo!();
            }
        };

        let rows = db::get_all_todos().await?;

        Ok(TodoOutput::new(rows))
    }
}

pub struct TodoOutput {
    todos: Vec<TodoItemRow>,
}

impl TodoOutput {
    fn new(todos: Vec<TodoItemRow>) -> Self {
        Self { todos }
    }
}

impl Display for TodoOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.todos.is_empty() {
            return write!(f, "list is empty");
        }

        writeln!(f, "{:<5}| {:<30}", "id", "item")?;
        write!(f, "=====|=============================")?;
        for todo in &self.todos {
            write!(f, "\n{:<5}|{:<30}", todo.id(), todo.item())?;
        }

        Ok(())
    }
}
