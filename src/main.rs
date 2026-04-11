// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <seiversiana@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::collections::HashMap;

use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use colored::Colorize;
use directories::ProjectDirs;
use jiff::{civil::DateTime, Span};
use serde::{de::DeserializeOwned, Serialize};



enum FatalError
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

impl FatalError
{
	fn message(&self) -> String
	{
		match self
		{
			Self::ProjectDirsUnavailable =>
				"Failed to retrieve project dirs.".to_string(),

			Self::BadDataTree(path) =>
				format!("Data directory '{}' is in an invalid state.", path.display()),

			Self::DirCreateFail(path) =>
				format!("Failed to create directory '{}'.", path.display()),

			Self::FileCreateFail(path) =>
				format!("Failed to create file '{}'.", path.display()),

			Self::FileReadFail(path) =>
				format!("Failed to read from '{}'.", path.display()),

			Self::FileWriteFail(path) =>
				format!("Failed to write to '{}'.", path.display()),

			Self::DeserializeFail(path) =>
				format!("Failed to deserialize from '{}'.", path.display()),

			Self::SerializeFail(path) =>
				format!("Failed to serialize to '{}'.", path.display())
		}
	}
}

enum Exit
{
	NotInitialized,
	AlreadyInitialized(PathBuf),
	TodoExists(String)
}

impl Exit
{
	fn message(&self) -> String
	{
		match self
		{
			Self::NotInitialized =>
				"selfish is not yet initialized.".to_string(),

			Self::AlreadyInitialized(path) =>
				format!("selfish is already initialized at {}.", path.display()),

			Self::TodoExists(name) =>
				format!("A todo named '{}' already exists.", name)
		}
	}
}

enum Outcome
{
	FatalError(FatalError),
	Exit(Exit)
}



struct DataTree
{
	dirs: ProjectDirs
}

impl DataTree
{
	fn from_default() -> Result<Self, Outcome>
	{
		let dirs = ProjectDirs::from("com", "seiversiana", "selfish")
			.ok_or(Outcome::FatalError(FatalError::ProjectDirsUnavailable))?;

		Ok(DataTree { dirs })
	}

	fn initialized(&self) -> Result<bool, Outcome>
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
			Err(Outcome::FatalError(FatalError::BadDataTree(self.root())))
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

struct ScheduleItem
{
	todo: String,
	start: DateTime,
	end: DateTime
}



type Todos = HashMap<String, ()>;
type Schedule = Vec<ScheduleItem>;



fn create_if_missing(path: &Path, content: &str) -> Result<(), Outcome>
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
			return Err(Outcome::FatalError(FatalError::FileCreateFail(path.to_path_buf())));
		}
	};

	file.write_all(content.as_bytes())
		.map_err(|_| Outcome::FatalError(FatalError::FileWriteFail(path.to_path_buf())))
}

fn init(tree: &DataTree) -> Result<(), Outcome>
{
	let initialized = tree.initialized();

	match initialized
	{
		Ok(true) => {
			return Err(Outcome::Exit(Exit::AlreadyInitialized(tree.root().to_path_buf())));
		}

		Err(_) =>
		{
			println!("{}", "selfish data dir already exists but is incomplete. Creating the missing files.".blue());
		}

		_ =>
		{
			let root = tree.root();
			fs::create_dir_all(&root).map_err(|_| Outcome::FatalError(FatalError::DirCreateFail(root)))?;
		}
	}

	let content = "{}";
	create_if_missing(&tree.todo_path(), &content)?;
	create_if_missing(&tree.schedule_path(), &content)?;

	println!("{}", "Successfully initialized selfish.".green());

	Ok(())
}

fn todo(tree: &DataTree, command: TodoCommands) -> Result<(), Outcome>
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



fn read_deserialize<T: DeserializeOwned>(path: &Path) -> Result<T, Outcome>
{
	let data = fs::read_to_string(path)
		.map_err(|_| Outcome::FatalError(FatalError::FileReadFail(path.to_path_buf())))?;

	serde_json::from_str(&data)
		.map_err(|_| Outcome::FatalError(FatalError::DeserializeFail(path.to_path_buf())))
}

fn write_serialize<T: Serialize>(path: &Path, data: &T) -> Result<(), Outcome>
{
	let data = serde_json::to_string_pretty(data)
		.map_err(|_| Outcome::FatalError(FatalError::SerializeFail(path.to_path_buf())))?;

	fs::write(&path, data)
		.map_err(|_| Outcome::FatalError(FatalError::FileWriteFail(path.to_path_buf())))
}

fn read_todo(tree: &DataTree) -> Result<Todos, Outcome>
{
	read_deserialize(&tree.todo_path())
}

fn write_todo(tree: &DataTree, todos: &Todos) -> Result<(), Outcome>
{
	write_serialize(&tree.todo_path(), todos)
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
	Todo { #[command(subcommand)] command: TodoCommands },
	Schedule { #[command(subcommand)] command: ScheduleCommands }
}

#[derive(Subcommand)]
enum TodoCommands
{
	Add { name: String },
	List
}

#[derive(Subcommand)]
enum ScheduleCommands
{
	Add
	{
		todo: String,
		start: DateTime,
		duration: Span
	}
}



fn run() -> Result<(), Outcome>
{
	let selfish = Selfish::parse();
	let data_tree = DataTree::from_default()?;

	match selfish.command
	{
		Commands::Init => init(&data_tree),
		Commands::Todo { command } => todo(&data_tree, command),
		Commands::Schedule { command } => todo!()
	}
}

fn main()
{
	if let Err(outcome) = run()
	{
		match outcome
		{
			Outcome::FatalError(kind) => { println!("{}", kind.message().red()); }
			Outcome::Exit(kind) => { println!("{}", kind.message().yellow()); }
		}
	}
}
