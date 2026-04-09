// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <seiversiana@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

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



fn dirs() -> Result<ProjectDirs, Error>
{
	ProjectDirs::from("com", "seiversiana", "selfish")
		.ok_or(Error::ProjectDirsUnavailable)
}

fn init(path: &Path) -> Result<(), Error>
{
	if path.exists()
	{
		println!("{}", "selfish is already initialized!".yellow());
		return Ok(());
	}

	fs::create_dir_all(&path)
		.map_err(|_| Error::DirCreateFail(path.to_path_buf()))?;

	let todo_path = path.join("todo.json");
	fs::write(&todo_path, b"{}")
		.map_err(|_| Error::FileCreateFail(todo_path))?;

	println!("{}", "Successfully initialized selfish.".green());

	Ok(())
}



fn read_todo(path: &Path) -> Result<HashMap<String, ()>, Error>
{
	let todo_path = path.join("todo.json");

	let data = fs::read_to_string(&todo_path)
		.map_err(|_| Error::FileReadFail(todo_path.clone()))?
		.to_string();

	Ok(serde_json::from_str::<HashMap<String, ()>>(&data)
		.map_err(|_| Error::DeserializeFail(todo_path))?)
}

fn write_todo(path: &Path, todos: HashMap<String, ()>) -> Result<(), Error>
{
	let todo_path = path.join("todo.json");

	let data = serde_json::to_string_pretty(&todos)
		.map_err(|_| Error::SerializeFail(todo_path.clone()))?;

	fs::write(&todo_path, data)
		.map_err(|_| Error::FileWriteFail(todo_path))?;

	Ok(())
}

fn todo_add(path: &Path, name: String) -> Result<(), Error>
{
	if !path.exists()
	{
		println!("{}", "selfish is not yet initialized!".yellow());
		return Ok(());
	}

	let mut todos = read_todo(path)?;

	if todos.contains_key(&name)
	{
		println!("{}", "A todo of that name already exists!".yellow());
		return Ok(());
	}

	todos.insert(name, ());
	write_todo(path, todos)?;
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



fn todo(path: &Path, command: TodoCommands) -> Result<(), Error>
{
	match command
	{
		TodoCommands::Add { name } => todo_add(path, name)
	}
}

fn run() -> Result<(), Error>
{
	let selfish = Selfish::parse();
	let dirs = dirs()?;
	let data_dir = dirs.data_local_dir();

	match selfish.command
	{
		Commands::Init => init(data_dir),
		Commands::Todo { command } => todo(data_dir, command)
	}
}

fn main()
{
	if let Err(error) = run()
	{
		println!("{}", error.message().red())
	}
}
