// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <seiversiana@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use colored::Colorize;
use directories::ProjectDirs;



enum Error
{
	ProjectDirsUnavailable,
	DirCreateFail(PathBuf),
	FileCreateFail(PathBuf),
	FileReadFail(PathBuf),
	FileWriteFail(PathBuf),
	DeserializeFail(PathBuf),
	SerializeFail(PathBuf)
}

impl Error
{
	fn message(&self) -> String
	{
		match self
		{
			Self::ProjectDirsUnavailable =>
				"Failed to retrieve project dirs.".to_string(),

			Self::DirCreateFail(path) =>
				format!("Failed to create dir at {}.", path.display()),

			Self::FileCreateFail(path) =>
				format!("Failed to create file at {}.", path.display()),

			Self::FileReadFail(path) =>
				format!("Failed to read file at {}.", path.display()),

			Self::FileWriteFail(path) =>
				format!("Failed to write file at {}.", path.display()),

			Self::DeserializeFail(path) =>
				format!("Failed to deserialize {}.", path.display()),

			Self::SerializeFail(path) =>
				format!("Failed to serialize {}.", path.display())
		}
	}
}



struct DataTree
{
	dirs: ProjectDirs
}

impl DataTree
{
	fn from_default() -> Result<Self, Error>
	{
		let dirs = ProjectDirs::from("com", "seiversiana", "selfish")
			.ok_or(Error::ProjectDirsUnavailable)?;

		Ok(DataTree { dirs })
	}

	fn root(&self) -> PathBuf
	{
		self.dirs.data_local_dir().to_path_buf()
	}

	fn todo_path(&self) -> PathBuf
	{
		self.root().join("todo.json")
	}
}



fn init(tree: &DataTree) -> Result<(), Error>
{
	let root = tree.root();
	let todo_path = tree.todo_path();

	if root.exists()
	{
		println!("{}", "selfish is already initialized!".yellow());
		return Ok(());
	}

	fs::create_dir_all(&root)
		.map_err(|_| Error::DirCreateFail(root))?;

	fs::write(&todo_path, b"{}")
		.map_err(|_| Error::FileCreateFail(todo_path))?;

	println!("{}", "Successfully initialized selfish.".green());

	Ok(())
}



fn read_todo(tree: &DataTree) -> Result<HashMap<String, ()>, Error>
{
	let todo_path = tree.todo_path();

	let data = fs::read_to_string(&todo_path)
		.map_err(|_| Error::FileReadFail(todo_path.clone()))?
		.to_string();

	Ok(serde_json::from_str::<HashMap<String, ()>>(&data)
		.map_err(|_| Error::DeserializeFail(todo_path))?)
}

fn write_todo(tree: &DataTree, todos: HashMap<String, ()>) -> Result<(), Error>
{
	let todo_path = tree.todo_path();

	let data = serde_json::to_string_pretty(&todos)
		.map_err(|_| Error::SerializeFail(todo_path.clone()))?;

	fs::write(&todo_path, data)
		.map_err(|_| Error::FileWriteFail(todo_path))?;

	Ok(())
}

fn todo_add(tree: &DataTree, name: String) -> Result<(), Error>
{
	if !tree.root().exists()
	{
		println!("{}", "selfish is not yet initialized!".yellow());
		return Ok(());
	}

	let mut todos = read_todo(tree)?;

	if todos.contains_key(&name)
	{
		println!("{}", "A todo of that name already exists!".yellow());
		return Ok(());
	}

	todos.insert(name, ());
	write_todo(tree, todos)?;
	println!("{}", "Added todo item.".green());

	Ok(())
}



#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Selfish
{
	#[command(subcommand)]
	command: Commands
}

#[derive(Subcommand)]
enum Commands
{
	Init,
	Todo { #[command(subcommand)] command: TodoCommands }
}

#[derive(Subcommand)]
enum TodoCommands
{
	Add { name: String }
}



fn todo(tree: &DataTree, command: TodoCommands) -> Result<(), Error>
{
	match command
	{
		TodoCommands::Add { name } => todo_add(tree, name)
	}
}

fn run() -> Result<(), Error>
{
	let selfish = Selfish::parse();
	let data_tree = DataTree::from_default()?;

	match selfish.command
	{
		Commands::Init => init(&data_tree),
		Commands::Todo { command } => todo(&data_tree, command)
	}
}

fn main()
{
	if let Err(error) = run()
	{
		println!("{}", error.message().red())
	}
}
