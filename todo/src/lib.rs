use std::fmt::Display;

use clap::{Parser, ValueEnum};
use color_eyre::eyre::Result;

mod db;

#[derive(Clone, Debug, Parser, ValueEnum)]
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
    pub fn exec(&self) -> Result<TodoOutput> {
        println!("{:#?}", self);
        Ok(TodoOutput::new(Vec::new()))
    }
}

pub struct TodoOutput {
    todos: Vec<String>,
}

impl TodoOutput {
    fn new(todos: Vec<String>) -> Self {
        Self { todos }
    }
}

impl Display for TodoOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.todos.join("\n"))
    }
}
