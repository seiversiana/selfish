// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <seiversiana@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use clap::Parser;

mod cli;
mod outcome;

use cli::{Cli, Commands};



fn main()
{
	let cli = Cli::parse();

	match &cli.command
	{
		Commands::Init => todo!()
	}
}
