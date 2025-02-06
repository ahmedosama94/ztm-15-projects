use std::fmt::Display;

use clap::{Parser, ValueEnum};
use color_eyre::eyre::Result;
use db::models::TodoItemRow;
use derive_more::Display;

mod db;

#[derive(Clone, Debug, Parser, ValueEnum, Display)]
enum SubCommand {
    Add,
    Done,
    Edit,
    List,
    Remove,
}

#[derive(Debug, Parser)]
pub struct Todo {
    #[arg(value_name = "subcommand", default_value = "list")]
    subcommand: SubCommand,

    #[arg(value_name = "items", value_delimiter = ' ', num_args = 1..)]
    items: Vec<String>,
}

impl Todo {
    pub async fn exec(&self) -> Result<TodoOutput> {
        match self.subcommand {
            SubCommand::Add => {
                db::add_todos(&self.items).await?;
            }
            SubCommand::Edit => {
                if self.items.len() % 2 == 1 {
                    panic!("The input has to be pairs of ids and todo items");
                }

                let mut id_and_item_pairs = Vec::new();

                for half_idx in 0..self.items.iter().len() {
                    let idx = half_idx * 2;

                    let id = &self.items[idx];
                    let id: u32 = match id.parse() {
                        Ok(id) => id,
                        Err(_) => panic!("Could not parse id '{}'", id),
                    };
                    let item = self.items[idx + 1].clone();

                    id_and_item_pairs.push((id, item));
                }

                db::edit_todos(id_and_item_pairs).await?;
            }
            SubCommand::List => {}
            _ => {
                todo!("{}", self.subcommand.to_string().to_lowercase());
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
