// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <seiversiana@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::collections::HashMap;

use colored::Colorize;

use crate::cli::TodoCommands;
use crate::files::{read_deserialize, write_serialize};
use crate::storage::DataTree;
use crate::outcome::{Exit, Outcome};



pub type Todos = HashMap<String, ()>;



pub fn todo(tree: &DataTree, command: TodoCommands) -> Result<(), Outcome>
{
	if !tree.initialized()?
	{
		return Err(Outcome::Exit(Exit::NotInitialized));
	}

	let mut todos = read_todo(tree)?;

	match command
	{
		TodoCommands::Add { name } =>
		{
			if todos.contains_key(&name)
			{
				return Err(Outcome::Exit(Exit::TodoExists(name)));
			}

			todos.insert(name, ());
			write_todo(tree, &todos)?;
			println!("{}", "Added todo item.".green());
		}

		TodoCommands::List =>
		{
			for (name, _) in todos
			{
				println!("{}", name);
			}
		}
	}

	Ok(())
}

fn read_todo(tree: &DataTree) -> Result<Todos, Outcome>
{
	read_deserialize(&tree.todo_path())
}

fn write_todo(tree: &DataTree, todos: &Todos) -> Result<(), Outcome>
{
	write_serialize(&tree.todo_path(), todos)
}
