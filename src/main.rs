// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <seiversiana@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::fs;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use colored::Colorize;
use directories::ProjectDirs;



enum Error
{
	ProjectDirsUnavailable,
	DataDirCreateFail(PathBuf)
}

impl Error
{
	fn message(&self) -> String
	{
		match self
		{
			Self::ProjectDirsUnavailable =>
				"Could not retrieve project dirs.".to_string(),
			Self::DataDirCreateFail(path) =>
				format!("Could not create data dir at {}.", path.display())
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
	}
	else
	{
		fs::create_dir_all(path)
			.map_err(|_| Error::DataDirCreateFail(path.to_path_buf()))?;

		println!("{}", "Successfully initialized selfish.".green());
	}

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
	Init
}



fn run() -> Result<(), Error>
{
	let selfish = Selfish::parse();

	match &selfish.command
	{
		Commands::Init => init(dirs()?.config_local_dir())
	}
}

fn main()
{
	if let Err(error) = run()
	{
		println!("{}", error.message().red())
	}
}
