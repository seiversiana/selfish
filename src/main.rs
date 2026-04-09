// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <seiversiana@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use colored::Colorize;
use directories::ProjectDirs;



fn selfish_dirs() -> ProjectDirs
{
	return ProjectDirs::from("com", "seiversiana", "selfish").expect("Could not retrieve directories");
}

fn data_dir() -> PathBuf
{
	return selfish_dirs().data_local_dir().to_path_buf();
}

fn init()
{
	if data_dir().exists()
	{
		println!("{}", "selfish is already initialized!".red());
		return;
	}

	fs::create_dir_all(data_dir()).expect("Could not create store");

	println!("{}", "Successfully initialized selfish.".green());
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



fn main()
{
	let selfish = Selfish::parse();

	match &selfish.command
	{
		Commands::Init => init()
	}
}
