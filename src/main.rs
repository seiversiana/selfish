// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <seiversiana@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::collections::HashMap;

use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use colored::Colorize;
use directories::ProjectDirs;



enum Error
{
	ProjectDirsUnavailable,
	BadDataTree(PathBuf),
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

			Self::BadDataTree(path) =>
				format!("Bad data tree at {}.", path.display()),

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

	fn initialized(&self) -> Result<bool, Error>
	{
		let existences =
		[
			self.root().exists(),
			self.todo_path().exists()
		];

		let first = existences[0];
		if existences.iter().all(|&v| v == first)
		{
			Ok(first)
		}
		else
		{
			Err(Error::BadDataTree(self.root()))
		}
	}

	fn root(&self) -> PathBuf
	{
		self.dirs.data_local_dir().to_path_buf()
	}

	fn todo_path(&self) -> PathBuf
	{
		self.root().join("todo.json")
	}

	fn schedule_path(&self) -> PathBuf
	{
		self.root().join("schedule.json")
	}
}



fn create_if_missing(path: &Path, content: &str) -> Result<(), Error>
{
	let mut file = match fs::OpenOptions::new()
		.write(true)
		.create_new(true)
		.open(path)
	{
		Ok(file) => file,
		Err(error) if error.kind() == io::ErrorKind::AlreadyExists =>
		{
			return Ok(());
		}
		Err(_) =>
		{
			return Err(Error::FileCreateFail(path.to_path_buf()));
		}
	};

	file.write_all(content.as_bytes())
		.map_err(|_| Error::FileWriteFail(path.to_path_buf()))?;

	Ok(())
}

fn init(tree: &DataTree) -> Result<(), Error>
{
	let initialized = tree.initialized();

	match initialized
	{
		Ok(true) =>
		{
			println!("{}", "selfish is already initialized!".yellow());
			return Ok(());
		}

		Err(_) =>
		{
			println!("{}", "selfish data dir already exists but is incomplete. Creating the missing files.".yellow());
		}

		_ =>
		{
			let root = tree.root();
			fs::create_dir_all(&root).map_err(|_| Error::DirCreateFail(root))?;
		}
	}

	let content = "{}";
	create_if_missing(&tree.todo_path(), &content)?;
	create_if_missing(&tree.schedule_path(), &content)?;

	println!("{}", "Successfully initialized selfish.".green());

	Ok(())
}

fn todo(tree: &DataTree, command: TodoCommands) -> Result<(), Error>
{
	if !tree.initialized()?
	{
		println!("{}", "selfish is not yet initialized!".yellow());
		return Ok(());
	}

	let mut todos = read_todo(tree)?;

	match command
	{
		TodoCommands::Add { name } =>
		{
			if todos.contains_key(&name)
			{
				println!("{}", "A todo of that name already exists!".yellow());
				return Ok(());
			}

			todos.insert(name, ());
			write_todo(tree, todos)?;
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



fn read_todo(tree: &DataTree) -> Result<HashMap<String, ()>, Error>
{
	let todo_path = tree.todo_path();

	let data = fs::read_to_string(&todo_path)
		.map_err(|_| Error::FileReadFail(todo_path.clone()))?;

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
	Add { name: String },
	List
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
