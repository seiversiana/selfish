// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <seiversiana@gmail.com>
// SPDX-License-Identifier: MPL-2.0

mod cli;
mod init;
mod files;
mod outcome;
mod schedule;
mod storage;
mod todo;

use clap::Parser;
use colored::Colorize;

use crate::cli::{Commands, Selfish};
use crate::init::init;
use crate::outcome::Outcome;
use crate::storage::DataTree;
use crate::todo::todo;



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
